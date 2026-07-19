//! A library for loading and storing Gfx2 shaders. Gfx2 is the common data format used by GX2, the graphics library of the Nintendo Wii U. The offical SDK includes a libary called `gfd` for loading shaders and texture from `gsh` / `gtx` files. This crate is the open Rust alternative for `gfd`.

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VertexShader {
    #[serde(with = "BigArray")]
    pub regs: [u32; 52],
    pub program: Vec<u8>,
    pub mode: ShaderMode,
    pub uniform_blocks: Vec<UniformBlock>,
    pub uniform_vars: Vec<UniformVar>,
    pub initial_values: Vec<InitialValue>,
    pub loop_vars: Vec<LoopVar>,
    pub sampler_vars: Vec<SamplerVar>,
    pub attrib_vars: Vec<AttribVar>,
    pub ring_item_size: u32,
    pub has_stream_out: bool,
    pub stream_out_stride: [u32; 4],
}

impl VertexShader {
    pub const SQ_PGM_RESOURCES_VS: usize = 0;
    pub const VGT_PRIMITIVEID_EN: usize = 1;
    pub const SPI_VS_OUT_CONFIG: usize = 2;
    pub const NUM_SPI_VS_OUT_ID: usize = 3;

    pub const SPI_VS_OUT_ID_0: usize = 4;
    pub const SPI_VS_OUT_ID_1: usize = 5;
    pub const SPI_VS_OUT_ID_2: usize = 6;
    pub const SPI_VS_OUT_ID_3: usize = 7;
    pub const SPI_VS_OUT_ID_4: usize = 8;
    pub const SPI_VS_OUT_ID_5: usize = 9;
    pub const SPI_VS_OUT_ID_6: usize = 10;
    pub const SPI_VS_OUT_ID_7: usize = 11;
    pub const SPI_VS_OUT_ID_8: usize = 12;
    pub const SPI_VS_OUT_ID_9: usize = 13;

    pub const PA_CL_VS_OUT_CNTL: usize = 14;
    pub const SQ_VTX_SEMANTIC_CLEAR: usize = 15;
    pub const NUM_SQ_VTX_SEMANTIC: usize = 16;

    pub const SQ_VTX_SEMANTIC_0: usize = 17;
    pub const SQ_VTX_SEMANTIC_1: usize = 18;
    pub const SQ_VTX_SEMANTIC_2: usize = 19;
    pub const SQ_VTX_SEMANTIC_3: usize = 20;
    pub const SQ_VTX_SEMANTIC_4: usize = 21;
    pub const SQ_VTX_SEMANTIC_5: usize = 22;
    pub const SQ_VTX_SEMANTIC_6: usize = 23;
    pub const SQ_VTX_SEMANTIC_7: usize = 24;
    pub const SQ_VTX_SEMANTIC_8: usize = 25;
    pub const SQ_VTX_SEMANTIC_9: usize = 26;
    pub const SQ_VTX_SEMANTIC_10: usize = 27;
    pub const SQ_VTX_SEMANTIC_11: usize = 28;
    pub const SQ_VTX_SEMANTIC_12: usize = 29;
    pub const SQ_VTX_SEMANTIC_13: usize = 30;
    pub const SQ_VTX_SEMANTIC_14: usize = 31;
    pub const SQ_VTX_SEMANTIC_15: usize = 32;
    pub const SQ_VTX_SEMANTIC_16: usize = 33;
    pub const SQ_VTX_SEMANTIC_17: usize = 34;
    pub const SQ_VTX_SEMANTIC_18: usize = 35;
    pub const SQ_VTX_SEMANTIC_19: usize = 36;
    pub const SQ_VTX_SEMANTIC_20: usize = 37;
    pub const SQ_VTX_SEMANTIC_21: usize = 38;
    pub const SQ_VTX_SEMANTIC_22: usize = 39;
    pub const SQ_VTX_SEMANTIC_23: usize = 40;
    pub const SQ_VTX_SEMANTIC_24: usize = 41;
    pub const SQ_VTX_SEMANTIC_25: usize = 42;
    pub const SQ_VTX_SEMANTIC_26: usize = 43;
    pub const SQ_VTX_SEMANTIC_27: usize = 44;
    pub const SQ_VTX_SEMANTIC_28: usize = 45;
    pub const SQ_VTX_SEMANTIC_29: usize = 46;
    pub const SQ_VTX_SEMANTIC_30: usize = 47;
    pub const SQ_VTX_SEMANTIC_31: usize = 48;

    pub const VGT_STRMOUT_BUFFER_EN: usize = 49;
    pub const VGT_VERTEX_REUSE_BLOCK_CNTL: usize = 50;
    pub const VGT_HOS_REUSE_DEPTH: usize = 51;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PixelShader {
    #[serde(with = "BigArray")]
    pub regs: [u32; 41],
    pub program: Vec<u8>,
    pub mode: ShaderMode,
    pub uniform_blocks: Vec<UniformBlock>,
    pub uniform_vars: Vec<UniformVar>,
    pub initial_values: Vec<InitialValue>,
    pub loop_vars: Vec<LoopVar>,
    pub sampler_vars: Vec<SamplerVar>,
}

impl PixelShader {
    pub const SQ_PGM_RESOURCES_PS: usize = 0;
    pub const SQ_PGM_EXPORTS_PS: usize = 1;
    pub const SPI_PS_IN_CONTROL_0: usize = 2;
    pub const SPI_PS_IN_CONTROL_1: usize = 3;
    pub const NUM_SPI_PS_INPUT_CNTL: usize = 4;

    pub const SPI_PS_INPUT_CNTL_0: usize = 5;
    pub const SPI_PS_INPUT_CNTL_1: usize = 6;
    pub const SPI_PS_INPUT_CNTL_2: usize = 7;
    pub const SPI_PS_INPUT_CNTL_3: usize = 8;
    pub const SPI_PS_INPUT_CNTL_4: usize = 9;
    pub const SPI_PS_INPUT_CNTL_5: usize = 10;
    pub const SPI_PS_INPUT_CNTL_6: usize = 11;
    pub const SPI_PS_INPUT_CNTL_7: usize = 12;
    pub const SPI_PS_INPUT_CNTL_8: usize = 13;
    pub const SPI_PS_INPUT_CNTL_9: usize = 14;
    pub const SPI_PS_INPUT_CNTL_10: usize = 15;
    pub const SPI_PS_INPUT_CNTL_11: usize = 16;
    pub const SPI_PS_INPUT_CNTL_12: usize = 17;
    pub const SPI_PS_INPUT_CNTL_13: usize = 18;
    pub const SPI_PS_INPUT_CNTL_14: usize = 19;
    pub const SPI_PS_INPUT_CNTL_15: usize = 20;
    pub const SPI_PS_INPUT_CNTL_16: usize = 21;
    pub const SPI_PS_INPUT_CNTL_17: usize = 22;
    pub const SPI_PS_INPUT_CNTL_18: usize = 23;
    pub const SPI_PS_INPUT_CNTL_19: usize = 24;
    pub const SPI_PS_INPUT_CNTL_20: usize = 25;
    pub const SPI_PS_INPUT_CNTL_21: usize = 26;
    pub const SPI_PS_INPUT_CNTL_22: usize = 27;
    pub const SPI_PS_INPUT_CNTL_23: usize = 28;
    pub const SPI_PS_INPUT_CNTL_24: usize = 29;
    pub const SPI_PS_INPUT_CNTL_25: usize = 30;
    pub const SPI_PS_INPUT_CNTL_26: usize = 31;
    pub const SPI_PS_INPUT_CNTL_27: usize = 32;
    pub const SPI_PS_INPUT_CNTL_28: usize = 33;
    pub const SPI_PS_INPUT_CNTL_29: usize = 34;
    pub const SPI_PS_INPUT_CNTL_30: usize = 35;
    pub const SPI_PS_INPUT_CNTL_31: usize = 36;

    pub const CB_SHADER_MASK: usize = 37;
    pub const CB_SHADER_CONTROL: usize = 38;
    pub const DB_SHADER_CONTROL: usize = 39;
    pub const SPI_INPUT_Z: usize = 40;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeometryShader {
    pub regs: [u32; 19],
    pub program: Vec<u8>,
    pub copy_program: Vec<u8>,
    pub mode: ShaderMode,
    pub uniform_blocks: Vec<UniformBlock>,
    pub uniform_vars: Vec<UniformVar>,
    pub initial_values: Vec<InitialValue>,
    pub loop_vars: Vec<LoopVar>,
    pub sampler_vars: Vec<SamplerVar>,
    pub ring_item_size: u32,
    pub has_stream_out: bool,
    pub stream_out_stride: [u32; 4],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComputeShader {
    pub regs: [u32; 12],
    pub program: Vec<u8>,
    pub mode: ShaderMode,
    pub uniform_blocks: Vec<UniformBlock>,
    pub uniform_vars: Vec<UniformVar>,
    pub initial_values: Vec<InitialValue>,
    pub loop_vars: Vec<LoopVar>,
    pub sampler_vars: Vec<SamplerVar>,
    pub work_group_size: (u32, u32, u32),
    pub over_64_mode: bool,
    pub waves_per_simd: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Texture {
    pub surface: Surface,
    pub view_first_mip: u32,
    pub view_num_mips: u32,
    pub view_first_slice: u32,
    pub view_num_slices: u32,
    pub comp_map: u32,
    pub regs: [u32; 5],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UniformBlock {
    pub name: String,
    pub location: u32,
    pub size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UniformVar {
    pub name: String,
    pub ty: VarType,
    pub count: u32,
    pub offset: u32,
    pub index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SamplerVar {
    pub name: String,
    pub ty: SamplerType,
    pub location: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttribVar {
    pub name: String,
    pub ty: VarType,
    pub count: u32,
    pub location: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

/// Compiled GX2 shader data
///
/// [Gfx2] implements [serde::Serialize] / [serde::Deserialize] so the structure can be saved and loaded with any compatible serde data format.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Gfx2 {
    pub magic: [u8; 4],
    pub version: (u8, u8),
    pub gpu: u8,
    pub vertex: Vec<VertexShader>,
    pub pixel: Vec<PixelShader>,
    pub geometry: Vec<GeometryShader>,
    pub compute: Vec<ComputeShader>,
    pub texture: Vec<Texture>,
}

impl Default for Gfx2 {
    fn default() -> Self {
        Self {
            magic: *b"Gfx2",
            version: (7, 1),
            gpu: 2,
            vertex: Vec::new(),
            pixel: Vec::new(),
            geometry: Vec::new(),
            compute: Vec::new(),
            texture: Vec::new(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to parse GSH data")]
    Gsh(binrw::Error),
    #[error("Unexpected block type '{0:?}' at index {1}")]
    UnexpectedBlockType(parser::BlockType, usize),
    #[cfg(feature = "postcard")]
    #[error("Failed to handle postcard data")]
    Postcard(#[from] postcard::Error),
}

// #[from] binrw::Error doesnt work
impl From<binrw::Error> for Error {
    fn from(value: binrw::Error) -> Self {
        Self::Gsh(value)
    }
}

impl Gfx2 {
    /// Deserialize a gsh encoded byte slice.
    ///
    /// # Example
    ///
    /// ```
    /// use gfx2::Gfx2;
    ///
    /// let data = include_bytes!("../tests/program.gsh");
    /// let gfx2 = Gfx2::from_gsh(data).unwrap();
    ///
    /// assert_eq!(gfx2.vertex.len(), 1);
    /// assert_eq!(gfx2.pixel.len(), 1);
    /// ```
    pub fn from_gsh(data: impl AsRef<[u8]>) -> Result<Self, Error> {
        use binrw::io::Cursor;
        let gfx2 = parser::Gfx2::read(&mut Cursor::new(&data))?;
        Self::try_from(gfx2)
    }

    #[cfg(feature = "postcard")]
    /// Convenience function to deserialize a postcard encoded byte slice. For more information see [postcard](https://crates.io/crates/postcard).
    pub fn from_bytes(data: impl AsRef<[u8]>) -> Result<Self, Error> {
        let gfx2 = postcard::from_bytes(data.as_ref())?;
        Ok(gfx2)
    }

    #[cfg(feature = "postcard")]
    /// Convenience function to serialize postcard encoded byte data. For more information see [postcard](https://crates.io/crates/postcard).
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        Ok(postcard::to_allocvec(self)?)
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
    // TODO: more tests
}
