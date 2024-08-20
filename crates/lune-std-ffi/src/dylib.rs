use std::{
    collections::HashMap,
    ffi::{c_char, c_void, CString},
    ptr,
};

use dlopen2::raw::Library;
use libffi::{low::CodePtr, middle::Arg};
use mlua::{prelude::*, LightUserData};

use crate::{symbol::SymbolShape, types::FfiType};

#[derive(Debug, Clone)]
pub struct LibrarySymbol {
    pub ptr: *const c_void,
    pub shape: SymbolShape,
    pub result: FfiType,
}

#[derive(Debug)]
pub struct DynamicLibrary {
    symbols: HashMap<String, LibrarySymbol>,
}

impl DynamicLibrary {
    pub fn new(lib: Library, shape: Vec<SymbolShape>) -> Self {
        let symbols: HashMap<String, LibrarySymbol> = shape
            .iter()
            .map(|sym| unsafe {
                let ptr: *const c_void = lib.symbol(&sym.ident).unwrap();
                assert!(!ptr.is_null(), "Symbol {} not found", sym.ident);

                (
                    sym.ident.clone(),
                    LibrarySymbol {
                        ptr,
                        shape: sym.clone(),
                        result: sym.result.clone(),
                    },
                )
            })
            .collect();

        Self { symbols }
    }

    pub fn symbol<'lua>(&self, lua: &'lua Lua, ident: String) -> LuaResult<LuaFunction<'lua>> {
        let sym = self.symbols.get(&ident).unwrap().to_owned();

        lua.create_function(move |lua, args: LuaMultiValue| unsafe {
            println!("Started call routine {:#?}", sym.ptr);

            let result = sym.shape.clone().into_cif().call::<*const c_void>(
                CodePtr::from_ptr(sym.ptr),
                &args
                    .into_iter()
                    .map(|arg| match arg {
                        LuaValue::Integer(i) => Arg::new(&i),
                        LuaValue::Number(n) => Arg::new(&n),
                        LuaValue::String(s) => {
                            let c_str = CString::new(s.to_str().unwrap()).unwrap();
                            Arg::new(&c_str)
                        }
                        LuaValue::Boolean(b) => Arg::new(&b),
                        LuaValue::Nil => Arg::new(&ptr::null::<*const c_void>()),
                        LuaValue::Table(_) => todo!("structs?"),
                        LuaValue::LightUserData(ud) => Arg::new(&ud.0),
                        LuaValue::Function(_) => todo!("functions?"),
                        _ => todo!("idk"),
                    })
                    .collect::<Vec<Arg>>()[..],
            );

            println!("Called function");

            println!("Doing result conversion");
            Ok(match sym.result {
                FfiType::Void => LuaValue::Nil,

                FfiType::Struct(_) => todo!("idk"),
                FfiType::Int8 => LuaValue::Integer(*result.cast::<i8>() as i32),
                FfiType::Int16 => LuaValue::Integer(*result.cast::<i16>() as i32),
                FfiType::Int32 => LuaValue::Integer(*result.cast::<i32>()),
                FfiType::Int64 => LuaValue::Number(*result.cast::<i64>() as f64),
                FfiType::UInt8 => LuaValue::Integer(*result.cast::<u8>() as i32),
                FfiType::UInt16 => LuaValue::Integer(*result.cast::<u16>() as i32),
                FfiType::UInt32 => LuaValue::Integer(*result.cast::<u32>() as i32),
                FfiType::UInt64 => LuaValue::Number(*result.cast::<u64>() as f64),
                FfiType::Float32 => LuaValue::Number(*result.cast::<f32>() as f64),
                FfiType::Float64 => LuaValue::Number(*result.cast::<f64>()),
                FfiType::String => LuaValue::String(
                    lua.create_string(
                        CString::from_raw(result as *mut c_char)
                            .to_string_lossy()
                            .to_string(),
                    )?,
                ),

                FfiType::Pointer => LuaValue::LightUserData(LightUserData(result.cast_mut())),
            })
        })
    }
}

impl LuaUserData for DynamicLibrary {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("symbol", |lua, this, ident: String| this.symbol(lua, ident));
    }
}
