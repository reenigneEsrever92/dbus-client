use dbus::blocking::{Connection, Proxy};

fn main() {
}

fn discover_tree(path: String, proxy: Proxy<&Connection>) {
    let (capas,): (String,) = proxy.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ()).unwrap();
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use dbus::blocking::Connection;

    #[test]
    fn test_get_bus_names() {
        let connection = Connection::new_session().unwrap();
        let proxy = connection.with_proxy("org.freedesktop.DBus", "/", Duration::from_secs(1));
        let (names,): (Vec<String>,) = proxy.method_call("org.freedesktop.DBus", "ListNames", ()).unwrap();
        for name in names { println!("{}", name); }
    }

    #[test]
    fn test_get_objects() {
        let connection = Connection::new_session().unwrap();
        let proxy = connection.with_proxy("org.gnome.Shell", "/", Duration::from_secs(1));
        let (capas,): (String,) = proxy.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ()).unwrap();
        println!("{}", capas);
    }

    #[test]
    fn test_get_open_windows() {
        let connection = Connection::new_session().unwrap();
        let proxy = connection.with_proxy("org.gnome.Shell", "/", Duration::from_secs(1));
        let (capas,): (String,) = proxy.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ()).unwrap();
        println!("{}", capas);
    }
}