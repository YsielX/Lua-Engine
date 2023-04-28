pub mod binary_chunk;
mod reader;
mod header_const;
mod tag_const;

pub fn undump(data: Vec<u8>) -> Box<binary_chunk::Prototype> {
    let mut reader = reader::Reader{data:data};
    reader.check_header();
    reader.read_byte();
    reader.read_proto(&String::from(""))
}