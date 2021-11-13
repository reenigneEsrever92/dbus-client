use std::{fmt::Display, time::Duration};

use dbus::{
    arg::RefArg,
    blocking::{Connection, Proxy},
};
use xml::{
    attribute::{self, Attribute, OwnedAttribute},
    name::OwnedName,
    reader::{Error, XmlEvent},
    EventReader,
};

fn main() {
    let connection = Connection::new_session().unwrap();
    let proxy = connection.with_proxy("org.freedesktop.DBus", "/", Duration::from_secs(1));

    get_node("/", proxy, 0);
}

fn get_node(
    path: &str,
    proxy: Proxy<&Connection>,
    iteration: u32,
) -> (Vec<String>, Vec<String>) {
    let (capas,): (String,) = proxy
        .method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
        .unwrap();

    let space = (0..iteration)
        .into_iter()
        .map(|_| "    ")
        .collect::<Vec<&str>>()
        .join("");

    let parser = EventReader::from_str(capas.as_str());

    let startElements = parser
        .into_iter()
        .filter(filter_events)
        .map(map_events)
        .filter(filter_start_elements)
        .map(map_start_elements);

    let nodes = startElements
        .filter(filter_nodes(String::from("node")))
        .filter(filter_with_attribute(String::from("name")))
        .map(map_to_attribute(String::from("name")));

    println!("{}{}", space, capas);

    (Vec::from_iter(nodes), Vec::new())
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
        name,
        attributes,
        namespace,
    } = event
    {
        (name, attributes)
    } else {
        panic!("")
    }
}

fn filter_nodes(node_type: String) -> impl Fn(&(OwnedName, Vec<OwnedAttribute>)) -> bool {
    move |el| el.0.local_name.eq(&node_type)
}

fn filter_with_attribute(attr_name: String) -> impl Fn(&(OwnedName, Vec<OwnedAttribute>)) -> bool {
    move |elem| {
        elem.1
            .iter()
            .find(|attr: &&OwnedAttribute| attr.name.local_name.eq(&attr_name))
            .is_some()
    }
}

fn map_to_attribute(attr_name: String) -> impl Fn((OwnedName, Vec<OwnedAttribute>)) -> String {
    move |elem| {
        let attr: Option<&OwnedAttribute> = elem
            .1
            .iter()
            .find(|attr: &&OwnedAttribute| attr.name.local_name.eq(&attr_name));
        attr.unwrap().value.clone()
    }
}

fn print(indent: u32, subject: String) {
    let ind = (0..indent)
        .into_iter()
        .map(|_| "    ")
        .collect::<Vec<&str>>()
        .join("");

    println!("{}{}", ind, subject);
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use dbus::blocking::Connection;

    #[test]
    fn test_get_bus_names() {
        let connection = Connection::new_session().unwrap();
        let proxy = connection.with_proxy("org.freedesktop.DBus", "/", Duration::from_secs(1));
        let (names,): (Vec<String>,) = proxy
            .method_call("org.freedesktop.DBus", "ListNames", ())
            .unwrap();
        for name in names {
            println!("{}", name);
        }
    }

    #[test]
    fn test_get_objects() {
        let connection = Connection::new_session().unwrap();
        let proxy = connection.with_proxy("org.gnome.Shell", "/", Duration::from_secs(1));
        let (capas,): (String,) = proxy
            .method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .unwrap();
        println!("{}", capas);
    }

    #[test]
    fn test_get_open_windows() {
        let connection = Connection::new_session().unwrap();
        let proxy = connection.with_proxy("org.gnome.Shell", "/", Duration::from_secs(1));
        let (capas,): (String,) = proxy
            .method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .unwrap();
        println!("{}", capas);
    }
}
