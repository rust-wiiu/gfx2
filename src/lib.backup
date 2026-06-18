use byteorder::{BigEndian, ReadBytesExt};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use std::io::Cursor;

/// Extract a null-terminated string from a buffer
fn extract_string(cursor: &mut Cursor<&[u8]>, data: &[u8]) -> String {
    let offset = (cursor.read_u32::<BigEndian>().unwrap() & 0x000F_FFFF) as usize;

    std::ffi::CStr::from_bytes_until_nul(&data[offset..])
        .unwrap()
        .to_string_lossy()
        .to_string()
}

/// Extract an array of structs from a buffer
fn extract_ptr<T: Parse>(cursor: &mut Cursor<&[u8]>, data: &[u8]) -> Vec<T> {
    let len = cursor.read_u32::<BigEndian>().unwrap() as usize;
    let offset = (cursor.read_u32::<BigEndian>().unwrap() & 0x000F_FFFF) as usize;

    (0..len)
        .map(|i| T::parse(&data, offset + i * T::BYTES))
        .collect()
}

trait Parse {
    const BYTES: usize;

    fn parse(data: &[u8], offset: usize) -> Self;
}

#[derive(
    Debug, Clone, Copy, PartialEq, IntoPrimitive, TryFromPrimitive, Serialize, Deserialize,
)]
#[repr(u32)]
enum BlockType {
    EndOfFile = 1,
    Padding = 2,
    VertexHeader = 3,
    VertexProgram = 5,
    FragmentHeader = 6,
    FragmentProgram = 7,
    GeometryHeader = 8,
    GeometryProgram = 9,
    GeometryCopyProgram = 10,
    TextureHeader = 11,
    TextureImageData = 12,
    TextureMipmipData = 13,
    ComputeHeader = 14,
    ComputeProgram = 15,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Block {
    EndOfFile,
    Padding,
    VertexHeader(VertexHeader),
    VertexProgram(Vec<u8>),
    FragmentHeader(FragmentHeader),
    FragmentProgram(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VertexHeader {
    #[serde(with = "BigArray")]
    pub registers: [u32; 52],
    pub program_size: u32,
    // pub program_ptr: u32,
    pub shader_mode: ShaderMode,
    pub uniform_blocks: Vec<UniformBlock>,
    pub uniform_vars: Vec<UniformVar>,
    pub initial_values: Vec<InitialValue>,
    pub loop_vars: Vec<LoopVar>,
    pub sampler_vars: Vec<SamplerVar>,
    pub attrib_vars: Vec<AttributeVar>,
    pub ring_item_size: u32,
    pub has_stream_out: u32,
    pub stream_out_stride: [u32; 4],
    pub buffer: Buffer,
}

impl Parse for VertexHeader {
    const BYTES: usize = 0x134;

    fn parse(data: &[u8], offset: usize) -> Self {
        let mut cursor = Cursor::new(&data[offset..offset + Self::BYTES]);

        let registers = std::array::from_fn(|_| cursor.read_u32::<BigEndian>().unwrap());
        let program_size = cursor.read_u32::<BigEndian>().unwrap();
        assert_eq!(cursor.read_u32::<BigEndian>().unwrap(), 0); // program_ptr
        let shader_mode = ShaderMode::try_from(cursor.read_u32::<BigEndian>().unwrap()).unwrap();
        let uniform_blocks = extract_ptr(&mut cursor, data);
        let uniform_vars = extract_ptr(&mut cursor, data);
        let initial_values = extract_ptr(&mut cursor, data);
        let loop_vars = extract_ptr(&mut cursor, data);
        let sampler_vars = extract_ptr(&mut cursor, data);
        let attrib_vars = extract_ptr(&mut cursor, data);
        let ring_item_size = cursor.read_u32::<BigEndian>().unwrap();
        let has_stream_out = cursor.read_u32::<BigEndian>().unwrap();
        let stream_out_stride = std::array::from_fn(|_| cursor.read_u32::<BigEndian>().unwrap());
        let buffer = Buffer {
            flags: cursor.read_u32::<BigEndian>().unwrap(),
            elem_size: cursor.read_u32::<BigEndian>().unwrap(),
            len: cursor.read_u32::<BigEndian>().unwrap(),
            ptr: cursor.read_u32::<BigEndian>().unwrap(),
        };

        Self {
            registers,
            program_size,
            shader_mode,
            uniform_blocks,
            uniform_vars,
            initial_values,
            loop_vars,
            sampler_vars,
            attrib_vars,
            ring_item_size,
            has_stream_out,
            stream_out_stride,
            buffer,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FragmentHeader {
    #[serde(with = "BigArray")]
    pub registers: [u32; 41],
    pub program_size: u32,
    // pub program_ptr: u32,
    pub shader_mode: ShaderMode,
    pub uniform_blocks: Vec<UniformBlock>,
    pub uniform_vars: Vec<UniformVar>,
    pub initial_values: Vec<InitialValue>,
    pub loop_vars: Vec<LoopVar>,
    pub sampler_vars: Vec<SamplerVar>,
    pub buffer: Buffer,
}

impl Parse for FragmentHeader {
    const BYTES: usize = 0xE8;

    fn parse(data: &[u8], offset: usize) -> Self {
        let mut cursor = Cursor::new(&data[offset..offset + Self::BYTES]);

        let registers = std::array::from_fn(|_| cursor.read_u32::<BigEndian>().unwrap());
        let program_size = cursor.read_u32::<BigEndian>().unwrap();
        assert_eq!(cursor.read_u32::<BigEndian>().unwrap(), 0); // program_ptr
        let shader_mode = ShaderMode::try_from(cursor.read_u32::<BigEndian>().unwrap()).unwrap();
        let uniform_blocks = extract_ptr(&mut cursor, data);
        let uniform_vars = extract_ptr(&mut cursor, data);
        let initial_values = extract_ptr(&mut cursor, data);
        let loop_vars = extract_ptr(&mut cursor, data);
        let sampler_vars = extract_ptr(&mut cursor, data);
        let buffer = Buffer {
            flags: cursor.read_u32::<BigEndian>().unwrap(),
            elem_size: cursor.read_u32::<BigEndian>().unwrap(),
            len: cursor.read_u32::<BigEndian>().unwrap(),
            ptr: cursor.read_u32::<BigEndian>().unwrap(),
        };

        Self {
            registers,
            program_size,
            shader_mode,
            uniform_blocks,
            uniform_vars,
            initial_values,
            loop_vars,
            sampler_vars,
            buffer,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UniformBlock {
    pub name: String,
    pub offset: u32,
    pub size: u32,
}

impl Parse for UniformBlock {
    const BYTES: usize = 12;

    fn parse(data: &[u8], offset: usize) -> Self {
        let mut cursor = Cursor::new(&data[offset..offset + Self::BYTES]);

        Self {
            name: extract_string(&mut cursor, data),
            offset: cursor.read_u32::<BigEndian>().unwrap(),
            size: cursor.read_u32::<BigEndian>().unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UniformVar {
    pub name: String,
    pub ty: VarType,
    pub count: u32,
    pub offset: u32,
    pub block: u32,
}

impl Parse for UniformVar {
    const BYTES: usize = 20;

    fn parse(data: &[u8], offset: usize) -> Self {
        let mut cursor = Cursor::new(&data[offset..offset + Self::BYTES]);

        Self {
            name: extract_string(&mut cursor, data),
            ty: VarType::try_from(cursor.read_u32::<BigEndian>().unwrap()).unwrap(),
            count: cursor.read_u32::<BigEndian>().unwrap(),
            offset: cursor.read_u32::<BigEndian>().unwrap(),
            block: cursor.read_u32::<BigEndian>().unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitialValue {
    pub value: [f32; 4],
    pub offset: u32,
}

impl Parse for InitialValue {
    const BYTES: usize = 20;

    fn parse(data: &[u8], offset: usize) -> Self {
        let mut cursor = Cursor::new(&data[offset..offset + Self::BYTES]);

        Self {
            value: std::array::from_fn(|_| cursor.read_f32::<BigEndian>().unwrap()),
            offset: cursor.read_u32::<BigEndian>().unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoopVar {
    pub offset: u32,
    pub value: u32,
}

impl Parse for LoopVar {
    const BYTES: usize = 8;

    fn parse(data: &[u8], offset: usize) -> Self {
        let mut cursor = Cursor::new(&data[offset..offset + Self::BYTES]);

        Self {
            offset: cursor.read_u32::<BigEndian>().unwrap(),
            value: cursor.read_u32::<BigEndian>().unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, IntoPrimitive, TryFromPrimitive, Serialize, Deserialize)]
#[repr(u32)]
pub enum SamplerType {
    D1 = 0,
    D2 = 1,
    D3 = 2,
    Cube = 3,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SamplerVar {
    pub name: String,
    pub ty: SamplerType,
    pub location: u32,
}

impl Parse for SamplerVar {
    const BYTES: usize = 12;

    fn parse(data: &[u8], offset: usize) -> Self {
        let mut cursor = Cursor::new(&data[offset..offset + Self::BYTES]);

        Self {
            name: extract_string(&mut cursor, data),
            ty: SamplerType::try_from(cursor.read_u32::<BigEndian>().unwrap()).unwrap(),
            location: cursor.read_u32::<BigEndian>().unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, IntoPrimitive, TryFromPrimitive, Serialize, Deserialize)]
#[repr(u32)]
pub enum VarType {
    Void = 0,
    Bool = 1,
    Int = 2,
    UInt = 3,
    Float = 4,
    Double = 5,
    Double2 = 6,
    Double3 = 7,
    Double4 = 8,
    Float2 = 9,
    Float3 = 10,
    Float4 = 11,
    Bool2 = 12,
    Bool3 = 13,
    Bool4 = 14,
    Int2 = 15,
    Int3 = 16,
    Int4 = 17,
    UInt2 = 18,
    UInt3 = 19,
    UInt4 = 20,
    Float2x2 = 21,
    Float2x3 = 22,
    Float2x4 = 23,
    Float3x2 = 24,
    Float3x3 = 25,
    Float3x4 = 26,
    Float4x2 = 27,
    Float4x3 = 28,
    Float4x4 = 29,
    Double2x2 = 30,
    Double2x3 = 31,
    Double2x4 = 32,
    Double3x2 = 33,
    Double3x3 = 34,
    Double3x4 = 35,
    Double4x2 = 36,
    Double4x3 = 37,
    Double4x4 = 38,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeVar {
    pub name: String,
    pub ty: VarType,
    pub count: u32,
    pub location: u32,
}

impl Parse for AttributeVar {
    const BYTES: usize = 16;

    fn parse(data: &[u8], offset: usize) -> Self {
        let mut cursor = Cursor::new(&data[offset..offset + Self::BYTES]);

        Self {
            name: extract_string(&mut cursor, data),
            ty: VarType::try_from(cursor.read_u32::<BigEndian>().unwrap()).unwrap(),
            count: cursor.read_u32::<BigEndian>().unwrap(),
            location: cursor.read_u32::<BigEndian>().unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Buffer {
    pub flags: u32,
    pub elem_size: u32,
    pub len: u32,
    pub ptr: u32,
}

#[derive(Debug, Clone, PartialEq, IntoPrimitive, TryFromPrimitive, Serialize, Deserialize)]
#[repr(u32)]
pub enum ShaderMode {
    UniformRegister = 0,
    UniformBlock = 1,
    GeometryShader = 2,
    ComputeShader = 3,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gfd {
    pub aligned: bool,
    pub padding: u64,
    pub blocks: Vec<Block>,
}

impl Gfd {
    pub fn parse(bytes: &[u8]) -> Result<Self, ()> {
        let mut header = Cursor::new(&bytes[0..0x20]);

        assert_eq!(header.read_u32::<BigEndian>().unwrap(), 0x47667832); // magic
        assert_eq!(header.read_u32::<BigEndian>().unwrap(), 0x20); // size
        assert_eq!(header.read_u32::<BigEndian>().unwrap(), 7); // major version
        assert_eq!(header.read_u32::<BigEndian>().unwrap(), 1); // minor version
        assert_eq!(header.read_u32::<BigEndian>().unwrap(), 2); // gpu
        let aligned = header.read_u32::<BigEndian>().unwrap() != 0;
        let padding = header.read_u64::<BigEndian>().unwrap();

        let mut blocks = Vec::new();
        let mut i = 0x20;

        while i < bytes.len() {
            let mut header = Cursor::new(&bytes[i..i + 0x20]);

            assert_eq!(header.read_u32::<BigEndian>().unwrap(), 0x424C4B7B); // magic
            assert_eq!(header.read_u32::<BigEndian>().unwrap(), 0x20); // size
            assert_eq!(header.read_u32::<BigEndian>().unwrap(), 1); // major version
            assert_eq!(header.read_u32::<BigEndian>().unwrap(), 0); // minor version
            let ty = BlockType::try_from(header.read_u32::<BigEndian>().unwrap()).unwrap();
            let len = header.read_u32::<BigEndian>().unwrap() as usize;
            assert_eq!(header.read_u64::<BigEndian>().unwrap(), 0); // padding

            let data = &bytes[i + 0x20..i + 0x20 + len];
            match ty {
                BlockType::EndOfFile => break,
                BlockType::VertexProgram => {
                    blocks.push(Block::VertexProgram(data.to_vec()));
                }
                BlockType::FragmentProgram => {
                    blocks.push(Block::FragmentProgram(data.to_vec()));
                }
                BlockType::VertexHeader => {
                    blocks.push(Block::VertexHeader(VertexHeader::parse(data, 0)));
                }
                BlockType::FragmentHeader => {
                    blocks.push(Block::FragmentHeader(FragmentHeader::parse(data, 0)));
                }
                _ => {
                    println!("Unsupported block type: {:?}, length: {}", ty, len);
                }
            }

            i += 0x20 + len;
        }

        Ok(Self {
            aligned,
            padding,
            blocks,
        })
    }
}
