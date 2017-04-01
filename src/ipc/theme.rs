use dbus::arg::{Array};
use dbus::tree::MethodErr;
use dbus::MessageItem;

use rustc_serialize::json::{Json, ToJson};
use ::registry;
use uuid::Uuid;

use super::utils::{parse_uuid, lock_tree_dbus};
use super::{DBusFactory, DBusObjPath};

pub fn setup(f: &mut DBusFactory) -> DBusObjPath {
    f.object_path("/org/way_cooler/Theme", ()).introspectable().add(
        f.interface("org.way_cooler.Theme", ())
            .add_m(
                f.method("Windows", (), |m| {
                    let lock = registry::clients_read();
                    let client = lock.client(Uuid::nil()).unwrap();
                    let handle = registry::ReadHandle::new(&client);
                    let json: Json = handle.read("windows".into()).ok()
                        .expect("No windows category").to_json();
                    Ok(vec![m.msg.method_return()
                            .append(format!("{}", json))
                    ])
                }).outarg::<String, _>("json")
            )
    )
}
