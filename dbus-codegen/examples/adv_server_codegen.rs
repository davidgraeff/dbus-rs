// This is a modified version of dbus/examples/adv_server.rs that uses code generated from dbus-codegen.

extern crate dbus;

//
// Usually you would put imported code in a separate .rs file, but I've just copy-pasted it into
// a module here, for simplicity.

mod com_example_dbus_rs {
// === Imported code start ===

// This code was autogenerated with dbus-codegen-rust, see https://github.com/diwic/dbus-rs

#![allow(dead_code)]
use dbus as dbus;
use dbus::arg;
use dbus::tree;

pub trait Device {
    type Err;
    fn check(&self) -> Result<(), Self::Err>;
    fn get_checking(&self) -> Result<bool, Self::Err>;
    fn get_description(&self) -> Result<String, Self::Err>;
    fn get_online(&self) -> Result<bool, Self::Err>;
    fn set_online(&self, value: bool) -> Result<(), Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> Device for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn check(&self) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"com.example.dbus.rs.device".into(), &"check".into(), |_| {
        })?;
        m.as_result()?;
        Ok(())
    }

    fn get_checking(&self) -> Result<bool, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "com.example.dbus.rs.device", "checking")
    }

    fn get_description(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "com.example.dbus.rs.device", "description")
    }

    fn get_online(&self) -> Result<bool, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "com.example.dbus.rs.device", "online")
    }

    fn set_online(&self, value: bool) -> Result<(), Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::set(&self, "com.example.dbus.rs.device", "online", value)
    }
}

pub fn device_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    D::Property: Default,
    D::Signal: Default,
    T: Device<Err=tree::MethodErr>,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("com.example.dbus.rs.device", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        d.check()?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("check", Default::default(), h);
    let i = i.add_m(m);

    let p = factory.property::<bool, _>("checking", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_checking()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<&str, _>("description", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_description()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<bool, _>("online", Default::default());
    let p = p.access(tree::Access::ReadWrite);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_online()?);
        Ok(())
    });
    let fclone = f.clone();
    let p = p.on_set(move |iter, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        d.set_online(iter.read()?)?;
        Ok(())
    });
    let i = i.add_p(p);
    let s = factory.signal("CheckComplete", Default::default());
    let i = i.add_s(s);
    i
}

#[derive(Debug)]
pub struct DeviceCheckComplete {
}

impl arg::AppendAll for DeviceCheckComplete {
    fn append(&self, _: &mut arg::IterAppend) {
    }
}

impl arg::ReadAll for DeviceCheckComplete {
    fn read(_: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(DeviceCheckComplete {
        })
    }
}

impl dbus::message::SignalArgs for DeviceCheckComplete {
    const NAME: &'static str = "CheckComplete";
    const INTERFACE: &'static str = "com.example.dbus.rs.device";
}

// === Imported code end ===
}

use dbus::{Connection, BusType, tree, Path};
use dbus::tree::{Interface, MTFn, MethodErr};

use std::sync::Arc;
use std::sync::mpsc;
use std::cell::Cell;
use std::thread;


// Our storage device
#[derive(Debug)]
struct Device {
    description: String,
    path: Path<'static>,
    index: i32,
    online: Cell<bool>,
    checking: Cell<bool>,
    check_complete_sender: mpsc::Sender<i32>,
}

// Every storage device has its own object path.
// We therefore create a link from the object path to the Device.
#[derive(Copy, Clone, Default, Debug)]
struct TData;
impl tree::DataType for TData {
    type Tree = ();
    type ObjectPath = Arc<Device>;
    type Property = ();
    type Interface = ();
    type Method = ();
    type Signal = ();
}


impl Device {
    // Creates a "test" device (not a real one, since this is an example).
    fn new_bogus(index: i32, s: mpsc::Sender<i32>) -> Device {
        Device {
            description: format!("This is device {}, which is {}.", index,
                ["totally awesome", "really fancy", "still going strong"][(index as usize) % 3]),
            path: format!("/Device{}", index).into(),
            index: index,
            online: Cell::new(index % 2 == 0),
            checking: Cell::new(false),
            check_complete_sender: s,
        }
    } 
}

// Here's where we implement the code for our interface.
impl com_example_dbus_rs::Device for Device {
    type Err = tree::MethodErr;
    fn check(&self) -> Result<(), Self::Err> {
        if self.checking.get() {
            return Err(MethodErr::failed(&"Device currently under check, cannot start another check"))
        }
        if self.online.get() {
            return Err(MethodErr::failed(&"Device is currently online, cannot start check"))
        }
        self.checking.set(true);

        // Start some lengthy processing in a separate thread...
        let devindex = self.index;
        let ch = self.check_complete_sender.clone();
        thread::spawn(move || {

            // Bogus check of device
            use std::time::Duration;
            thread::sleep(Duration::from_secs(15));

            // Tell main thread that we finished
            ch.send(devindex).unwrap();
        });
        Ok(())
    }
    fn get_checking(&self) -> Result<bool, Self::Err> {
        Ok(self.checking.get())
    }
    fn get_description(&self) -> Result<String, Self::Err> {
        Ok(self.description.clone())
    }
    fn get_online(&self) -> Result<bool, Self::Err> {
        Ok(self.online.get())
    }
    fn set_online(&self, value: bool) -> Result<(), Self::Err> {
        if value && self.checking.get() {
            Err(MethodErr::failed(&"Device currently under check, cannot bring online")) }
        else {
            self.online.set(value);
            Ok(())
        }
    }
}

fn create_iface() -> Interface<MTFn<TData>, TData> {
    let f = tree::Factory::new_fn();
    com_example_dbus_rs::device_server(&f, (), |m| {
        // Just provide a link from MethodInfo (m) to the &Device
        // we should call.
        let a: &Arc<Device> = m.path.get_data();
        let b: &Device = &a;
        b
    })
}

fn create_tree(devices: &[Arc<Device>], iface: &Arc<Interface<MTFn<TData>, TData>>)
    -> tree::Tree<MTFn<TData>, TData> {

    let f = tree::Factory::new_fn();
    let mut tree = f.tree(());
    for dev in devices {
        tree = tree.add(f.object_path(dev.path.clone(), dev.clone())
            .introspectable()
            .add(iface.clone())
        );
    }
    tree 
}

fn run() -> Result<(), Box<std::error::Error>> {
    let (check_complete_s, check_complete_r) = mpsc::channel::<i32>(); 

    // Create our bogus devices
    let devices: Vec<Arc<Device>> = (0..10).map(|i|
        Arc::new(Device::new_bogus(i, check_complete_s.clone()))
    ).collect();

    // Create tree
    let iface = create_iface();
    let tree = create_tree(&devices, &Arc::new(iface));

    // Setup DBus connection
    let c = Connection::get_private(BusType::Session)?;
    c.register_name("com.example.dbus.rs.advancedserverexample", 0)?;
    tree.set_registered(&c, true)?;

    // ...and serve incoming requests.
    c.add_handler(tree);
    loop {
        // Wait for incoming messages. This will block up to one second.
        // Discard the result - relevant messages have already been handled.
        c.incoming(1000).next();

        // Do all other things we need to do in our main loop.
        if let Ok(idx) = check_complete_r.try_recv() {
            let dev = &devices[idx as usize];
            dev.checking.set(false);
            let sig = com_example_dbus_rs::DeviceCheckComplete {};
            use dbus::message::SignalArgs;
            c.send(sig.to_emit_message(&dev.path))
                .map_err(|_| "Sending DBus signal failed")?;
        }
    }
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}
