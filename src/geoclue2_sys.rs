// This code was autogenerated with dbus-codegen-rust, see https://github.com/diwic/dbus-rs

#![allow(dead_code)]
use dbus as dbus;
use dbus::arg;
use dbus::tree;

pub trait OrgFreedesktopDBusProperties {
    type Err;
    fn get(&self, interface_name: &str, property_name: &str) -> Result<arg::Variant<Box<arg::RefArg>>, Self::Err>;
    fn get_all(&self, interface_name: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>, Self::Err>;
    fn set(&self, interface_name: &str, property_name: &str, value: arg::Variant<Box<arg::RefArg>>) -> Result<(), Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgFreedesktopDBusProperties for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn get(&self, interface_name: &str, property_name: &str) -> Result<arg::Variant<Box<arg::RefArg>>, Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.DBus.Properties".into(), &"Get".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(interface_name);
            i.append(property_name);
        }));
        try!(m.as_result());
        let mut i = m.iter_init();
        let value: arg::Variant<Box<arg::RefArg>> = try!(i.read());
        Ok(value)
    }

    fn get_all(&self, interface_name: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>, Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.DBus.Properties".into(), &"GetAll".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(interface_name);
        }));
        try!(m.as_result());
        let mut i = m.iter_init();
        let properties: ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>> = try!(i.read());
        Ok(properties)
    }

    fn set(&self, interface_name: &str, property_name: &str, value: arg::Variant<Box<arg::RefArg>>) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.DBus.Properties".into(), &"Set".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(interface_name);
            i.append(property_name);
            i.append(value);
        }));
        try!(m.as_result());
        Ok(())
    }
}

pub fn org_freedesktop_dbus_properties_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    T: OrgFreedesktopDBusProperties<Err=tree::MethodErr>,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.DBus.Properties", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let interface_name: &str = try!(i.read());
        let property_name: &str = try!(i.read());
        let d = fclone(minfo);
        let value = try!(d.get(interface_name, property_name));
        let rm = minfo.msg.method_return();
        let rm = rm.append1(value);
        Ok(vec!(rm))
    };
    let m = factory.method("Get", Default::default(), h);
    let m = m.in_arg(("interface_name", "s"));
    let m = m.in_arg(("property_name", "s"));
    let m = m.out_arg(("value", "v"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let interface_name: &str = try!(i.read());
        let d = fclone(minfo);
        let properties = try!(d.get_all(interface_name));
        let rm = minfo.msg.method_return();
        let rm = rm.append1(properties);
        Ok(vec!(rm))
    };
    let m = factory.method("GetAll", Default::default(), h);
    let m = m.in_arg(("interface_name", "s"));
    let m = m.out_arg(("properties", "a{sv}"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let interface_name: &str = try!(i.read());
        let property_name: &str = try!(i.read());
        let value: arg::Variant<Box<arg::RefArg>> = try!(i.read());
        let d = fclone(minfo);
        try!(d.set(interface_name, property_name, value));
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Set", Default::default(), h);
    let m = m.in_arg(("interface_name", "s"));
    let m = m.in_arg(("property_name", "s"));
    let m = m.in_arg(("value", "v"));
    let i = i.add_m(m);
    i
}

#[derive(Debug, Default)]
pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties: ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>,
    pub invalidated_properties: Vec<String>,
}

impl dbus::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
    fn append(&self, i: &mut arg::IterAppend) {
        (&self.interface_name as &arg::RefArg).append(i);
        (&self.changed_properties as &arg::RefArg).append(i);
        (&self.invalidated_properties as &arg::RefArg).append(i);
    }
    fn get(&mut self, i: &mut arg::Iter) -> Result<(), arg::TypeMismatchError> {
        self.interface_name = try!(i.read());
        self.changed_properties = try!(i.read());
        self.invalidated_properties = try!(i.read());
        Ok(())
    }
}

pub trait OrgFreedesktopDBusIntrospectable {
    type Err;
    fn introspect(&self) -> Result<String, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgFreedesktopDBusIntrospectable for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn introspect(&self) -> Result<String, Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.DBus.Introspectable".into(), &"Introspect".into(), |_| {
        }));
        try!(m.as_result());
        let mut i = m.iter_init();
        let xml_data: String = try!(i.read());
        Ok(xml_data)
    }
}

pub fn org_freedesktop_dbus_introspectable_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    T: OrgFreedesktopDBusIntrospectable<Err=tree::MethodErr>,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.DBus.Introspectable", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        let xml_data = try!(d.introspect());
        let rm = minfo.msg.method_return();
        let rm = rm.append1(xml_data);
        Ok(vec!(rm))
    };
    let m = factory.method("Introspect", Default::default(), h);
    let m = m.out_arg(("xml_data", "s"));
    let i = i.add_m(m);
    i
}

pub trait OrgFreedesktopDBusPeer {
    type Err;
    fn ping(&self) -> Result<(), Self::Err>;
    fn get_machine_id(&self) -> Result<String, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgFreedesktopDBusPeer for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn ping(&self) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.DBus.Peer".into(), &"Ping".into(), |_| {
        }));
        try!(m.as_result());
        Ok(())
    }

    fn get_machine_id(&self) -> Result<String, Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.DBus.Peer".into(), &"GetMachineId".into(), |_| {
        }));
        try!(m.as_result());
        let mut i = m.iter_init();
        let machine_uuid: String = try!(i.read());
        Ok(machine_uuid)
    }
}

pub fn org_freedesktop_dbus_peer_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    T: OrgFreedesktopDBusPeer<Err=tree::MethodErr>,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.DBus.Peer", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        try!(d.ping());
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Ping", Default::default(), h);
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        let machine_uuid = try!(d.get_machine_id());
        let rm = minfo.msg.method_return();
        let rm = rm.append1(machine_uuid);
        Ok(vec!(rm))
    };
    let m = factory.method("GetMachineId", Default::default(), h);
    let m = m.out_arg(("machine_uuid", "s"));
    let i = i.add_m(m);
    i
}

pub trait OrgFreedesktopGeoClue2Manager {
    type Err;
    fn get_client(&self) -> Result<dbus::Path<'static>, Self::Err>;
    fn add_agent(&self, id: &str) -> Result<(), Self::Err>;
    fn get_in_use(&self) -> Result<bool, Self::Err>;
    fn get_available_accuracy_level(&self) -> Result<u32, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgFreedesktopGeoClue2Manager for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn get_client(&self) -> Result<dbus::Path<'static>, Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.GeoClue2.Manager".into(), &"GetClient".into(), |_| {
        }));
        try!(m.as_result());
        let mut i = m.iter_init();
        let client: dbus::Path<'static> = try!(i.read());
        Ok(client)
    }

    fn add_agent(&self, id: &str) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.GeoClue2.Manager".into(), &"AddAgent".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(id);
        }));
        try!(m.as_result());
        Ok(())
    }

    fn get_in_use(&self) -> Result<bool, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.GeoClue2.Manager", "InUse")
    }

    fn get_available_accuracy_level(&self) -> Result<u32, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.GeoClue2.Manager", "AvailableAccuracyLevel")
    }
}

pub fn org_freedesktop_geo_clue2_manager_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    D::Property: Default,
    T: OrgFreedesktopGeoClue2Manager<Err=tree::MethodErr>,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.GeoClue2.Manager", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        let client = try!(d.get_client());
        let rm = minfo.msg.method_return();
        let rm = rm.append1(client);
        Ok(vec!(rm))
    };
    let m = factory.method("GetClient", Default::default(), h);
    let m = m.out_arg(("client", "o"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let id: &str = try!(i.read());
        let d = fclone(minfo);
        try!(d.add_agent(id));
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("AddAgent", Default::default(), h);
    let m = m.in_arg(("id", "s"));
    let i = i.add_m(m);

    let p = factory.property::<bool, _>("InUse", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(try!(d.get_in_use()));
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<u32, _>("AvailableAccuracyLevel", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(try!(d.get_available_accuracy_level()));
        Ok(())
    });
    let i = i.add_p(p);
    i
}
