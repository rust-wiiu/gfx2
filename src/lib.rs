#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod parser;

use alloc::{string::String, vec::Vec};
use binrw::BinRead;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

pub use parser::{
    AntiAlias, Dimension, Format, InitialValue, LoopVar, SamplerType, ShaderMode, TileMode, Usage,
    VarType,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct VertexShader {
    #[serde(with = "BigArray")]
    regs: [u32; 52],
    program: Vec<u8>,
    mode: ShaderMode,
    uniform_blocks: Vec<UniformBlock>,
    uniform_vars: Vec<UniformVar>,
    initial_values: Vec<InitialValue>,
    loop_vars: Vec<LoopVar>,
    sampler_vars: Vec<SamplerVar>,
    attrib_vars: Vec<AttribVar>,
    ring_item_size: u32,
    has_stream_out: bool,
    stream_out_stride: [u32; 4],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PixelShader {
    #[serde(with = "BigArray")]
    regs: [u32; 41],
    program: Vec<u8>,
    mode: ShaderMode,
    uniform_blocks: Vec<UniformBlock>,
    uniform_vars: Vec<UniformVar>,
    initial_values: Vec<InitialValue>,
    loop_vars: Vec<LoopVar>,
    sampler_vars: Vec<SamplerVar>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryShader {
    regs: [u32; 19],
    program: Vec<u8>,
    copy_program: Vec<u8>,
    mode: ShaderMode,
    uniform_blocks: Vec<UniformBlock>,
    uniform_vars: Vec<UniformVar>,
    initial_values: Vec<InitialValue>,
    loop_vars: Vec<LoopVar>,
    sampler_vars: Vec<SamplerVar>,
    ring_item_size: u32,
    has_stream_out: bool,
    stream_out_stride: [u32; 4],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComputeShader {
    regs: [u32; 12],
    program: Vec<u8>,
    mode: ShaderMode,
    uniform_blocks: Vec<UniformBlock>,
    uniform_vars: Vec<UniformVar>,
    initial_values: Vec<InitialValue>,
    loop_vars: Vec<LoopVar>,
    sampler_vars: Vec<SamplerVar>,
    work_group_size: (u32, u32, u32),
    over_64_mode: bool,
    waves_per_simd: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Texture {
    surface: Surface,
    view_first_mip: u32,
    view_num_mips: u32,
    view_first_slice: u32,
    view_num_slices: u32,
    comp_map: u32,
    regs: [u32; 5],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UniformBlock {
    pub name: String,
    pub location: u32,
    pub size: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UniformVar {
    pub name: String,
    pub ty: VarType,
    pub count: u32,
    pub offset: u32,
    pub index: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamplerVar {
    pub name: String,
    pub ty: SamplerType,
    pub location: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttribVar {
    pub name: String,
    pub ty: VarType,
    pub count: u32,
    pub location: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Surface {
    pub dimension: Dimension,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub mip_levels: u32,
    pub format: Format,
    pub aa: AntiAlias,
    pub usage: Usage,
    pub image: Vec<u8>,
    pub mipmap: Vec<u8>,
    pub tile_mode: TileMode,
    pub swizzle: u32,
    pub alignment: u32,
    pub pitch: u32,
    pub mip_level_offsets: [u32; 13],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gfx2 {
    pub magic: [u8; 4],
    pub version: (u8, u8),
    pub gpu: u8,
    // pub align: u32,
    // pub blocks: Vec<Block>,
    pub vertex: Vec<VertexShader>,
    pub pixel: Vec<PixelShader>,
    pub geometry: Vec<GeometryShader>,
    pub compute: Vec<ComputeShader>,
    pub texture: Vec<Texture>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to parse data")]
    Parsing(binrw::Error),
    #[error("Unexpected block type '{0:?}' at index {1}")]
    UnexpectedBlockType(parser::BlockType, usize),
}

impl From<binrw::Error> for Error {
    fn from(value: binrw::Error) -> Self {
        Self::Parsing(value)
    }
}

impl Gfx2 {
    pub fn parse(data: impl AsRef<[u8]>) -> Result<Self, Error> {
        use binrw::io::Cursor;

        let gfx2 = parser::Gfx2::read(&mut Cursor::new(&data))?;

        Self::try_from(gfx2)
    }
}

impl TryFrom<parser::Gfx2> for Gfx2 {
    type Error = Error;

    fn try_from(value: parser::Gfx2) -> Result<Self, Self::Error> {
        let mut s = Self {
            magic: value.header.magic,
            version: (
                value.header.major_version as u8,
                value.header.minor_version as u8,
            ),
            gpu: value.header.gpu_version as u8,
            vertex: Vec::new(),
            pixel: Vec::new(),
            geometry: Vec::new(),
            compute: Vec::new(),
            texture: Vec::new(),
        };

        let mut blocks = value.blocks.into_iter().enumerate();

        while let Some((i, block)) = blocks.next() {
            use parser::{BlockData as Data, BlockType as Type};

            macro_rules! next_block {
                ($blocks:expr, $variant:path, $index:expr) => {{
                    let (next_i, block) = $blocks
                        .next()
                        .ok_or(Error::UnexpectedBlockType(Type::EndOfFile, $index))?;

                    let $variant(data) = block.data else {
                        return Err(Error::UnexpectedBlockType(block.block_type, next_i));
                    };

                    data
                }};
            }

            match (block.block_type, block.data) {
                (Type::VertexHeader, Data::VertexHeader(header)) => {
                    let program = next_block!(blocks, Data::VertexProgram, i + 1);

                    s.vertex.push(VertexShader {
                        regs: header.regs,
                        program,
                        mode: header.mode,
                        uniform_blocks: header
                            .uniform_blocks
                            .into_inner()
                            .into_iter()
                            .map(Into::into)
                            .collect(),
                        uniform_vars: header
                            .uniform_vars
                            .into_inner()
                            .into_iter()
                            .map(Into::into)
                            .collect(),
                        initial_values: header.initial_values.into_inner(),
                        loop_vars: header.loop_vars.into_inner(),
                        sampler_vars: header
                            .sampler_vars
                            .into_inner()
                            .into_iter()
                            .map(Into::into)
                            .collect(),
                        attrib_vars: header
                            .attrib_vars
                            .into_inner()
                            .into_iter()
                            .map(Into::into)
                            .collect(),
                        ring_item_size: header.ring_item_size,
                        has_stream_out: header.has_stream_out,
                        stream_out_stride: header.stream_out_stride,
                    });
                }
                (Type::PixelHeader, Data::PixelHeader(header)) => {
                    let program = next_block!(blocks, Data::PixelProgram, i + 1);

                    s.pixel.push(PixelShader {
                        regs: header.regs,
                        program,
                        mode: header.mode,
                        uniform_blocks: header
                            .uniform_blocks
                            .into_inner()
                            .into_iter()
                            .map(Into::into)
                            .collect(),
                        uniform_vars: header
                            .uniform_vars
                            .into_inner()
                            .into_iter()
                            .map(Into::into)
                            .collect(),
                        initial_values: header.initial_values.into_inner(),
                        loop_vars: header.loop_vars.into_inner(),
                        sampler_vars: header
                            .sampler_vars
                            .into_inner()
                            .into_iter()
                            .map(Into::into)
                            .collect(),
                    });
                }
                (Type::GeometryHeader, Data::GeometryHeader(header)) => {
                    let program = next_block!(blocks, Data::PixelProgram, i + 1);
                    let copy_program = next_block!(blocks, Data::GeometryCopyProgram, i + 2);

                    s.geometry.push(GeometryShader {
                        regs: header.regs,
                        program,
                        copy_program,
                        mode: header.mode,
                        uniform_blocks: header
                            .uniform_blocks
                            .into_inner()
                            .into_iter()
                            .map(Into::into)
                            .collect(),
                        uniform_vars: header
                            .uniform_vars
                            .into_inner()
                            .into_iter()
                            .map(Into::into)
                            .collect(),
                        initial_values: header.initial_values.into_inner(),
                        loop_vars: header.loop_vars.into_inner(),
                        sampler_vars: header
                            .sampler_vars
                            .into_inner()
                            .into_iter()
                            .map(Into::into)
                            .collect(),
                        ring_item_size: header.ring_item_size,
                        has_stream_out: header.has_stream_out,
                        stream_out_stride: header.stream_out_stride,
                    });
                }
                (Type::ComputeHeader, Data::ComputeHeader(header)) => {
                    let program = next_block!(blocks, Data::ComputeProgram, i + 1);

                    s.compute.push(ComputeShader {
                        regs: header.regs,
                        program,
                        mode: header.mode,
                        uniform_blocks: header
                            .uniform_blocks
                            .into_inner()
                            .into_iter()
                            .map(Into::into)
                            .collect(),
                        uniform_vars: header
                            .uniform_vars
                            .into_inner()
                            .into_iter()
                            .map(Into::into)
                            .collect(),
                        initial_values: header.initial_values.into_inner(),
                        loop_vars: header.loop_vars.into_inner(),
                        sampler_vars: header
                            .sampler_vars
                            .into_inner()
                            .into_iter()
                            .map(Into::into)
                            .collect(),
                        work_group_size: (
                            header.work_group_size_x,
                            header.work_group_size_y,
                            header.work_group_size_z,
                        ),
                        over_64_mode: header.over_64_mode,
                        waves_per_simd: header.waves_per_simd,
                    });
                }
                (Type::TextureHeader, Data::TextureHeader(header)) => {
                    let image = next_block!(blocks, Data::ComputeProgram, i + 1);
                    let mipmap = next_block!(blocks, Data::ComputeProgram, i + 2);

                    s.texture.push(Texture {
                        surface: Surface {
                            dimension: header.surface.dimension,
                            width: header.surface.width,
                            height: header.surface.height,
                            depth: header.surface.depth,
                            mip_levels: header.surface.mip_levels,
                            format: header.surface.format,
                            aa: header.surface.aa,
                            usage: header.surface.usage,
                            image,
                            mipmap,
                            tile_mode: header.surface.tile_mode,
                            swizzle: header.surface.swizzle,
                            alignment: header.surface.alignment,
                            pitch: header.surface.pitch,
                            mip_level_offsets: header.surface.mip_level_offsets,
                        },
                        view_first_mip: header.view_first_mip,
                        view_num_mips: header.view_num_mips,
                        view_first_slice: header.view_first_slice,
                        view_num_slices: header.view_num_slices,
                        comp_map: header.comp_map,
                        regs: header.regs,
                    });
                }
                (parser::BlockType::EndOfFile, _) => break,
                (ty, _) => return Err(Error::UnexpectedBlockType(ty, i)),
            }
        }

        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gfx2_parse() {
        let gsh = std::fs::read("tests/program.gsh").unwrap();
        let gfx2 = Gfx2::parse(&gsh).unwrap();

        assert_eq!(gfx2.version, (7, 1));
        assert_eq!(gfx2.gpu, 2);
        assert_eq!(gfx2.vertex.len(), 1);
        assert_eq!(gfx2.pixel.len(), 1);
    }
}
