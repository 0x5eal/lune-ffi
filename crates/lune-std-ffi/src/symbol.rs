use libffi::middle::{Builder, Cif};
use mlua::prelude::*;

use crate::types::FfiType;

#[derive(Debug, Clone)]
pub struct SymbolShape {
    pub ident: String,
    pub parameters: Vec<FfiType>,
    pub result: FfiType,
}

impl SymbolShape {
    pub fn into_cif(self) -> Cif {
        Builder::new()
            .args(self.parameters.iter().map(|p| p.clone().into_type()))
            .res(self.result.into_type())
            .into_cif()
    }
}

impl FromLua<'_> for SymbolShape {
    fn from_lua(value: LuaValue, _lua: &Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(table) => {
                let ident = table.get::<_, String>("name")?;
                let parameters = table.get::<_, Vec<FfiType>>("parameters")?;
                let result = table.get::<_, FfiType>("result")?;

                Ok(Self {
                    ident,
                    parameters,
                    result,
                })
            }

            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "SymbolShape",
                message: Some(format!(
                    "Invalid symbol shape - expected table or nil, got {}",
                    value.type_name()
                )),
            }),
        }
    }
}
