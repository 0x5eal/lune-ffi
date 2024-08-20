#![allow(clippy::cargo_common_metadata)]

use dlopen2::raw::Library;
use lune_utils::TableBuilder;
use mlua::prelude::*;

mod dylib;
pub(crate) mod symbol;
pub(crate) mod types;

/**
    Creates the `ffi` standard library module.

    # Errors

    Errors when out of memory.
*/
pub fn module(lua: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(lua)?
        .with_function("load", ffi_load)?
        .build_readonly()
}

fn ffi_load(
    _: &Lua,
    (path, shape): (String, Vec<symbol::SymbolShape>),
) -> LuaResult<dylib::DynamicLibrary> {
    let lib = Library::open(&path).into_lua_err()?;
    Ok(dylib::DynamicLibrary::new(lib, shape))
}
