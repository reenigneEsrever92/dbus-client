use std::{convert::Infallible, ops::Not, time::Duration};

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
use log::{debug, info, warn, LevelFilter};
use simple_logger::SimpleLogger;
use xml::{
    attribute::OwnedAttribute,
    reader::{Error, XmlEvent},
    EventReader,
};

use crate::dbus_argument::DBusArgument;

mod dbus_argument;
mod dbus_error;
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
            cmd.value_of("argument").unwrap_or(""),
        ),
        _ => {
            println!("{}", matches.usage())
        }
    }
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
    args: &str,
) {
    let entries = describe(bus_name, path, connection);

    let interface = entries.iter().find(|entry| {
        if let Entry::Interface { name, methods: _ } = entry {
            name.eq(&interface_name)
        } else {
            false
        }
    });

    debug!("Found interface: {:?}\n", interface);

    if let Some(interface) = interface {
        if let Entry::Interface { name: _, methods } = interface {
            let method = methods
                .iter()
                .find(|method| method.name.as_str() == method_name.as_str());

            if let Some(method) = method {
                debug!("Found method: {:?}\n", method);

                let signature = method
                    .args
                    .iter()
                    .filter(|arg| arg.direction.eq(&Some("in".into())))
                    .map(|arg| arg.typ.clone())
                    .join("");

                debug!("Signature: {:?}\n", Into::<String>::into(&signature));

                let dbus_type: DBusType = if signature.is_empty() { 
                    signature.as_str().into()
                } else {
                    format!("({})", signature).as_str().into()
                };
                
                let dbus_value: DBusValue = args.into();

                do_call(
                    connection,
                    bus_name,
                    path,
                    interface_name,
                    method_name,
                    DBusArgument {
                        dbus_type: &dbus_type,
                        dbus_value: &dbus_value,
                    },
                );
            } else {
                println!("No method: {} found!", method_name)
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
    args: DBusArgument,
) {
    let mut message = Message::call_with_args(bus_name, path, interface_name, method_name, ());

    match args.validate() {
        Ok(args) => {
            if let DBusType::Struct(vec) = args.dbus_type {
                if let Some(message_item) = args.into() {
                    if let MessageItem::Struct(vec) = message_item {
                        vec.into_iter().for_each(|arg| {
                            message.append_items(&[arg]);
                        });
                    } else {
                        panic!("Top level argument has to be a struct")
                    }
                }
            }
        }
        Err(e) => panic!("Invalid argument: {:?}", e),
    }

    let response = connection
        .channel()
        .send_with_reply_and_block(message, Duration::from_secs(1));

    println!("{:?}", response.unwrap())
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
        Entry::Interface { name, methods } => {
            print(1, name);
            methods.iter().for_each(|method| {
                print(
                    2,
                    &format!(
                        "{} ( {} ) -> {}",
                        &method.name,
                        method
                            .args
                            .iter()
                            .filter(|arg| arg.direction.eq(&Some("in".into())))
                            .map(|arg| arg.typ.clone())
                            .join(""),
                        method
                            .args
                            .iter()
                            .filter(|arg| arg.direction.eq(&Some("out".into())))
                            .map(|arg| arg.typ.clone())
                            .join("")
                    ),
                );
            })
        }
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
    let proxy = connection.with_proxy(bus_name, object_path, Duration::from_secs(1));

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
