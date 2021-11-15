use std::{
    env,
    iter::{Filter, Map},
    ops::Deref,
    slice::Iter,
    time::Duration,
};

use clap::{App, Arg, SubCommand};
use dbus::{blocking::Connection, channel::Channel};
use log::{debug, Level, LevelFilter};
use simple_logger::SimpleLogger;
use xml::{
    attribute::OwnedAttribute,
    name::OwnedName,
    reader::{Error, XmlEvent},
    EventReader,
};

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
                    Arg::with_name("BUS_NAME")
                        .required(true)
                        .help("Name of the bus"),
                )
                .arg(
                    Arg::with_name("PATH")
                        .required(true)
                        .help("Path of the object to introspect"),
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
                .short("a")
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

    if let Some(_) = matches.subcommand_matches("list-names") {
        let connection = build_connection(matches.value_of("address").unwrap_or_default());
        list_names(connection);
    }

    if let Some(cmd) = matches.subcommand_matches("introspect") {
        let connection = build_connection(matches.value_of("address").unwrap_or_default());
        introspect(
            connection,
            cmd.value_of("BUS_NAME").unwrap().into(),
            cmd.value_of("PATH").unwrap().into(),
        );
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

impl Entry {
    fn is_node(&self) -> bool {
        match self {
            Entry::Node { .. } => true,
            _ => false,
        }
    }

    fn is_interface(&self) -> bool {
        match self {
            Entry::Interface { .. } => true,
            _ => false,
        }
    }

    fn is_method(&self) -> bool {
        match self {
            Entry::Method { .. } => true,
            _ => false,
        }
    }

    fn is_arg(&self) -> bool {
        match self {
            Entry::Arg { .. } => true,
            _ => false,
        }
    }

    fn get_name(&self) -> &String {
        match self {
            Entry::Node { name } => name,
            Entry::Interface { name } => name,
            Entry::Method { name } => name,
            Entry::Arg {
                name,
                typ,
                direction,
            } => name,
        }
    }

    fn get_type(&self) -> Option<&String> {
        match self {
            Entry::Arg { name, typ, .. } => Some(typ),
            _ => None,
        }
    }

    fn get_direction(&self) -> &Option<String> {
        match self {
            Entry::Arg {
                name: _,
                typ: _,
                direction,
            } => direction,
            _ => &None,
        }
    }
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

fn list_names(connection: Connection) {
    let proxy = connection.with_proxy("org.freedesktop.DBus", "/", Duration::from_secs(1));
    let (names,): (Vec<String>,) = proxy
        .method_call("org.freedesktop.DBus", "ListNames", ())
        .unwrap();

    println!("bus names:\n");
    names.iter().for_each(|name| println!("    {}", name));
}

fn introspect(connection: Connection, bus_name: String, path: String) {
    let (nodes, interfaces) = do_introspect(connection, bus_name, path);

    println!("paths:\n");
    nodes.iter().for_each(|node| print(1, node));

    println!("\ninterfaces:\n");
    interfaces.iter().for_each(|interface| print(1, interface));
}

fn do_introspect(
    connection: Connection,
    bus_name: String,
    object_path: String,
) -> (Vec<String>, Vec<String>) {
    let start_elements = get_capas(bus_name, object_path, connection);

    debug!("{:?}", start_elements);

    (get_nodes(&start_elements), get_interfaces(&start_elements))
}

fn get_methods(elements: &Vec<Entry>) -> Vec<(String, String, String)> {
    elements
        .iter()
        .filter(|el| el.is_method())
        .map(|el| {
            (
                el.get_name().clone(),
                el.get_type().unwrap().clone(),
                el.get_direction().to_owned()
                    .unwrap_or_else(|| String::from("")),
            )
        })
        .collect()
}

fn get_interfaces(elements: &Vec<Entry>) -> Vec<String> {
    elements
        .iter()
        .filter(|el| el.is_interface())
        .map(|el| el.get_name().clone())
        .collect()
}

fn get_nodes(elements: &Vec<Entry>) -> Vec<String> {
    elements
        .iter()
        .filter(|el| el.is_node())
        .map(|el| el.get_name().clone())
        .collect()
}

fn get_capas(bus_name: String, object_path: String, connection: Connection) -> Vec<Entry> {
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
        .filter(filter_with_attribute("name".into()))
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

fn filter_with_attribute(attr_name: String) -> impl Fn(&XmlEvent) -> bool {
    move |elem| match elem {
        XmlEvent::StartElement {
            name: _,
            attributes,
            ..
        } => attributes.get("name").is_some(),
        _ => false,
    }
}

fn map_to_attribute(attr_name: String) -> impl Fn(&(OwnedName, Vec<OwnedAttribute>)) -> String {
    move |elem| {
        let attr: Option<&OwnedAttribute> = elem
            .1
            .iter()
            .find(|attr: &&OwnedAttribute| attr.name.local_name.eq(&attr_name));
        attr.unwrap().value.clone()
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
