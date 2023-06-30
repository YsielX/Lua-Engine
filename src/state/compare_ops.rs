use crate::state::lua_value::LuaValue;

pub fn _eq(a: &LuaValue, b: &LuaValue) -> bool {
    match a {
        LuaValue::Nil => if let LuaValue::Nil = b {
            true
        } else {
            false
        }
        LuaValue::Boolean(x) => if let LuaValue::Boolean(y) = b {
            x == y
        } else {
            false
        }
        LuaValue::Str(x) => if let LuaValue::Str(y) = b {
            x == y
        } else {
            false
        }
        LuaValue::Integer(x) => match b {
            LuaValue::Integer(y) => x == y,
            LuaValue::Number(y) => (*x as f64) == *y,
            _ => false
        }
        LuaValue::Number(x) => match b {
            LuaValue::Integer(y) => *x == (*y as f64),
            LuaValue::Number(y) => x == y,
            _ => false
        }
        _ => unimplemented!()
    }
}

pub fn _lt(a: &LuaValue, b: &LuaValue) -> Option<bool> {
    match a {
        LuaValue::Str(x) => if let LuaValue::Str(y) = b {
            Some(x < y)
        } else {
            None
        }
        LuaValue::Integer(x) => match b {
            LuaValue::Integer(y) => Some(x < y),
            LuaValue::Number(y) => Some((*x as f64) < *y),
            _ => None
        }
        LuaValue::Number(x) => match b {
            LuaValue::Integer(y) => Some(*x < (*y as f64)),
            LuaValue::Number(y) => Some(x < y),
            _ => None
        }
        _ => None
    }
}

pub fn _le(a: &LuaValue, b: &LuaValue) -> Option<bool> {
    match a {
        LuaValue::Str(x) => if let LuaValue::Str(y) = b {
            Some(x <= y)
        } else {
            None
        }
        LuaValue::Integer(x) => match b {
            LuaValue::Integer(y) => Some(x <= y),
            LuaValue::Number(y) => Some((*x as f64) <= *y),
            _ => None
        }
        LuaValue::Number(x) => match b {
            LuaValue::Integer(y) => Some(*x <= (*y as f64)),
            LuaValue::Number(y) => Some(x <= y),
            _ => None
        }
        _ => None
    }
}