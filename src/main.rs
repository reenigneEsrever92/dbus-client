use std::{
    env,
    iter::{Filter, Map},
    ops::Deref,
    slice::Iter,
    time::Duration,
};

use clap::{App, Arg, SubCommand};
use dbus::{blocking::Connection, channel::Channel};
use log::{Level, LevelFilter, debug};
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
        0 => SimpleLogger::new().with_level(LevelFilter::Error).init().unwrap(),
        1 => SimpleLogger::new().with_level(LevelFilter::Warn).init().unwrap(),
        2 => SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap(),
        3 | _ => SimpleLogger::new().with_level(LevelFilter::Debug).init().unwrap(),
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

    (get_nodes(&start_elements), get_interfaces(&start_elements))
}

fn get_methods(elements: &Vec<(OwnedName, Vec<OwnedAttribute>)>) -> Vec<String> {
    elements
        .iter()
        .filter(filter_nodes(String::from("interface")))
        .filter(filter_with_attribute(String::from("name")))
        .map(map_to_attribute(String::from("name")))
        .collect()
}

fn get_interfaces(elements: &Vec<(OwnedName, Vec<OwnedAttribute>)>) -> Vec<String> {
    elements
        .iter()
        .filter(filter_nodes(String::from("interface")))
        .filter(filter_with_attribute(String::from("name")))
        .map(map_to_attribute(String::from("name")))
        .collect()
}

fn get_nodes(elements: &Vec<(OwnedName, Vec<OwnedAttribute>)>) -> Vec<String> {
    elements
        .iter()
        .filter(filter_nodes(String::from("node")))
        .filter(filter_with_attribute(String::from("name")))
        .map(map_to_attribute(String::from("name")))
        .collect()
}

fn get_capas(
    bus_name: String,
    object_path: String,
    connection: Connection,
) -> Vec<(OwnedName, Vec<OwnedAttribute>)> {
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
        .map(map_start_elements)
        .collect::<Vec<(OwnedName, Vec<OwnedAttribute>)>>()
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

fn map_start_elements(event: XmlEvent) -> (OwnedName, Vec<OwnedAttribute>) {
    if let XmlEvent::StartElement {
        name, attributes, ..
    } = event
    {
        (name, attributes)
    } else {
        panic!("")
    }
}

fn filter_nodes(node_type: String) -> impl Fn(&&(OwnedName, Vec<OwnedAttribute>)) -> bool {
    move |el| el.0.local_name.eq(&node_type)
}

fn filter_with_attribute(attr_name: String) -> impl Fn(&&(OwnedName, Vec<OwnedAttribute>)) -> bool {
    move |elem| {
        elem.1
            .iter()
            .find(|attr: &&OwnedAttribute| attr.name.local_name.eq(&attr_name))
            .is_some()
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
