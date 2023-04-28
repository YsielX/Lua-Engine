mod binchunk;
use binchunk::binary_chunk::{Prototype, Constant};
use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let mut args = env::args();
    let _program = args.next().expect("no program name");
    let arg1 = args.next().expect("no first argument");
    let data = fs::read(arg1).expect("Cannot open file");
    
    let proto = binchunk::undump(data);
    list(&proto);

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
    let func_type = if f.line_defined > 0 { "function" } else { "main" };
    let vararg_flag = if f.is_vararg > 0 { "+" } else { "" };

    println!("{} <{}:{},{}> ({} instructions)",func_type,f.source,f.line_defined,f.last_line_defined,f.code.len());
    println!("{}{} params, {} slots, {} upvalues, {} locals, {} constants, {} functions",
        f.num_params,vararg_flag,f.max_stack_size,f.upvalues.len(),f.loc_vars.len(),f.constants.len(),f.protos.len());
}

fn print_code(f: &Box<Prototype>) {
    for pc in 0..f.code.len() {
        let line = if !f.line_info.is_empty() { format!("{}", f.line_info[pc]) } else { String::from("-") };
        println!("\t{}\t[{}]\t0x\t{:08X}", pc + 1, line, f.code[pc]);
    }
}

fn print_detail(f: &Box<Prototype>) {
    println!("constants ({}):", f.constants.len());
    for i in 0..f.constants.len() {
        println!("\t{}\t{}", i+1, constant_to_string(&f.constants[i]));
    }

    println!("locals ({}):", f.loc_vars.len());
    for i in 0..f.loc_vars.len() {
        let loc_var = &f.loc_vars[i];
        println!("\t{}\t{}\t{}\t{}", i, loc_var.var_name, loc_var.start_pc+1, loc_var.end_pc+1);
    }

    println!("upvalues ({}):", f.upvalues.len());
    for i in 0..f.upvalues.len() {
        let upval = &f.upvalues[i];
        println!("\t{}\t{}\t{}\t{}", i, upval_name(&f, i), upval.instack, upval.idx);
    }
}

fn constant_to_string(k: &Constant) -> String {
    match k {
        Constant::Nil => format!("nil"),
        Constant::Boolean(arg) => format!("{}", arg),
        Constant::Integer(arg) => format!("{}", arg),
        Constant::Number(arg) => format!("{}", arg),
        Constant::Str(arg) => format!("{}", arg)
    }
}

fn upval_name(f: &Prototype, idx: usize) -> String {
    f.upvalue_names.get(idx).unwrap_or(&String::from("-")).clone()
}