mod vm;
mod api;
mod state;
mod binchunk;
use std::env;
use std::fs;
use std::io;
use api::lua_state::LuaAPI;
use vm::instruction::Instruction;
use vm::opcodes::{OpArgMode, OpMode};
use binchunk::binary_chunk::{Constant, Prototype};

// fn main() -> io::Result<()> {
//     let mut args = env::args();
//     let _program = args.next().expect("no program name");
//     let arg1 = args.next().expect("no first argument");
//     let data = fs::read(arg1).expect("Cannot open file");

//     let binarychunk = binchunk::undump(data);
//     list(&binarychunk.main_func);

//     Ok(())
// }

fn main() -> io::Result<()> {
    let mut ls = state::lua_state::LuaState::new();
    ls.push_boolean(true);
    println!("{:#?}",ls);
    ls.push_integer(10);
    println!("{:#?}",ls);
    ls.push_nil();
    println!("{:#?}",ls);
    ls.push_string(String::from("hello"));
    println!("{:#?}",ls);
    ls.push_value(-4);
    println!("{:#?}",ls);
    ls.replace(3);
    println!("{:#?}",ls);
    ls.set_top(6);
    println!("{:#?}",ls);
    ls.remove(-3);
    println!("{:#?}",ls);
    ls.set_top(-5);
    println!("{:#?}",ls);

    Ok(())
}

fn list(f: &Box<Prototype>) {
    print_header(&f);
    print_code(&f);
    print_detail(&f);
    for p in &f.protos {
        list(p);
    }
}

fn print_header(f: &Box<Prototype>) {
    let func_type = if f.line_defined > 0 {
        "function"
    } else {
        "main"
    };
    let vararg_flag = if f.is_vararg > 0 { "+" } else { "" };

    println!(
        "{} <{}:{},{}> ({} instructions)",
        func_type,
        f.source,
        f.line_defined,
        f.last_line_defined,
        f.code.len()
    );
    println!(
        "{}{} params, {} slots, {} upvalues, {} locals, {} constants, {} functions",
        f.num_params,
        vararg_flag,
        f.max_stack_size,
        f.upvalues.len(),
        f.loc_vars.len(),
        f.constants.len(),
        f.protos.len()
    );
}

fn print_code(f: &Box<Prototype>) {
    for pc in 0..f.code.len() {
        let line = if !f.line_info.is_empty() {
            format!("{}", f.line_info[pc])
        } else {
            String::from("-")
        };
        let i = f.code[pc];
        print!("\t{}\t[{}]\t{} \t", pc + 1, line, i.opname());
        print_operands(i);
    }
}

fn print_operands(i: u32) {
    match i.opmode() {
        OpMode::IABC => {
            let (a, b, c) = i.abc();
            print!("{}", a);
            match i.b_mode() {
                OpArgMode::OpArgN => (),
                _ => print!(" {}", if b > 0xff { -1 - (b & 0xff) } else { b }),
            }
            match i.c_mode() {
                OpArgMode::OpArgN => (),
                _ => print!(" {}", if c > 0xff { -1 - (c & 0xff) } else { c }),
            }
        }
        OpMode::IABx => {
            let (a, bx) = i.a_bx();
            print!("{}", a);
            match i.b_mode() {
                OpArgMode::OpArgK => print!(" {}", -1 - bx),
                OpArgMode::OpArgU => print!(" {}", bx),
                _ => unreachable!(),
            }
        }
        OpMode::IAsBx => {
            let (a, sbx) = i.a_sbx();
            print!("{} {}", a, sbx);
        }
        OpMode::IAx => {
            let ax = i.ax();
            print!("{}", -1 - ax);
        }
    }
    println!();
}

fn print_detail(f: &Box<Prototype>) {
    println!("constants ({}):", f.constants.len());
    for i in 0..f.constants.len() {
        println!("\t{}\t{}", i + 1, constant_to_string(&f.constants[i]));
    }

    println!("locals ({}):", f.loc_vars.len());
    for i in 0..f.loc_vars.len() {
        let loc_var = &f.loc_vars[i];
        println!(
            "\t{}\t{}\t{}\t{}",
            i,
            loc_var.var_name,
            loc_var.start_pc + 1,
            loc_var.end_pc + 1
        );
    }

    println!("upvalues ({}):", f.upvalues.len());
    for i in 0..f.upvalues.len() {
        let upval = &f.upvalues[i];
        println!(
            "\t{}\t{}\t{}\t{}",
            i,
            upval_name(&f, i),
            upval.instack,
            upval.idx
        );
    }
}

fn constant_to_string(k: &Constant) -> String {
    match k {
        Constant::Nil => format!("nil"),
        Constant::Boolean(arg) => format!("{}", arg),
        Constant::Integer(arg) => format!("{}", arg),
        Constant::Number(arg) => format!("{}", arg),
        Constant::Str(arg) => format!("{}", arg),
    }
}

fn upval_name(f: &Prototype, idx: usize) -> String {
    f.upvalue_names
        .get(idx)
        .unwrap_or(&String::from("-"))
        .clone()
}
