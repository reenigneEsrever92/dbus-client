use std::time::Duration;

use clap::{App, Arg, ArgGroup, SubCommand, Values};
use dbus::{
    arg::{
        messageitem::{MessageItem, MessageItemArray, MessageItemDict},
        ArgType, RefArg,
    },
    blocking::{BlockingSender, Connection},
    channel::Channel,
    message, Message, Signature,
};
use itertools::Itertools;
use log::{debug, LevelFilter};
use parser::Parser;
use simple_logger::SimpleLogger;
use dbus_type::DBusType;
use xml::{
    attribute::OwnedAttribute,
    reader::{Error, XmlEvent},
    EventReader,
};

mod nom_parser;
mod parser;
mod dbus_type;
mod value;
mod argument;

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
                connection,
                cmd.value_of("bus-name").unwrap().into(),
                cmd.value_of("path").unwrap().into(),
            );
        }
        ("call", Some(cmd)) => {
            let args: Vec<&str> = cmd
                .values_of("argument")
                .unwrap_or(Values::default())
                .collect();
            call(
                connection,
                cmd.value_of("bus-name").unwrap().into(),
                cmd.value_of("path").unwrap().into(),
                cmd.value_of("interface").unwrap().into(),
                cmd.value_of("method").unwrap().into(),
            )
        }
        _ => {
            println!("{}", matches.usage())
        }
    }
}

#[derive(Debug)]
enum Entry {
    Node {
        name: String,
    },
    Interface {
        name: String,
    },
    Method {
        name: String,
    },
    Arg {
        name: String,
        typ: String,
        direction: Option<String>,
    },
}

fn call(
    connection: Connection,
    bus_name: String,
    path: String,
    interface_name: String,
    method_name: String,
) {
    let mut message = Message::call_with_args(bus_name, path, interface_name, method_name, ());

    // if let Signature::Array(_) = &args {
    //     if let MessageItem::Array(array) = convert(&args) {
    //         array.into_vec().into_iter().for_each(|arg| {
    //             message.append_items(&[arg]);
    //         });
    //     }
    // }

    // if let Signature::Struct(value) = &args {
    //     value.iter().for_each(|val| {
    //         message.append_items(&[convert(&val)])
    //     });
    // }

    // args.into_iter().map(|arg| ArgType::);

    let response = connection
        .channel()
        .send_with_reply_and_block(message, Duration::from_secs(1));

    response
        .unwrap()
        .iter_init()
        .for_each(|arg| println!("{:?}", arg));
}

fn list_names(connection: Connection) {
    let proxy = connection.with_proxy("org.freedesktop.DBus", "/", Duration::from_secs(1));
    let (names,): (Vec<String>,) = proxy
        .method_call("org.freedesktop.DBus", "ListNames", ())
        .unwrap();

    println!("bus names:\n");
    names.iter().for_each(|name| println!("    {}", name));
}

fn introspect(connection: Connection, bus_name: String, path: String) {
    // let (nodes, interfaces) = do_introspect(connection, bus_name, path);

    let entries = get_entries(bus_name, path, connection);

    println!("paths:\n");
    entries.iter().for_each(|entry| match entry {
        Entry::Node { name } => print(1, name),
        _ => {}
    });

    println!("\ninterfaces:\n");
    entries.iter().for_each(|entry| match entry {
        Entry::Interface { name } => print(1, name),
        Entry::Method { name } => print(2, name),
        Entry::Arg {
            name,
            typ,
            direction,
        } => print(
            3,
            &format!(
                "name: {} typ: {} direction: {}",
                name,
                typ,
                direction.to_owned().unwrap_or("".into())
            ),
        ),
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

fn get_entries(bus_name: String, object_path: String, connection: Connection) -> Vec<Entry> {
    let proxy = connection.with_proxy(
        bus_name,
        if object_path.starts_with("/") {
            object_path
        } else {
            format!("/{}", object_path)
        },
        Duration::from_secs(1),
    );

    let (capas,): (String,) = proxy
        .method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
        .unwrap();

    debug!("{:?}", capas);

    let parser = EventReader::from_str(capas.as_str());

    parser
        .into_iter()
        .filter(filter_events)
        .map(map_events)
        .filter(filter_start_elements)
        .filter(filter_with_attribute())
        .map(map_start_elements)
        .filter(|el| el.is_some())
        .map(|el| el.unwrap())
        .collect()
}

fn filter_events(event: &Result<XmlEvent, Error>) -> bool {
    match event {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn map_events(event: Result<XmlEvent, Error>) -> XmlEvent {
    match event {
        Ok(event) => event,
        Err(_) => todo!(),
    }
}

fn filter_start_elements(event: &XmlEvent) -> bool {
    match event {
        XmlEvent::StartElement { .. } => true,
        _ => false,
    }
}

fn map_start_elements(event: XmlEvent) -> Option<Entry> {
    if let XmlEvent::StartElement {
        name, attributes, ..
    } = event
    {
        match name.local_name.as_str() {
            "node" => Some(Entry::Node {
                name: attributes.get("name").unwrap().value.clone(),
            }),
            "interface" => Some(Entry::Interface {
                name: attributes.get("name").unwrap().value.clone(),
            }),
            "method" => Some(Entry::Method {
                name: attributes.get("name").unwrap().value.clone(),
            }),
            "arg" => Some(Entry::Arg {
                name: attributes.get("name").unwrap().value.clone(),
                typ: attributes.get("type").unwrap().value.clone(),
                direction: attributes
                    .get("direction")
                    .map(|direction| direction.value.clone()),
            }),
            _ => None,
        }
    } else {
        panic!("")
    }
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

fn filter_with_attribute() -> impl Fn(&XmlEvent) -> bool {
    move |elem| match elem {
        XmlEvent::StartElement {
            name: _,
            attributes,
            ..
        } => attributes.get("name").is_some(),
        _ => false,
    }
}

fn print(indent: u32, subject: &String) {
    let ind = (0..indent)
        .into_iter()
        .map(|_| "    ")
        .collect::<Vec<&str>>()
        .join("");

    println!("{}{}", ind, subject);
}
