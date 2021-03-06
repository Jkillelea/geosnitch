// This code was autogenerated with dbus-codegen-rust, see https://github.com/diwic/dbus-rs

#![allow(dead_code)]
use dbus as dbus;
use dbus::arg;
use dbus::tree;

pub trait OrgFreedesktopGeoClue2Location {
    type Err;
    fn get_latitude(&self) -> Result<f64, Self::Err>;
    fn get_longitude(&self) -> Result<f64, Self::Err>;
    fn get_accuracy(&self) -> Result<f64, Self::Err>;
    fn get_altitude(&self) -> Result<f64, Self::Err>;
    fn get_speed(&self) -> Result<f64, Self::Err>;
    fn get_heading(&self) -> Result<f64, Self::Err>;
    fn get_description(&self) -> Result<String, Self::Err>;
    fn get_timestamp(&self) -> Result<(u64, u64), Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgFreedesktopGeoClue2Location for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn get_latitude(&self) -> Result<f64, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.GeoClue2.Location", "Latitude")
    }

    fn get_longitude(&self) -> Result<f64, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.GeoClue2.Location", "Longitude")
    }

    fn get_accuracy(&self) -> Result<f64, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.GeoClue2.Location", "Accuracy")
    }

    fn get_altitude(&self) -> Result<f64, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.GeoClue2.Location", "Altitude")
    }

    fn get_speed(&self) -> Result<f64, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.GeoClue2.Location", "Speed")
    }

    fn get_heading(&self) -> Result<f64, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.GeoClue2.Location", "Heading")
    }

    fn get_description(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.GeoClue2.Location", "Description")
    }

    fn get_timestamp(&self) -> Result<(u64, u64), Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.GeoClue2.Location", "Timestamp")
    }
}

pub fn org_freedesktop_geo_clue2_location_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    D::Property: Default,
    T: OrgFreedesktopGeoClue2Location<Err=tree::MethodErr>,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.GeoClue2.Location", data);
    let f = ::std::sync::Arc::new(f);
    let p = factory.property::<f64, _>("Latitude", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(try!(d.get_latitude()));
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<f64, _>("Longitude", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(try!(d.get_longitude()));
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<f64, _>("Accuracy", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(try!(d.get_accuracy()));
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<f64, _>("Altitude", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(try!(d.get_altitude()));
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<f64, _>("Speed", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(try!(d.get_speed()));
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<f64, _>("Heading", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(try!(d.get_heading()));
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<&str, _>("Description", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(try!(d.get_description()));
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<(u64, u64), _>("Timestamp", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(try!(d.get_timestamp()));
        Ok(())
    });
    let i = i.add_p(p);
    i
}
