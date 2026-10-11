//! The Vulkan binding convention, in one place.
//!
//! Every number the renderer needs to build descriptor set layouts,
//! push-constant ranges and vertex input descriptions for a translated
//! shader is a constant or a table in this module. The translator reads
//! them from here and nowhere else. The choices are this project's own
//! (Inferred: none of them is dictated by Direct3D 9 or Vulkan).
//!
//! # Descriptor sets
//!
//! | Set | Binding | Contents |
//! |---|---|---|
//! | 0 | 0 | vertex float constants: uniform block `{ vec4 c[256]; }` |
//! | 0 | 1 | pixel float constants: uniform block `{ vec4 c[224]; }` |
//! | 0 | 2 | vertex integer/boolean constants: `{ ivec4 i[16]; uint b; }` |
//! | 0 | 3 | pixel integer/boolean constants: same layout |
//! | 1 | 0-15 | pixel samplers `s0`-`s15`, combined image samplers |
//! | 1 | 16-19 | vertex samplers `s0`-`s3`, combined image samplers |
//!
//! Integer and boolean constants live in a uniform buffer rather than in
//! specialization constants because the game flips boolean constants
//! between draws; a specialization constant would need a pipeline per
//! value. Boolean `b#` is bit `#` of the `b` member. A `def`, `defi` or
//! `defb` in the shader overrides the buffer value for that register.
//!
//! # Push constants
//!
//! A vertex shader that writes a position reads one `vec4` push constant
//! at offset 0 (see [`POSITION_FIXUP_OFFSET`]) when
//! [`crate::Options::position_fixup`] is on:
//! `position.xy = position.xy * fixup.xy + fixup.zw * position.w`.
//! For Direct3D 9 behaviour on a viewport of `W` by `H` pixels with a
//! positive-height Vulkan viewport the renderer passes
//! `(1, -1, 1/W, 1/H)`: the `-1` flips Direct3D's y-up clip space to
//! Vulkan's y-down, and the offsets move the geometry by half a pixel
//! right and down so that Direct3D 9's integer pixel centres land on
//! Vulkan's half-integer ones. A renderer that does the flip with a
//! negative-height viewport and the half-pixel shift in the viewport
//! origin passes `(1, 1, 0, 0)` or turns the option off. Depth needs no
//! fixup: both APIs clip z to `[0, w]`.
//!
//! # Interface locations
//!
//! Vertex inputs and the varyings between the stages are matched by
//! Direct3D usage and usage index through two fixed tables
//! ([`VERTEX_INPUT_LOCATIONS`], [`VARYING_LOCATIONS`]), so a vertex and a
//! pixel shader translated separately always agree, and the renderer can
//! build `VkVertexInputAttributeDescription`s from the vertex declaration
//! alone. A usage missing from a table is rejected with
//! [`crate::Error::Unsupported`].

/// Descriptor set holding the constant buffers.
pub const CONSTANT_SET: u32 = 0;
/// Binding of the vertex float constant buffer.
pub const VS_FLOAT_BINDING: u32 = 0;
/// Binding of the pixel float constant buffer.
pub const PS_FLOAT_BINDING: u32 = 1;
/// Binding of the vertex integer/boolean constant buffer.
pub const VS_INT_BOOL_BINDING: u32 = 2;
/// Binding of the pixel integer/boolean constant buffer.
pub const PS_INT_BOOL_BINDING: u32 = 3;

/// Float constant registers in a vertex shader (`vs_3_0`: 256).
pub const VS_FLOAT_CONSTANTS: u32 = 256;
/// Float constant registers in a pixel shader (`ps_3_0`: 224).
pub const PS_FLOAT_CONSTANTS: u32 = 224;
/// Integer constant registers per stage.
pub const INT_CONSTANTS: u32 = 16;
/// Boolean constant registers per stage.
pub const BOOL_CONSTANTS: u32 = 16;
/// Byte stride of one constant register in the buffers.
pub const CONSTANT_STRIDE: u32 = 16;
/// Byte offset of the boolean bit mask in the integer/boolean buffer.
pub const BOOL_MASK_OFFSET: u32 = INT_CONSTANTS * CONSTANT_STRIDE;

/// Descriptor set holding the samplers.
pub const SAMPLER_SET: u32 = 1;
/// First binding of pixel samplers (`s0`).
pub const PS_SAMPLER_BINDING_BASE: u32 = 0;
/// First binding of vertex samplers (`s0`).
pub const VS_SAMPLER_BINDING_BASE: u32 = 16;
/// Sampler registers in a pixel shader.
pub const PS_SAMPLERS: u16 = 16;
/// Sampler registers in a vertex shader.
pub const VS_SAMPLERS: u16 = 4;

/// Byte offset of the position fixup `vec4` in the vertex push constants.
pub const POSITION_FIXUP_OFFSET: u32 = 0;
/// Size in bytes of the vertex push-constant range.
pub const PUSH_CONSTANT_SIZE: u32 = 16;

/// Pixel colour outputs `oC0`-`oC3` use locations 0-3.
pub const COLOR_OUTPUTS: u16 = 4;

/// `D3DDECLUSAGE_*` values.
#[allow(missing_docs)]
pub mod usage {
    pub const POSITION: u8 = 0;
    pub const BLENDWEIGHT: u8 = 1;
    pub const BLENDINDICES: u8 = 2;
    pub const NORMAL: u8 = 3;
    pub const PSIZE: u8 = 4;
    pub const TEXCOORD: u8 = 5;
    pub const TANGENT: u8 = 6;
    pub const BINORMAL: u8 = 7;
    pub const TESSFACTOR: u8 = 8;
    pub const POSITIONT: u8 = 9;
    pub const COLOR: u8 = 10;
    pub const FOG: u8 = 11;
    pub const DEPTH: u8 = 12;
    pub const SAMPLE: u8 = 13;
}

/// Name of a usage value, for messages and reflection dumps.
#[must_use]
pub fn usage_name(u: u8) -> &'static str {
    match u {
        usage::POSITION => "position",
        usage::BLENDWEIGHT => "blendweight",
        usage::BLENDINDICES => "blendindices",
        usage::NORMAL => "normal",
        usage::PSIZE => "psize",
        usage::TEXCOORD => "texcoord",
        usage::TANGENT => "tangent",
        usage::BINORMAL => "binormal",
        usage::TESSFACTOR => "tessfactor",
        usage::POSITIONT => "positiont",
        usage::COLOR => "color",
        usage::FOG => "fog",
        usage::DEPTH => "depth",
        usage::SAMPLE => "sample",
        _ => "unknown",
    }
}

/// Vertex input locations: `(usage, usage index, location)`.
///
/// Sixteen entries, the Vulkan minimum for `maxVertexInputAttributes`.
/// Every attribute is read as a `vec4` of floats; the renderer picks a
/// vertex format that produces Direct3D 9's float values (for example a
/// scaled, not normalised, format for `UBYTE4` blend indices, and a
/// `B8G8R8A8_UNORM` format for `D3DCOLOR`).
pub const VERTEX_INPUT_LOCATIONS: &[(u8, u8, u32)] = &[
    (usage::POSITION, 0, 0),
    (usage::BLENDWEIGHT, 0, 1),
    (usage::BLENDINDICES, 0, 2),
    (usage::NORMAL, 0, 3),
    (usage::COLOR, 0, 4),
    (usage::COLOR, 1, 5),
    (usage::TEXCOORD, 0, 6),
    (usage::TEXCOORD, 1, 7),
    (usage::TEXCOORD, 2, 8),
    (usage::TEXCOORD, 3, 9),
    (usage::TEXCOORD, 4, 10),
    (usage::TEXCOORD, 5, 11),
    (usage::TEXCOORD, 6, 12),
    (usage::TEXCOORD, 7, 13),
    (usage::TANGENT, 0, 14),
    (usage::BINORMAL, 0, 15),
];

/// Varying locations (vertex outputs and pixel inputs):
/// `(usage, usage index, location)`.
///
/// Position 0 is the `Position` built-in and point size the `PointSize`
/// built-in, so neither appears here. The semantics shader model 3
/// programs normally use (texture coordinates 0-9, the ten `ps_3_0`
/// input registers' worth; two colours; fog; normal, tangent, binormal)
/// sit in locations 0-15, inside the Vulkan minimum of 64 components for
/// `maxVertexOutputComponents` and `maxFragmentInputComponents`. Rarer
/// semantics use 16-28 and need a limit of at least 116 components where
/// they occur (desktop drivers report 128). Only declared varyings are
/// emitted.
pub const VARYING_LOCATIONS: &[(u8, u8, u32)] = &[
    (usage::TEXCOORD, 0, 0),
    (usage::TEXCOORD, 1, 1),
    (usage::TEXCOORD, 2, 2),
    (usage::TEXCOORD, 3, 3),
    (usage::TEXCOORD, 4, 4),
    (usage::TEXCOORD, 5, 5),
    (usage::TEXCOORD, 6, 6),
    (usage::TEXCOORD, 7, 7),
    (usage::TEXCOORD, 8, 8),
    (usage::TEXCOORD, 9, 9),
    (usage::COLOR, 0, 10),
    (usage::COLOR, 1, 11),
    (usage::FOG, 0, 12),
    (usage::NORMAL, 0, 13),
    (usage::TANGENT, 0, 14),
    (usage::BINORMAL, 0, 15),
    (usage::TEXCOORD, 10, 16),
    (usage::TEXCOORD, 11, 17),
    (usage::TEXCOORD, 12, 18),
    (usage::TEXCOORD, 13, 19),
    (usage::TEXCOORD, 14, 20),
    (usage::TEXCOORD, 15, 21),
    (usage::NORMAL, 1, 22),
    (usage::POSITION, 1, 23),
    (usage::POSITION, 2, 24),
    (usage::POSITION, 3, 25),
    (usage::BLENDWEIGHT, 0, 26),
    (usage::BLENDINDICES, 0, 27),
    (usage::DEPTH, 0, 28),
];

fn lookup(table: &[(u8, u8, u32)], u: u8, index: u8) -> Option<u32> {
    table
        .iter()
        .find(|&&(tu, ti, _)| tu == u && ti == index)
        .map(|&(_, _, l)| l)
}

/// Location of a vertex input, if the convention has one.
#[must_use]
pub fn vertex_input_location(u: u8, index: u8) -> Option<u32> {
    lookup(VERTEX_INPUT_LOCATIONS, u, index)
}

/// Location of a varying, if the convention has one.
#[must_use]
pub fn varying_location(u: u8, index: u8) -> Option<u32> {
    lookup(VARYING_LOCATIONS, u, index)
}
