use libffi::middle::Type;
use mlua::prelude::*;

#[derive(Debug, Clone)]
pub enum FfiType {
    Void,
    Struct(Box<[FfiType]>),
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
    String,
    Pointer,
}

impl FfiType {
    pub fn into_type(self) -> Type {
        match self {
            Self::Void => Type::void(),
            Self::Struct(fields) => Type::structure(fields.iter().map(|f| f.clone().into_type())),
            Self::Int8 => Type::i8(),
            Self::Int16 => Type::i16(),
            Self::Int32 => Type::i32(),
            Self::Int64 => Type::i64(),
            Self::UInt8 => Type::u8(),
            Self::UInt16 => Type::u16(),
            Self::UInt32 => Type::u32(),
            Self::UInt64 => Type::u64(),
            Self::Float32 => Type::f32(),
            Self::Float64 => Type::f64(),
            Self::String => Type::pointer(), // FIXME: This needs to be a char* instead of void*
            Self::Pointer => Type::pointer(),
        }
    }
}

impl<'lua> FromLua<'lua> for FfiType {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::String(ref s) => match s.to_str()? {
                "void" => Ok(Self::Void),
                "int8" | "i8" | "signed char" => Ok(Self::Int8),
                "int16" | "i16" | "short" => Ok(Self::Int16),
                "int32" | "i32" | "int" => Ok(Self::Int32),
                "int64" | "i64" | "long" => Ok(Self::Int64),
                "uint8" | "u8" | "char" | "unsigned char" => Ok(Self::UInt8),
                "uint16" | "u16" | "unsigned short int" => Ok(Self::UInt16),
                "uint32" | "u32" | "unsigned int" => Ok(Self::UInt32),
                "uint64" | "u64" | "unsigned long" => Ok(Self::UInt64),
                "float32" | "f32" | "float" => Ok(Self::Float32),
                "float64" | "f64" | "double" => Ok(Self::Float64),
                "string" | "char*" => Ok(Self::String),
                "pointer" | "void*" => Ok(Self::Pointer),
                other => Err(LuaError::FromLuaConversionError {
                    from: value.type_name(),
                    to: "FfiType",
                    message: Some(
                        format!(
                            "Invalid FfiType - expected a valid C or Rust-like type, got {other}"
                        )
                        .to_string(),
                    ),
                }),
            },

            LuaValue::Table(table) => {
                let fields = table
                    .pairs()
                    .collect::<LuaResult<Vec<(String, Self)>>>()?
                    .iter()
                    .map(|(_, ty)| ty.clone())
                    .collect::<Box<[FfiType]>>();

                Ok(Self::Struct(fields))
            }

            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "FfiType",
                message: Some(format!(
                    "Invalid FfiType - expected string or table, got {}",
                    value.type_name()
                )),
            }),
        }
    }
}
