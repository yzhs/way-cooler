//! Awesome compatibility modules

use wlroots;

use awesome::lua::setup_lua;
use rlua::{self, LightUserData, Lua, Table};
use std::{env, mem, path::PathBuf};
use xcb::{xkb, Connection};

mod awesome;
mod button;
mod class;
mod client;
mod drawable;
mod drawin;
mod key;
pub mod keygrabber;
pub mod lua;
mod mouse;
pub mod mousegrabber;
mod object;
mod property;
mod root;
mod screen;
pub mod signal;
mod tag;
mod xproperty;

pub use self::lua::{LUA, NEXT_LUA};

pub use self::drawin::{Drawin, DRAWINS_HANDLE};
pub use self::key::Key;
pub use self::keygrabber::keygrabber_handle;
pub use self::mousegrabber::mousegrabber_handle;
pub use self::object::{Object, Objectable};
pub use self::root::ROOT_KEYS_HANDLE;
pub use self::signal::*;

use compositor::Server;

pub const GLOBAL_SIGNALS: &'static str = "__awesome_global_signals";
pub const XCB_CONNECTION_HANDLE: &'static str = "__xcb_connection";

/// Called from `wayland_glib_interface.c` after every call back into the
/// wayland event loop.
///
/// This restarts the Lua thread if there is a new one pending
#[no_mangle]
pub extern "C" fn refresh_awesome() {
    NEXT_LUA.with(|new_lua_check| {
                      if new_lua_check.get() {
                          new_lua_check.set(false);
                          LUA.with(|lua| {
                                       let mut lua = lua.borrow_mut();
                                       unsafe {
                                           *lua = rlua::Lua::new_with_debug();
                                       }
                                   });
                          let compositor = wlroots::compositor_handle().unwrap();
                          setup_lua(compositor);
                      }
                  });
}

pub fn init(lua: &Lua, server: &mut Server) -> rlua::Result<()> {
    setup_awesome_path(lua)?;
    setup_global_signals(lua)?;
    setup_xcb_connection(lua)?;
    button::init(lua)?;
    awesome::init(lua)?;
    key::init(lua)?;
    client::init(lua)?;
    screen::init(lua, server)?;
    keygrabber::init(lua)?;
    root::init(lua)?;
    mouse::init(lua)?;
    tag::init(lua)?;
    drawin::init(lua)?;
    drawable::init(lua)?;
    mousegrabber::init(lua)?;
    Ok(())
}

fn setup_awesome_path(lua: &Lua) -> rlua::Result<()> {
    let globals = lua.globals();
    let package: Table = globals.get("package")?;
    let mut path = package.get::<_, String>("path")?;
    let mut cpath = package.get::<_, String>("cpath")?;

    for mut xdg_data_path in
        env::var("XDG_DATA_DIRS").unwrap_or("/usr/local/share:/usr/share".into())
                                 .split(':')
                                 .map(PathBuf::from)
    {
        xdg_data_path.push("awesome/lib");
        path.push_str(&format!(";{0}/?.lua;{0}/?/init.lua",
                               xdg_data_path.as_os_str().to_string_lossy()));
        cpath.push_str(&format!(";{}/?.so", xdg_data_path.into_os_string().to_string_lossy()));
    }

    for mut xdg_config_path in env::var("XDG_CONFIG_DIRS").unwrap_or("/etc/xdg".into())
                                                          .split(':')
                                                          .map(PathBuf::from)
    {
        xdg_config_path.push("awesome");
        cpath.push_str(&format!(";{}/?.so",
                                xdg_config_path.into_os_string().to_string_lossy()));
    }

    package.set("path", path)?;
    package.set("cpath", cpath)?;

    Ok(())
}

/// Set up global signals value
///
/// We need to store this in Lua, because this make it safer to use.
fn setup_global_signals(lua: &Lua) -> rlua::Result<()> {
    lua.set_named_registry_value(GLOBAL_SIGNALS, lua.create_table()?)
}

/// Sets up the xcb connection and stores it in Lua (for us to access it later)
fn setup_xcb_connection(lua: &Lua) -> rlua::Result<()> {
    let con = match Connection::connect(None) {
        Err(err) => {
            error!("Way Cooler requires XWayland in order to function");
            error!("However, xcb could not connect to it. Is it running?");
            error!("{:?}", err);
            panic!("Could not connect to XWayland instance");
        }
        Ok(con) => con.0
    };
    // Tell xcb we are using the xkb extension
    match xkb::use_extension(&con, 1, 0).get_reply() {
        Ok(r) => {
            if !r.supported() {
                panic!("xkb-1.0 is not supported");
            }
        }
        Err(err) => {
            panic!("Could not get xkb extension supported version {:?}", err);
        }
    }
    lua.set_named_registry_value(XCB_CONNECTION_HANDLE,
                                  LightUserData(con.get_raw_conn() as _))?;
    mem::forget(con);
    Ok(())
}

pub fn dummy<'lua>(_: &'lua Lua, _: rlua::Value) -> rlua::Result<()> {
    Ok(())
}
