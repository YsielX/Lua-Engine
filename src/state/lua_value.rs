use crate::api::consts;

#[derive(Clone, Debug)]
pub enum LuaValue {
    Nil,
    Boolean(bool),
    Number(f64),
    Integer(i64),
    Str(String)
}

impl LuaValue {
    pub fn ty(&self) -> i8 {
        match self {
            LuaValue::Nil => consts::LUA_TNIL,
            LuaValue::Boolean(_) => consts::LUA_TBOOLEAN,
            LuaValue::Integer(_) => consts::LUA_TNUMBER,
            LuaValue::Number(_) => consts::LUA_TNUMBER,
            LuaValue::Str(_) => consts::LUA_TSTRING
        }
    }

}