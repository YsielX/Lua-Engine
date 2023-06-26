use super::lua_value::LuaValue;

pub const OPS: &'static [(fn(i64, i64) -> i64, fn(f64, f64) -> f64)] = &[
    (|a, b| a+b, |a, b| a+b),
    (|a, b| a-b, |a, b| a-b),
    (|a, b| a*b, |a, b| a*b),
    (i_mod, f_mod),
    (inone, |a, b| a.powf(b)),
    (inone, |a, b| a/b),
    (i_floor_div, f_floor_div),
    (|a, b| a&b, fnone),
    (|a, b| a|b, fnone),
    (|a, b| a^b, fnone),
    (shift_left, fnone),
    (shift_right, fnone),
    (|a, _b| -a, |a, _b| -a),
    (|a, _b| !a, fnone),
];

pub fn _arith(a: &LuaValue, b: &LuaValue, op: u8) -> Option<LuaValue> {
    let iop = OPS[op as usize].0;
    let fop = OPS[op as usize].1;
    if fop == fnone {
        if let Some(x) = a.to_integer() {
            if let Some(y) = b.to_integer() {
                return Some(LuaValue::Integer(iop(x, y)));
            }
        }
    } else {
        if iop != inone {
            if let Some(x) = a.to_integer() {
                if let Some(y) = b.to_integer() {
                    return Some(LuaValue::Integer(iop(x, y)));
                }
            }
        }
        if let Some(x) = a.to_number() {
            if let Some(y) = b.to_number() {
                return Some(LuaValue::Number(fop(x, y)));
            }
        }
    }
    None
}


fn inone(_a: i64, _b: i64) -> i64 {
    0
}

fn fnone(_a: f64, _b: f64) -> f64 {
    0.0
}

fn is_positive_infinite(n: f64) -> bool {
    n.is_infinite() && n.is_sign_positive()
}

fn is_negative_infinite(n: f64) -> bool {
    n.is_infinite() && n.is_sign_negative()
}

fn i_floor_div(a: i64, b: i64) -> i64 {
    if a>0 && b>0 || a<0 && b<0 || a%b==0 {
        a/b
    } else {
        a/b-1
    }
}

fn f_floor_div(a: f64, b: f64) -> f64 {
    (a/b).floor()
}

fn i_mod(a: i64, b: i64) -> i64 {
    a-i_floor_div(a, b)*b
}

fn f_mod(a: f64, b: f64) -> f64 {
    if a > 0.0 && is_positive_infinite(b) || a < 0.0 && is_negative_infinite(b) {
        a
    } else if a > 0.0 && is_negative_infinite(b) || a < 0.0 && is_positive_infinite(b) {
        b
    } else {
        a - (a / b).floor() * b
    }
}

fn shift_left(a: i64, n: i64) -> i64 {
    if n>=0 {
        a << n
    } else {
        (a as u64 >> -n) as i64
    }
}

fn shift_right(a: i64, n: i64) -> i64 {
    if n>=0 {
        (a as u64 >> n) as i64
    } else {
        a << -n
    }
}