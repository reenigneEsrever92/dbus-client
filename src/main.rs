use std::{env, time::Duration};

use dbus::blocking::Connection;
use xml::{
    attribute::OwnedAttribute,
    name::OwnedName,
    reader::{Error, XmlEvent},
    EventReader,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1);

    match command {
        Some(command) => { 
            if command.eq(&String::from("list")) {
                list_names();
                return;
            } else if command.eq(&String::from("intro")) {
                introspect(args);
                return;
            } 
        },
        None => {},
    }

    println!("usage:\n");
    println!("    {} [command]\n", args.get(0).unwrap());
    println!("commands:\n");
    println!("    list");
    println!("    intro [bus] [path]")
}

fn list_names() {
    let connection = Connection::new_session().unwrap();
    let proxy = connection.with_proxy("org.freedesktop.DBus", "/", Duration::from_secs(1));
    let (names, ): (Vec<String>, ) = proxy.method_call("org.freedesktop.DBus", "ListNames", ()).unwrap();

    println!("bus names:\n");
    names.iter().for_each(|name| println!("    {}", name));
}

fn introspect(args: Vec<String>) {
    let bus_name = args.get(2).unwrap().to_owned();

    let path = args.get(3).unwrap().to_owned();

    let (nodes, interfaces) = do_introspect(bus_name, path);

    println!("object paths:\n");
    nodes.iter().for_each(|node| print(1, node));

    println!("\ninterfaces:\n");
    interfaces.iter().for_each(|interface| print(1, interface));
}

fn do_introspect(bus_name: String, object_path: String) -> (Vec<String>, Vec<String>) {
    let connection = Connection::new_session().unwrap();

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

    let parser = EventReader::from_str(capas.as_str());

    let start_elements = parser
        .into_iter()
        .filter(filter_events)
        .map(map_events)
        .filter(filter_start_elements)
        .map(map_start_elements)
        .collect::<Vec<(OwnedName, Vec<OwnedAttribute>)>>();

    let nodes = start_elements
        .iter()
        .filter(filter_nodes(String::from("node")))
        .filter(filter_with_attribute(String::from("name")))
        .map(map_to_attribute(String::from("name")));

    let interfaces = start_elements
        .iter()
        .filter(filter_nodes(String::from("interface")))
        .filter(filter_with_attribute(String::from("name")))
        .map(map_to_attribute(String::from("name")));

    // println!("{}", capas);

    (Vec::from_iter(nodes), Vec::from_iter(interfaces))
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
