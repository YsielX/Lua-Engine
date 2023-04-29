pub mod binary_chunk;
mod reader;
mod header_const;
mod tag_const;

pub fn undump(data: Vec<u8>) -> binary_chunk::BinaryChunk {
    let mut reader = reader::Reader{data};
    reader.read_binary_chunk()
}