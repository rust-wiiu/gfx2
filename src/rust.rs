use alloc::{ffi::CString, vec::Vec};

struct Gfx2 {
    magic: [u8; 4],
    version: (u8, u8),
    gpu: u8,
    align: u32,
    blocks: Vec<Block>,
}

enum ShaderMode {
    UniformRegister,
    UniformBlock,
    GeometryShader,
    ComputeShader,
}

enum Block {
    VertexShader {
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
        has_stream_out: u32, // bool?
        stream_out_stride: [u32; 4],
    },
    PixelShader {
        regs: [u32; 41],
        program: Vec<u8>,
        mode: ShaderMode,
        uniform_blocks: Vec<UniformBlock>,
        uniform_vars: Vec<UniformVar>,
        initial_values: Vec<InitialValue>,
        loop_vars: Vec<LoopVar>,
        sampler_vars: Vec<SamplerVar>,
    },
}

struct UniformBlock {
    name: CString,
    offset: u32,
    size: u32,
}

struct UniformVar {
    name: CString,
    ty: (),
    count: u32,
    offset: u32,
    block: u32,
}

struct InitialValue {
    value: [f32; 4],
    offset: u32,
}

struct LoopVar {
    offset: u32,
    value: u32,
}

struct SamplerVar {
    name: CString,
    ty: (),
    location: u32,
}

struct AttribVar {
    name: CString,
    ty: (),
    count: u32,
    location: u32,
}
