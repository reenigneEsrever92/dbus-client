use std::{convert::Infallible, time::Duration};

use clap::{App, Arg, SubCommand, Values};
use dbus::{
    arg::messageitem::{MessageItem, MessageItemArray, MessageItemDict},
    blocking::Connection,
    channel::Channel,
    strings::Signature,
    Message,
};
use dbus_type::DBusType;
use dbus_value::DBusValue;
use itertools::Itertools;
use log::{debug, warn, LevelFilter};
use simple_logger::SimpleLogger;
use xml::{
    attribute::OwnedAttribute,
    reader::{Error, XmlEvent},
    EventReader,
};

mod dbus_type;
mod dbus_value;

fn main() {
    let app = App::new("Dbus client for Introspection")
        .version("0.1.0")
        .author("Felix M. <fmarezki@gmail.com>")
        .about("Interact with dbus")
        .subcommand(
            SubCommand::with_name("list-names")
                .about("List bus names")
                .alias("ls"),
        )
        .subcommand(
            SubCommand::with_name("introspect")
                .about("Introspect object under a certain path")
                .alias("i")
                .arg(
                    Arg::with_name("bus-name")
                        .required(true)
                        .help("Name of the bus"),
                )
                .arg(
                    Arg::with_name("path")
                        .required(true)
                        .help("Path of the object to introspect"),
                ),
        )
        .subcommand(
            SubCommand::with_name("call")
                .about("Call a method on an interface")
                .alias("c")
                .arg(
                    Arg::with_name("bus-name")
                        .required(true)
                        .help("Name of the bus"),
                )
                .arg(
                    Arg::with_name("path")
                        .required(true)
                        .help("Path of the object"),
                )
                .arg(
                    Arg::with_name("interface")
                        .required(true)
                        .help("Interface name"),
                )
                .arg(Arg::with_name("method").required(true).help("Method name"))
                .arg(
                    Arg::with_name("argument")
                        .takes_value(true)
                        .required(false)
                        .help("Argument passed to the method call"),
                ),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the verbosity level of the logger"),
        )
        .arg(
            Arg::with_name("address")
                .short("d")
                .value_name("ADDRESS")
                .default_value("session")
                .help("A custom dbus address")
                .env("DBUS_CLIENT_ADDRESS"),
        );

    let matches = app.get_matches();

    match matches.occurrences_of("v") {
        0 => SimpleLogger::new()
            .with_level(LevelFilter::Error)
            .init()
            .unwrap(),
        1 => SimpleLogger::new()
            .with_level(LevelFilter::Warn)
            .init()
            .unwrap(),
        2 => SimpleLogger::new()
            .with_level(LevelFilter::Info)
            .init()
            .unwrap(),
        3 | _ => SimpleLogger::new()
            .with_level(LevelFilter::Debug)
            .init()
            .unwrap(),
    }

    let connection = build_connection(matches.value_of("address").unwrap_or_default());

    match matches.subcommand() {
        ("list-names", Some(_cmd)) => {
            list_names(connection);
        }
        ("introspect", Some(cmd)) => {
            introspect(
                &connection,
                &cmd.value_of("bus-name").unwrap().into(),
                &cmd.value_of("path").unwrap().into(),
            );
        }
        ("call", Some(cmd)) => call(
            &connection,
            &cmd.value_of("bus-name").unwrap().into(),
            &cmd.value_of("path").unwrap().into(),
            cmd.value_of("interface").unwrap().into(),
            cmd.value_of("method").unwrap().into(),
            cmd.value_of("argument"),
        ),
        _ => {
            println!("{}", matches.usage())
        }
    }
}

struct DBusArgument<'a> {
    dbus_type: &'a DBusType,
    dbus_value: &'a DBusValue,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DBusError {
    InvalidSignature,
    InvalidValue(String),
}

#[derive(Debug)]
enum Entry {
    Node { name: String },
    Interface { name: String, methods: Vec<Method> },
    Signal { name: String },
}

#[derive(Debug)]
struct Method {
    name: String,
    args: Vec<Argument>,
}

#[derive(Debug)]
struct Argument {
    name: String,
    typ: String,
    direction: Option<String>,
}

fn call(
    connection: &Connection,
    bus_name: &String,
    path: &String,
    interface_name: String,
    method_name: String,
    args: Option<&str>,
) {
    let entries = describe(bus_name, path, connection);

    let interface = entries.iter().find(|entry| {
        if let Entry::Interface { name, methods: _ } = entry {
            name.eq(&interface_name)
        } else {
            false
        }
    });

    println!("Found interface: {:?}\n", interface);

    if let Some(interface) = interface {
        if let Entry::Interface { name: _, methods } = interface {
            let method = methods
                .iter()
                .find(|method| method.name.as_str() == method_name.as_str());

            if let Some(method) = method {
                let signature: DBusType = method
                    .args
                    .iter()
                    .filter(|arg| arg.direction.eq(&Some("in".into())))
                    .map(|arg| arg.typ.clone())
                    .join("")
                    .as_str()
                    .into();

                let value: Option<DBusValue> = args.map(|args| args.into());

                println!("Found method: {:?}\n", method);
                println!("Signature: {:?}\n", Into::<String>::into(&signature));

                do_call(
                    connection,
                    bus_name,
                    path,
                    interface_name,
                    method_name,
                    value.as_ref().map(|value| DBusArgument {
                        dbus_type: &signature,
                        dbus_value: value,
                    }),
                );
            }
        }
    }
}

fn do_call(
    connection: &Connection,
    bus_name: &String,
    path: &String,
    interface_name: String,
    method_name: String,
    args: Option<DBusArgument>,
) {
    let mut message = Message::call_with_args(bus_name, path, interface_name, method_name, ());

    if let Some(args) = args {
        match args.validate() {
            Ok(arg) => {
                message.append_items(&[arg.into()]);
            }
            Err(e) => println!("Invalid argument: {:?}", e),
        }
    }

    let response = connection
        .channel()
        .send_with_reply_and_block(message, Duration::from_secs(1));

    let error = response.err().unwrap();

    println!("{}", error.name().unwrap());
    println!("{}", error.message().unwrap());
}

impl<'a> DBusArgument<'a> {
    pub fn validate(self) -> Result<DBusArgument<'a>, DBusError> {
        self.dbus_type
            .is_valid_value(&self.dbus_value)
            .map(|_| self)
    }
}

impl<'a> From<DBusArgument<'a>> for MessageItem {
    fn from(arg: DBusArgument) -> Self {
        match arg.dbus_type {
            DBusType::Boolean => {
                if let DBusValue::Boolean(value) = arg.dbus_value {
                    MessageItem::Bool(*value)
                } else {
                    panic!()
                }
            }
            DBusType::Byte => {
                if let DBusValue::Byte(value) = arg.dbus_value {
                    MessageItem::Byte(*value)
                } else {
                    panic!()
                }
            }
            DBusType::Int16 => {
                if let DBusValue::Int16(value) = arg.dbus_value {
                    MessageItem::Int16(*value)
                } else {
                    panic!()
                }
            }
            DBusType::Int32 => {
                if let DBusValue::Int32(value) = arg.dbus_value {
                    MessageItem::Int32(*value)
                } else {
                    panic!()
                }
            }
            DBusType::Int64 => {
                if let DBusValue::Int64(value) = arg.dbus_value {
                    MessageItem::Int64(*value)
                } else {
                    panic!()
                }
            }
            DBusType::UInt16 => {
                if let DBusValue::UInt16(value) = arg.dbus_value {
                    MessageItem::UInt16(*value)
                } else {
                    panic!()
                }
            }
            DBusType::UInt32 => {
                if let DBusValue::UInt32(value) = arg.dbus_value {
                    MessageItem::UInt32(*value)
                } else {
                    panic!()
                }
            }
            DBusType::UInt64 => {
                if let DBusValue::UInt64(value) = arg.dbus_value {
                    MessageItem::UInt64(*value)
                } else {
                    panic!()
                }
            }
            DBusType::Double => {
                if let DBusValue::Double(value) = arg.dbus_value {
                    MessageItem::Double(*value)
                } else {
                    panic!()
                }
            }
            DBusType::String => {
                if let DBusValue::String(value) = arg.dbus_value {
                    MessageItem::Str(value.clone())
                } else {
                    panic!()
                }
            }
            DBusType::ObjPath => {
                if let DBusValue::String(value) = arg.dbus_value {
                    MessageItem::ObjectPath(value.clone().into())
                } else {
                    panic!()
                }
            }
            DBusType::Signature => {
                if let DBusValue::String(value) = arg.dbus_value {
                    MessageItem::Signature(value.clone().into())
                } else {
                    panic!()
                }
            }
            DBusType::FileDescriptor => {
                if let DBusValue::String(_) = arg.dbus_value {
                    todo!()
                } else {
                    panic!()
                }
            }
            DBusType::Struct(types) => {
                if let DBusValue::Vec(values) = arg.dbus_value {
                    MessageItem::Struct(
                        types
                            .iter()
                            .zip(values.iter())
                            .map(|pair| {
                                DBusArgument {
                                    dbus_type: pair.0,
                                    dbus_value: pair.1,
                                }
                                .into()
                            })
                            .collect_vec(),
                    )
                } else {
                    panic!()
                }
            }
            DBusType::Array { value_type } => {
                if let DBusValue::Vec(values) = arg.dbus_value {
                    MessageItem::Array(
                        MessageItemArray::new(
                            values
                                .iter()
                                .map(|value| {
                                    DBusArgument {
                                        dbus_type: value_type,
                                        dbus_value: value,
                                    }
                                    .into()
                                })
                                .collect_vec(),
                            Into::<String>::into(value_type.as_ref()).into(),
                        )
                        .unwrap(),
                    )
                } else {
                    panic!()
                }
            }
            DBusType::Dictionary {
                key_type,
                value_type,
            } => {
                if let DBusValue::Vec(values) = arg.dbus_value {
                    MessageItem::Dict(
                        MessageItemDict::new(
                            values
                                .iter()
                                .step_by(2)
                                .zip(values.iter().skip(1).step_by(2))
                                .map(|entry| {
                                    (
                                        DBusArgument {
                                            dbus_type: key_type,
                                            dbus_value: entry.0,
                                        }
                                        .into(),
                                        DBusArgument {
                                            dbus_type: value_type,
                                            dbus_value: entry.1,
                                        }
                                        .into(),
                                    )
                                })
                                .collect_vec(),
                            Into::<String>::into(key_type.as_ref()).into(),
                            Into::<String>::into(value_type.as_ref()).into(),
                        )
                        .unwrap(),
                    )
                } else {
                    panic!()
                }
            }
            DBusType::Variant => todo!(),
        }
    }
}

fn list_names(connection: Connection) {
    let proxy = connection.with_proxy("org.freedesktop.DBus", "/", Duration::from_secs(1));
    let (names,): (Vec<String>,) = proxy
        .method_call("org.freedesktop.DBus", "ListNames", ())
        .unwrap();

    println!("bus names:\n");
    names.iter().for_each(|name| println!("    {}", name));
}

fn introspect(connection: &Connection, bus_name: &String, path: &String) {
    // let (nodes, interfaces) = do_introspect(connection, bus_name, path);

    let entries = describe(bus_name, path, connection);

    println!("paths:\n");
    entries.iter().for_each(|entry| match entry {
        Entry::Node { name } => print(1, name),
        _ => {}
    });

    println!("\ninterfaces:\n");
    entries.iter().for_each(|entry| match entry {
        Entry::Interface { name, methods: _ } => print(1, name),
        _ => {}
    });
}

fn build_connection(address: &str) -> Connection {
    if address.eq("session") {
        Connection::from(Channel::get_private(dbus::channel::BusType::Session).unwrap())
    } else if address.eq("system") {
        Connection::from(Channel::get_private(dbus::channel::BusType::Session).unwrap())
    } else {
        Connection::from(Channel::open_private(address).unwrap())
    }
}

fn describe(bus_name: &String, object_path: &String, connection: &Connection) -> Vec<Entry> {
    
    let proxy = connection.with_proxy(
        bus_name,
        object_path,
        Duration::from_secs(1),
    );

    let (capas,): (String,) = proxy
        .method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
        .unwrap();

    debug!("{:?}", capas);

    let mut entries = Vec::new();
    let parser = EventReader::from_str(capas.as_str());

    for e in parser {
        match e {
            Ok(elem) => {
                debug!("{:?}", elem);
                match elem {
                    XmlEvent::StartElement {
                        name,
                        attributes,
                        namespace: _,
                    } => match name.local_name.as_str() {
                        "node" => {
                            if attributes.get("name").is_some() {
                                entries.push(Entry::Node {
                                    name: attributes.get("name").unwrap().value.clone(),
                                })
                            }
                        }
                        "interface" => entries.push(Entry::Interface {
                            name: attributes.get("name").unwrap().value.clone(),
                            methods: Vec::new(),
                        }),
                        "signal" => entries.push(Entry::Signal {
                            name: attributes.get("name").unwrap().value.clone(),
                        }),
                        "method" => {
                            if let Entry::Interface { name: _, methods } =
                                entries.last_mut().unwrap()
                            {
                                (*methods).push(Method {
                                    name: attributes.get("name").unwrap().value.clone(),
                                    args: Vec::new(),
                                });
                            } else {
                            }
                        }
                        "arg" => {
                            if let Entry::Interface { name: _, methods } =
                                entries.last_mut().unwrap()
                            {
                                let method = methods.last_mut().unwrap();

                                method.args.push(Argument {
                                    name: attributes
                                        .get("name")
                                        .map(|attribute| attribute.value.clone())
                                        .unwrap_or("".into()),
                                    typ: attributes.get("type").unwrap().value.clone(),
                                    direction: attributes
                                        .get("direction")
                                        .map(|direction| direction.value.clone()),
                                });
                            } else {
                            }
                        }
                        _ => {}
                    },
                    // XmlEvent::EndElement { name } => todo!(),
                    _ => {}
                }
            }
            Err(err) => warn!("Xml error: {:?}", err),
        }
    }

    entries
}

trait Gettable {
    fn get(&self, name: &str) -> Option<&OwnedAttribute>;
}

impl Gettable for Vec<OwnedAttribute> {
    fn get(&self, name: &str) -> Option<&OwnedAttribute> {
        find_attribute(self, &name.into())
    }
}

fn find_attribute<'l>(attrs: &'l Vec<OwnedAttribute>, name: &String) -> Option<&'l OwnedAttribute> {
    attrs.iter().find(|attr| attr.name.local_name.eq(name))
}

fn print(indent: u32, subject: &String) {
    let ind = (0..indent)
        .into_iter()
        .map(|_| "    ")
        .collect::<Vec<&str>>()
        .join("");

    println!("{}{}", ind, subject);
}
