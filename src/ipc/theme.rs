use dbus::arg::{Array};
use dbus::tree::MethodErr;
use dbus::MessageItem;

use rustc_serialize::json::{self, Json, ToJson};
use ::registry;
use uuid::Uuid;

use super::utils::{parse_uuid, lock_tree_dbus};
use super::{DBusFactory, DBusObjPath};

pub fn setup(f: &mut DBusFactory) -> DBusObjPath {
    f.object_path("/org/way_cooler/Theme", ()).introspectable().add(
        f.interface("org.way_cooler.Theme", ())
            .add_m(
                f.method("GetWindows", (), |m| {
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
        // TODO Document
        // This does a join-like update on the settings
        // E.G: You don't need to specify all the values,
        // just the ones that change
            .add_m(
                f.method("SetWindows", (), |m| {
                    let mut args_iter = m.msg.iter_init();
                    let user_input = args_iter.read::<String>()?;
                    let json = Json::from_str(&*user_input).map_err(|err| {
                        MethodErr::failed(&format!("{:?}", err))
                    })?;
                    if !json.is_object() {
                        return Err(MethodErr::failed(&"JSON must be an object"))
                    }
                    let json = json.as_object().unwrap();

                    let lock = registry::clients_write();
                    let client = lock.client(Uuid::nil()).unwrap();
                    let mut handle = registry::WriteHandle::new(&client);
                    handle.write("windows".into()).ok()
                        .and_then(|windows| {
                            for (key, value) in json.iter() {
                                windows.insert(key.clone(), value.clone());
                            }
                            Some(())
                        }).expect("Could not add data to windows category");
                    Ok(vec![m.msg.method_return()
                            .append(true)
                    ])
                })
                    .outarg::<bool, _>("success")
                    .inarg::<String, _>("json_data")
            )
    )
}
