use alloc::{boxed::Box, string::ToString, vec::Vec};
use binrw::{
    BinRead, BinResult, FilePtr, NullString, binread,
    file_ptr::IntoSeekFrom,
    helpers::until,
    io::{Read, Seek, SeekFrom},
};
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct TaggedOffset<const TAG: u16>(pub u32);

impl<const TAG: u16> BinRead for TaggedOffset<TAG> {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        _: Self::Args<'_>,
    ) -> BinResult<Self> {
        let pos = reader.stream_position()?;
        let raw = u32::read_options(reader, endian, ())?;

        // Handle null pointers
        if raw == 0 {
            return Ok(TaggedOffset(0));
        }

        let tag = (raw >> 20) & 0xFFF;
        if tag != u32::from(TAG) {
            return Err(binrw::Error::NoVariantMatch { pos });
        }

        Ok(TaggedOffset(raw & 0xFFFFF))
    }
}

impl<const TAG: u16> IntoSeekFrom for TaggedOffset<TAG> {
    fn into_seek_from(self) -> SeekFrom {
        SeekFrom::Current(self.0 as i64)
    }
}

pub type ObjectOffset = TaggedOffset<0xD06>;
pub type StringOffset = TaggedOffset<0xCA7>;

#[binread]
#[br(big)]
#[derive(Debug)]
pub struct FileHeader {
    #[br(assert(magic == *b"Gfx2"))]
    pub magic: [u8; 4],
    #[br(assert(header_size == size_of::<FileHeader>() as u32))]
    pub header_size: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub gpu_version: u32,
    #[br(assert(alignment_mode == 0 || alignment_mode == 1))]
    pub alignment_mode: u32,
    pub padding: [u8; 8],
}

#[binread]
#[br(big)]
#[derive(Debug)]
pub struct Block {
    #[br(assert(magic == *b"BLK{"))]
    pub magic: [u8; 4],
    pub header_size: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub block_type: BlockType,
    pub data_size: u32,
    pub padding: u64,
    #[br(args(block_type, data_size), pad_size_to = data_size)]
    pub data: BlockData,
}

#[binread]
#[br(big, repr = u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    EndOfFile = 1,
    Padding = 2,
    VertexHeader = 3,
    VertexProgram = 5,
    PixelHeader = 6,
    PixelProgram = 7,
    GeometryHeader = 8,
    GeometryProgram = 9,
    GeometryCopyProgram = 10,
    TextureHeader = 11,
    TextureImageData = 12,
    TextureMipmapData = 13,
    ComputeHeader = 14,
    ComputeProgram = 15,
}

#[binread]
#[br(big, import(ty: BlockType, size: u32))]
#[derive(Debug)]
pub enum BlockData {
    #[br(pre_assert(ty == BlockType::VertexHeader))]
    VertexHeader(VertexHeader),
    #[br(pre_assert(ty == BlockType::VertexProgram))]
    VertexProgram(#[br(count = size)] Vec<u8>),
    //
    #[br(pre_assert(ty == BlockType::PixelHeader))]
    PixelHeader(PixelHeader),
    #[br(pre_assert(ty == BlockType::PixelProgram))]
    PixelProgram(#[br(count = size)] Vec<u8>),
    //
    #[br(pre_assert(ty == BlockType::GeometryHeader))]
    GeometryHeader(GeometryHeader),
    #[br(pre_assert(ty == BlockType::GeometryProgram))]
    GeometryProgram(#[br(count = size)] Vec<u8>),
    #[br(pre_assert(ty == BlockType::GeometryCopyProgram))]
    GeometryCopyProgram(#[br(count = size)] Vec<u8>),
    //
    #[br(pre_assert(ty == BlockType::ComputeHeader))]
    ComputeHeader(ComputeHeader),
    #[br(pre_assert(ty == BlockType::ComputeProgram))]
    ComputeProgram(#[br(count = size)] Vec<u8>),
    //
    #[br(pre_assert(ty == BlockType::TextureHeader))]
    TextureHeader(TextureHeader),
    #[br(pre_assert(ty == BlockType::TextureImageData))]
    TextureImage(#[br(count = size)] Vec<u8>),
    #[br(pre_assert(ty == BlockType::TextureMipmapData))]
    TextureMipmap(#[br(count = size)] Vec<u8>),
    //
    #[br(pre_assert(size == 0 || ty == BlockType::Padding))]
    None,
}

#[binread]
#[br(repr = u32)]
#[derive(Debug, Serialize, Deserialize)]
pub enum ShaderMode {
    UniformRegister = 0,
    UniformBlock = 1,
    GeometryShader = 2,
    ComputeShader = 3,
}

#[binread]
#[br(big, import(base: u64))]
#[derive(Debug)]
pub struct UniformBlock {
    #[br(offset = base)]
    pub name: FilePtr<StringOffset, NullString>,
    pub offset: u32,
    pub size: u32,
}

impl Into<crate::UniformBlock> for UniformBlock {
    fn into(self) -> crate::UniformBlock {
        crate::UniformBlock {
            name: self.name.into_inner().to_string(),
            location: self.offset,
            size: self.size,
        }
    }
}

#[binread]
#[br(repr = u32)]
#[derive(Debug, Serialize, Deserialize)]
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

#[binread]
#[br(big, import(base: u64))]
#[derive(Debug)]
pub struct UniformVar {
    #[br(offset = base)]
    pub name: FilePtr<StringOffset, NullString>,
    pub ty: VarType,
    pub offset: u32,
    pub block: u32,
}

impl Into<crate::UniformVar> for UniformVar {
    fn into(self) -> crate::UniformVar {
        crate::UniformVar {
            name: self.name.into_inner().to_string(),
            ty: self.ty,
            offset: self.offset,
            index: self.block,
        }
    }
}

#[binread]
#[br(big)]
#[derive(Debug, Serialize, Deserialize)]
pub struct InitialValue {
    pub value: [f32; 4],
    pub offset: u32,
}

#[binread]
#[br(big)]
#[derive(Debug, Serialize, Deserialize)]
pub struct LoopVar {
    pub offset: u32,
    pub value: u32,
}

#[binread]
#[br(repr = u32)]
#[derive(Debug, Serialize, Deserialize)]
pub enum SamplerType {
    D1 = 0,
    D2 = 1,
    D3 = 2,
    Cube = 3,
}

#[binread]
#[br(big, import(base: u64))]
#[derive(Debug)]
pub struct SamplerVar {
    #[br(offset = base)]
    pub name: FilePtr<StringOffset, NullString>,
    pub ty: SamplerType,
    pub location: u32,
}

impl Into<crate::SamplerVar> for SamplerVar {
    fn into(self) -> crate::SamplerVar {
        crate::SamplerVar {
            name: self.name.into_inner().to_string(),
            ty: self.ty,
            location: self.location,
        }
    }
}

#[binread]
#[br(big, import(base: u64))]
#[derive(Debug)]
pub struct AttribVar {
    #[br(offset = base)]
    pub name: FilePtr<StringOffset, NullString>,
    pub ty: VarType,
    pub count: u32,
    pub location: u32,
}

impl Into<crate::AttribVar> for AttribVar {
    fn into(self) -> crate::AttribVar {
        crate::AttribVar {
            name: self.name.into_inner().to_string(),
            ty: self.ty,
            count: self.count,
            location: self.location,
        }
    }
}

#[binread]
#[derive(Debug)]
pub struct RBuffer {
    pub flags: u32,
    pub ty_size: u32,
    pub len: u32,
    pub ptr: u32,
}

#[binread]
#[br(big, stream = s)]
#[derive(Debug)]
pub struct VertexHeader {
    #[br(temp, try_calc = s.stream_position())]
    base: u64,

    pub regs: [u32; 52],
    pub program_size: u32,
    pub program_ptr: u32,
    pub mode: ShaderMode,

    pub n_uniform_blocks: u32,
    #[br(offset = base, args { inner: binrw::args! { count: n_uniform_blocks as usize, inner: (base,)  } })]
    pub uniform_blocks: FilePtr<ObjectOffset, Vec<UniformBlock>>,

    pub n_uniform_vars: u32,
    #[br(offset = base, args { inner: binrw::args! { count: n_uniform_vars as usize, inner: (base,)  } })]
    pub uniform_vars: FilePtr<ObjectOffset, Vec<UniformVar>>,

    pub n_initial_values: u32,
    #[br(offset = base, args { inner: binrw::args! { count: n_initial_values as usize } })]
    pub initial_values: FilePtr<ObjectOffset, Vec<InitialValue>>,

    pub n_loop_vars: u32,
    #[br(offset = base, args { inner: binrw::args! { count: n_loop_vars as usize } })]
    pub loop_vars: FilePtr<ObjectOffset, Vec<LoopVar>>,

    pub n_sampler_vars: u32,
    #[br(offset = base, args { inner: binrw::args! { count: n_sampler_vars as usize, inner: (base,)  } })]
    pub sampler_vars: FilePtr<ObjectOffset, Vec<SamplerVar>>,

    pub n_attrib_vars: u32,
    #[br(offset = base, args { inner: binrw::args! { count: n_attrib_vars as usize, inner: (base,)  } })]
    pub attrib_vars: FilePtr<ObjectOffset, Vec<AttribVar>>,

    pub ring_item_size: u32,
    #[br(map = |x: u32| x != 0)]
    pub has_stream_out: bool,
    pub stream_out_stride: [u32; 4],
    pub r_buffer: RBuffer,
}

#[binread]
#[br(big, stream = s)]
#[derive(Debug)]
pub struct PixelHeader {
    #[br(temp, try_calc = s.stream_position())]
    base: u64,

    pub regs: [u32; 41],
    pub program_size: u32,
    pub program_ptr: u32,
    pub mode: ShaderMode,

    pub n_uniform_blocks: u32,
    #[br(offset = base, args { inner: binrw::args! { count: n_uniform_blocks as usize, inner: (base,)  } })]
    pub uniform_blocks: FilePtr<ObjectOffset, Vec<UniformBlock>>,

    pub n_uniform_vars: u32,
    #[br(offset = base, args { inner: binrw::args! { count: n_uniform_vars as usize, inner: (base,)  } })]
    pub uniform_vars: FilePtr<ObjectOffset, Vec<UniformVar>>,

    pub n_initial_values: u32,
    #[br(offset = base, args { inner: binrw::args! { count: n_initial_values as usize } })]
    pub initial_values: FilePtr<ObjectOffset, Vec<InitialValue>>,

    pub n_loop_vars: u32,
    #[br(offset = base, args { inner: binrw::args! { count: n_loop_vars as usize } })]
    pub loop_vars: FilePtr<ObjectOffset, Vec<LoopVar>>,

    pub n_sampler_vars: u32,
    #[br(offset = base, args { inner: binrw::args! { count: n_sampler_vars as usize, inner: (base,)  } })]
    pub sampler_vars: FilePtr<ObjectOffset, Vec<SamplerVar>>,

    pub r_buffer: RBuffer,
}

#[binread]
#[br(big, stream = s)]
#[derive(Debug)]
pub struct GeometryHeader {
    #[br(temp, try_calc = s.stream_position())]
    base: u64,

    pub regs: [u32; 19],
    pub program_size: u32,
    pub program_ptr: u32,
    pub copy_program_size: u32,
    pub copy_program_ptr: u32,
    pub mode: ShaderMode,

    pub n_uniform_blocks: u32,
    #[br(offset = base, args { inner: binrw::args! { count: n_uniform_blocks as usize, inner: (base,)  } })]
    pub uniform_blocks: FilePtr<ObjectOffset, Vec<UniformBlock>>,

    pub n_uniform_vars: u32,
    #[br(offset = base, args { inner: binrw::args! { count: n_uniform_vars as usize, inner: (base,)  } })]
    pub uniform_vars: FilePtr<ObjectOffset, Vec<UniformVar>>,

    pub n_initial_values: u32,
    #[br(offset = base, args { inner: binrw::args! { count: n_initial_values as usize } })]
    pub initial_values: FilePtr<ObjectOffset, Vec<InitialValue>>,

    pub n_loop_vars: u32,
    #[br(offset = base, args { inner: binrw::args! { count: n_loop_vars as usize } })]
    pub loop_vars: FilePtr<ObjectOffset, Vec<LoopVar>>,

    pub n_sampler_vars: u32,
    #[br(offset = base, args { inner: binrw::args! { count: n_sampler_vars as usize, inner: (base,)  } })]
    pub sampler_vars: FilePtr<ObjectOffset, Vec<SamplerVar>>,

    pub ring_item_size: u32,
    #[br(map = |x: u32| x != 0)]
    pub has_stream_out: bool,
    pub stream_out_stride: [u32; 4],
    pub r_buffer: RBuffer,
}

#[binread]
#[br(big, stream = s)]
#[derive(Debug)]
pub struct ComputeHeader {
    #[br(temp, try_calc = s.stream_position())]
    base: u64,

    pub regs: [u32; 12],
    pub program_size: u32,
    pub program_ptr: u32,
    pub mode: ShaderMode,

    pub n_uniform_blocks: u32,
    #[br(offset = base, args { inner: binrw::args! { count: n_uniform_blocks as usize, inner: (base,)  } })]
    pub uniform_blocks: FilePtr<ObjectOffset, Vec<UniformBlock>>,

    pub n_uniform_vars: u32,
    #[br(offset = base, args { inner: binrw::args! { count: n_uniform_vars as usize, inner: (base,)  } })]
    pub uniform_vars: FilePtr<ObjectOffset, Vec<UniformVar>>,

    pub n_initial_values: u32,
    #[br(offset = base, args { inner: binrw::args! { count: n_initial_values as usize } })]
    pub initial_values: FilePtr<ObjectOffset, Vec<InitialValue>>,

    pub n_loop_vars: u32,
    #[br(offset = base, args { inner: binrw::args! { count: n_loop_vars as usize } })]
    pub loop_vars: FilePtr<ObjectOffset, Vec<LoopVar>>,

    pub n_sampler_vars: u32,
    #[br(offset = base, args { inner: binrw::args! { count: n_sampler_vars as usize, inner: (base,)  } })]
    pub sampler_vars: FilePtr<ObjectOffset, Vec<SamplerVar>>,

    pub work_group_size_x: u32,
    pub work_group_size_y: u32,
    pub work_group_size_z: u32,
    #[br(map = |x: u32| x != 0)]
    pub over_64_mode: bool,
    pub waves_per_simd: u32,
    pub r_buffer: RBuffer,
}

#[binread]
#[br(big)]
#[derive(Debug)]
pub struct TextureHeader {
    pub surface: Surface,
    pub view_first_mip: u32,
    pub view_num_mips: u32,
    pub view_first_slice: u32,
    pub view_num_slices: u32,
    pub comp_map: u32,
    pub regs: [u32; 5],
}

#[binread]
#[br(big)]
#[derive(Debug)]
pub struct Surface {
    pub dimension: Dimension,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub mip_levels: u32,
    pub format: Format,
    pub aa: AntiAlias,
    pub usage: Usage,
    pub image_size: u32,
    pub image_ptr: u32,
    pub mipmap_size: u32,
    pub mipmap_ptr: u32,
    pub tile_mode: TileMode,
    pub swizzle: u32,
    pub alignment: u32,
    pub pitch: u32,
    pub mip_level_offsets: [u32; 13],
}

#[binread]
#[br(big, repr = u32)]
#[derive(Debug, Serialize, Deserialize)]
pub enum Dimension {
    D1 = 0,
    D2 = 1,
    D3 = 2,
    Cube = 3,
    D1Array = 4,
    D2Array = 5,
    D2Msaa = 6,
    D2MsaaArray = 7,
}

#[binread]
#[br(big, repr = u32)]
#[derive(Debug, Serialize, Deserialize)]
pub enum Format {
    Invalid = 0x0,
    UnormR8 = 0x1,
    UnormR4G4 = 0x2,
    UnormR16 = 0x5,
    UnormR8G8 = 0x7,
    UnormR5G6B5 = 0x8,
    UnormR5G5B5A1 = 0xA,
    UnormR4G4B4A4 = 0xB,
    UnormA1B5G5R5 = 0xC,
    UnormR16G16 = 0xF,
    UnormR24X8 = 0x11,
    UnormR10G10B10A2 = 0x19,
    UnormR8G8B8A8 = 0x1A,
    UnormA2B10G10R10 = 0x1B,
    UnormR16G16B16A16 = 0x1F,
    UnormBc1 = 0x31,
    UnormBc2 = 0x32,
    UnormBc3 = 0x33,
    UnormBc4 = 0x34,
    UnormBc5 = 0x35,
    UnormNv12 = 0x81,
    UintR8 = 0x101,
    UintR16 = 0x105,
    UintR8G8 = 0x107,
    UintR32 = 0x10D,
    UintR16G16 = 0x10F,
    UintX24G8 = 0x111,
    UintR10G10B10A2 = 0x119,
    UintR8G8B8A8 = 0x11A,
    UintA2B10G10R10 = 0x11B,
    UintG8X24 = 0x11C,
    UintR32G32 = 0x11D,
    UintR16G16B16A16 = 0x11F,
    UintR32G32B32A32 = 0x122,
    SnormR8 = 0x201,
    SnormR16 = 0x205,
    SnormR8G8 = 0x207,
    SnormR16G16 = 0x20F,
    SnormR10G10B10A2 = 0x219,
    SnormR8G8B8A8 = 0x21A,
    SnormR16G16B16A16 = 0x21F,
    SnormBc4 = 0x234,
    SnormBc5 = 0x235,
    SintR8 = 0x301,
    SintR16 = 0x305,
    SintR8G8 = 0x307,
    SintR32 = 0x30D,
    SintR16G16 = 0x30F,
    SintR10G10B10A2 = 0x319,
    SintR8G8B8A8 = 0x31A,
    SintR32G32 = 0x31D,
    SintR16G16B16A16 = 0x31F,
    SintR32G32B32A32 = 0x322,
    SrgbR8G8B8A8 = 0x41A,
    SrgbBc1 = 0x431,
    SrgbBc2 = 0x432,
    SrgbBc3 = 0x433,
    FloatR16 = 0x806,
    FloatR32 = 0x80E,
    FloatR16G16 = 0x810,
    FloatD24S8 = 0x811,
    FloatR11G11B10 = 0x816,
    FloatX8X24 = 0x81C,
    FloatR32G32 = 0x81E,
    FloatR16G16B16A16 = 0x820,
    FloatR32G32B32A32 = 0x823,
}

#[binread]
#[br(big, repr = u32)]
#[derive(Debug, Serialize, Deserialize)]
pub enum AntiAlias {
    Mode1x = 0,
    Mode2x = 1,
    Mode4x = 2,
    Mode8x = 3,
}

#[binread]
#[br(big, repr = u32)]
#[derive(Debug, Default, Serialize, Deserialize)]
pub enum TileMode {
    #[default]
    Default = 0,
    LinearAligned = 1,
    Tiled1dThin1 = 2,
    Tiled1dThick = 3,
    Tiled2dThin1 = 4,
    Tiled2dThin2 = 5,
    Tiled2dThin4 = 6,
    Tiled2dThick = 7,
    Tiled2bThin1 = 8,
    Tiled2bThin2 = 9,
    Tiled2bThin4 = 10,
    Tiled2bThick = 11,
    Tiled3dThin1 = 12,
    Tiled3dThick = 13,
    Tiled3bThin1 = 14,
    Tiled3bThick = 15,
    LinearSpecial = 16,
}

bitflags! {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Usage: u32 {
        const NONE = 0x0;
        const TEXTURE = 0x1;
        const COLOR_BUFFER = 0x2;
        const DEPTH_BUFFER = 0x4;
        const SCAN_BUFFER = 0x8;
        const TV = 0x8000_0000;
    }
}

impl BinRead for Usage {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let val = u32::read_options(reader, endian, args)?;
        Ok(Self::from_bits_retain(val))
    }
}

#[binread]
#[br(big)]
#[derive(Debug)]
pub struct Gfx2 {
    pub header: FileHeader,
    #[br(parse_with = until(|b: &Block| b.block_type == BlockType::EndOfFile))]
    pub blocks: Vec<Block>,
}
