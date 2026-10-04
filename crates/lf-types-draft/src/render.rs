//! Rendering: render phases, particles, sky and shader fragments.
//!
//! Holds 20 draft layouts: `CRenderPhase` and `CRenderPhaseHeight`, RAGE's particle classes
//! (`rage::ptx*`, `rage::rmPtfx*`), the sky and procedural-texture classes, `rage::ShaderFragment`,
//! and `rmcInstanceWheelData` (its stated base is `rage::rmcInstanceDataBase`). Every layout is
//! Inferred; size confidence (the analysis lanes' own rating) is high for 0, medium for 0 and low
//! for 20. The conventions are those of the crate root.

use crate::Ptr32;
use lf_core::{assert_offset, assert_size};

/// Merged layout for `CRenderPhase`.
///
/// Size: 0x964 (low). Bases: none.
/// Lanes: c-render, via:CRenderPhaseBlit, via:CRenderPhaseCascadeShadows, via:CRenderPhaseDeferredLighting_LightsToScreen, via:CRenderPhaseDeferredLighting_SceneToGBuffer, via:CRenderPhaseDrawScene, via:CRenderPhaseHeight, via:CRenderPhaseHtml, via:CRenderPhaseInteriorReflection, via:CRenderPhaseMirrorReflection, via:CRenderPhasePlayerSettings, via:CRenderPhasePostRenderViewport, via:CRenderPhasePreRenderViewport, via:CRenderPhaseReflection, via:CRenderPhaseScript2d, via:CRenderPhaseWarpShadow, via:CRenderPhaseWaterReflection, via:CRenderPhaseWaterSurface.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CRenderPhase {
    /// Unknown bytes (0x0..0x10).
    pub _pad_0000: [u8; 0x10],
    /// field_10 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_14: u8,
    /// Unknown bytes (0x15..0x17).
    pub _pad_0015: [u8; 0x2],
    /// field_17 (confidence: low, kind: int16, lanes: c-render).
    pub field_17: [u8; 2],
    /// field_19 (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_19: u8,
    /// field_1a (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_1a: u8,
    /// field_1b (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_1b: u8,
    /// field_1c (confidence: high, kind: bool-or-byte, lanes: c-render,via:CRenderPhaseDeferredLighting_LightsToScreen,via:CRenderPhaseHeight,via:CRenderPhaseInteriorReflection,via:CRenderPhasePostRenderViewport,via:CRenderPhasePreRenderViewport,via:CRenderPhaseReflection,via:CRenderPhaseScript2d moved from siblings:CRenderPhaseDeferredLighting_LightsToScreen,CRenderPhaseHeight,CRenderPhaseInteriorReflection,CRenderPhasePostRenderViewport,CRenderPhasePreRenderViewport,CRenderPhaseReflection,CRenderPhaseScript2d).
    pub field_1c: u8,
    /// field_1d (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_1d: u8,
    /// field_1e (confidence: low, kind: int16, lanes: c-render).
    pub field_1e: u16,
    /// field_20 (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_20: u8,
    /// Unknown bytes (0x21..0x34).
    pub _pad_0021: [u8; 0x13],
    /// field_34 (confidence: low, kind: pointer, lanes: c-render).
    pub field_34: Ptr32<u8>,
    /// Unknown bytes (0x38..0x3c).
    pub _pad_0038: [u8; 0x4],
    /// field_3c (confidence: low, kind: pointer, lanes: c-render).
    pub field_3c: Ptr32<u8>,
    /// field_40 (confidence: low, kind: pointer, lanes: c-render).
    pub field_40: Ptr32<u8>,
    /// field_44 (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_44: u8,
    /// Unknown bytes (0x45..0x50).
    pub _pad_0045: [u8; 0xb],
    /// field_50 (confidence: low, kind: pointer, lanes: c-render).
    pub field_50: Ptr32<u8>,
    /// Unknown bytes (0x54..0xb0).
    pub _pad_0054: [u8; 0x5c],
    /// field_b0 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_b0: Ptr32<u8>,
    /// Unknown bytes (0xb4..0x4a0).
    pub _pad_00b4: [u8; 0x3ec],
    /// field_4a0 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_4a0: Ptr32<u8>,
    /// Unknown bytes (0x4a4..0x510).
    pub _pad_04a4: [u8; 0x6c],
    /// field_510 (confidence: medium, kind: pointer, lanes: c-render,via:CRenderPhaseInteriorReflection,via:CRenderPhaseMirrorReflection moved from siblings:CRenderPhaseInteriorReflection,CRenderPhaseMirrorReflection).
    pub field_510: Ptr32<u8>,
    /// field_514 (confidence: medium, kind: pointer, lanes: c-render,via:CRenderPhaseInteriorReflection,via:CRenderPhaseMirrorReflection moved from siblings:CRenderPhaseInteriorReflection,CRenderPhaseMirrorReflection).
    pub field_514: Ptr32<u8>,
    /// field_518 (confidence: medium, kind: pointer, lanes: c-render,via:CRenderPhaseInteriorReflection,via:CRenderPhaseMirrorReflection moved from siblings:CRenderPhaseInteriorReflection,CRenderPhaseMirrorReflection).
    pub field_518: Ptr32<u8>,
    /// Unknown bytes (0x51c..0x890).
    pub _pad_051c: [u8; 0x374],
    /// field_890 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_890: Ptr32<u8>,
    /// field_894 (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_894: u8,
    /// Unknown bytes (0x895..0x898).
    pub _pad_0895: [u8; 0x3],
    /// field_898 (confidence: low, kind: pointer, lanes: c-render).
    pub field_898: Ptr32<u8>,
    /// field_89c (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_89c: u8,
    /// Unknown bytes (0x89d..0x8a0).
    pub _pad_089d: [u8; 0x3],
    /// field_8a0 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_8a0: Ptr32<u8>,
    /// field_8a4 (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_8a4: u8,
    /// Unknown bytes (0x8a5..0x8a8).
    pub _pad_08a5: [u8; 0x3],
    /// field_8a8 (confidence: low, kind: pointer, lanes: c-render).
    pub field_8a8: Ptr32<u8>,
    /// field_8ac (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_8ac: u8,
    /// Unknown bytes (0x8ad..0x8b0).
    pub _pad_08ad: [u8; 0x3],
    /// field_8b0 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_8b0: Ptr32<u8>,
    /// field_8b4 (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_8b4: u8,
    /// Unknown bytes (0x8b5..0x8b8).
    pub _pad_08b5: [u8; 0x3],
    /// field_8b8 (confidence: low, kind: pointer, lanes: c-render).
    pub field_8b8: Ptr32<u8>,
    /// field_8bc (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_8bc: u8,
    /// Unknown bytes (0x8bd..0x8c0).
    pub _pad_08bd: [u8; 0x3],
    /// field_8c0 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_8c0: Ptr32<u8>,
    /// field_8c4 (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_8c4: u8,
    /// Unknown bytes (0x8c5..0x8c8).
    pub _pad_08c5: [u8; 0x3],
    /// field_8c8 (confidence: low, kind: pointer, lanes: c-render).
    pub field_8c8: Ptr32<u8>,
    /// field_8cc (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_8cc: u8,
    /// Unknown bytes (0x8cd..0x8d0).
    pub _pad_08cd: [u8; 0x3],
    /// field_8d0 (confidence: low, kind: pointer, lanes: c-render).
    pub field_8d0: Ptr32<u8>,
    /// Unknown bytes (0x8d4..0x8dc).
    pub _pad_08d4: [u8; 0x8],
    /// field_8dc (confidence: low, kind: pointer, lanes: c-render).
    pub field_8dc: Ptr32<u8>,
    /// Unknown bytes (0x8e0..0x8e8).
    pub _pad_08e0: [u8; 0x8],
    /// field_8e8 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_8e8: Ptr32<u8>,
    /// Unknown bytes (0x8ec..0x8f4).
    pub _pad_08ec: [u8; 0x8],
    /// field_8f4 (confidence: low, kind: pointer, lanes: c-render).
    pub field_8f4: Ptr32<u8>,
    /// field_8f8 (confidence: low, kind: pointer, lanes: c-render).
    pub field_8f8: Ptr32<u8>,
    /// field_8fc (confidence: low, kind: pointer, lanes: c-render).
    pub field_8fc: Ptr32<u8>,
    /// field_900 (confidence: medium, kind: pointer, lanes: c-render,via:CRenderPhaseCascadeShadows,via:CRenderPhaseDeferredLighting_SceneToGBuffer,via:CRenderPhaseDrawScene,via:CRenderPhaseHeight,via:CRenderPhaseInteriorReflection,via:CRenderPhaseMirrorReflection,via:CRenderPhaseReflection,via:CRenderPhaseWarpShadow,via:CRenderPhaseWaterReflection moved from siblings:CRenderPhaseCascadeShadows,CRenderPhaseDeferredLighting_SceneToGBuffer,CRenderPhaseDrawScene,CRenderPhaseHeight,CRenderPhaseInteriorReflection,CRenderPhaseMirrorReflection,CRenderPhaseReflection,CRenderPhaseWarpShadow,CRenderPhaseWaterReflection).
    pub field_900: Ptr32<u8>,
    /// Unknown bytes (0x904..0x934).
    pub _pad_0904: [u8; 0x30],
    /// field_934 (confidence: medium, kind: pointer, lanes: c-render,via:CRenderPhaseHeight,via:CRenderPhaseInteriorReflection moved from siblings:CRenderPhaseHeight,CRenderPhaseInteriorReflection).
    pub field_934: Ptr32<u8>,
    /// field_938 (confidence: low, kind: pointer, lanes: c-render).
    pub field_938: Ptr32<u8>,
    /// Unknown bytes (0x93c..0x940).
    pub _pad_093c: [u8; 0x4],
    /// field_940 (confidence: high, kind: pointer, lanes: c-render,via:CRenderPhaseBlit,via:CRenderPhaseCascadeShadows,via:CRenderPhaseDeferredLighting_SceneToGBuffer,via:CRenderPhaseHeight,via:CRenderPhaseHtml,via:CRenderPhaseInteriorReflection,via:CRenderPhaseMirrorReflection,via:CRenderPhasePlayerSettings,via:CRenderPhasePostRenderViewport,via:CRenderPhasePreRenderViewport,via:CRenderPhaseScript2d,via:CRenderPhaseWarpShadow,via:CRenderPhaseWaterSurface moved from siblings:CRenderPhaseBlit,CRenderPhaseCascadeShadows,CRenderPhaseDeferredLighting_SceneToGBuffer,CRenderPhaseHeight,CRenderPhaseHtml,CRenderPhaseInteriorReflection,CRenderPhaseMirrorReflection,CRenderPhasePlayerSettings,CRenderPhasePostRenderViewport,CRenderPhasePreRenderViewport,CRenderPhaseScript2d,CRenderPhaseWarpShadow,CRenderPhaseWaterSurface).
    pub field_940: Ptr32<u8>,
    /// field_944 (confidence: high, kind: pointer, lanes: c-render,via:CRenderPhaseDeferredLighting_SceneToGBuffer,via:CRenderPhaseHeight,via:CRenderPhasePlayerSettings,via:CRenderPhaseScript2d,via:CRenderPhaseWaterSurface moved from siblings:CRenderPhaseDeferredLighting_SceneToGBuffer,CRenderPhaseHeight,CRenderPhasePlayerSettings,CRenderPhaseScript2d,CRenderPhaseWaterSurface).
    pub field_944: Ptr32<u8>,
    /// field_948 (confidence: medium, kind: pointer, lanes: c-render,via:CRenderPhaseDrawScene,via:CRenderPhaseHtml,via:CRenderPhasePlayerSettings,via:CRenderPhaseScript2d moved from siblings:CRenderPhaseDrawScene,CRenderPhaseHtml,CRenderPhasePlayerSettings,CRenderPhaseScript2d).
    pub field_948: Ptr32<u8>,
    /// field_94c (confidence: high, kind: pointer, lanes: c-render,via:CRenderPhaseDrawScene,via:CRenderPhaseScript2d moved from siblings:CRenderPhaseDrawScene,CRenderPhaseScript2d).
    pub field_94c: Ptr32<u8>,
    /// field_950 (confidence: high, kind: pointer, lanes: c-render,via:CRenderPhaseDrawScene,via:CRenderPhaseHeight,via:CRenderPhaseScript2d moved from siblings:CRenderPhaseDrawScene,CRenderPhaseHeight,CRenderPhaseScript2d).
    pub field_950: Ptr32<u8>,
    /// field_954 (confidence: high, kind: pointer, lanes: c-render,via:CRenderPhaseDrawScene,via:CRenderPhaseHeight moved from siblings:CRenderPhaseDrawScene,CRenderPhaseHeight).
    pub field_954: Ptr32<u8>,
    /// field_958 (confidence: high, kind: pointer, lanes: c-render,via:CRenderPhaseDrawScene,via:CRenderPhaseHeight moved from siblings:CRenderPhaseDrawScene,CRenderPhaseHeight).
    pub field_958: Ptr32<u8>,
    /// field_95c (confidence: high, kind: pointer, lanes: c-render,via:CRenderPhaseDrawScene,via:CRenderPhaseHeight moved from siblings:CRenderPhaseDrawScene,CRenderPhaseHeight).
    pub field_95c: Ptr32<u8>,
    /// field_960 (confidence: high, kind: pointer, lanes: c-render,via:CRenderPhaseDrawScene,via:CRenderPhaseScript2d moved from siblings:CRenderPhaseDrawScene,CRenderPhaseScript2d).
    pub field_960: Ptr32<u8>,
}
assert_size!(CRenderPhase, 0x964); // merged size 0x964 rounded to 4
assert_offset!(CRenderPhase, field_10, 0x10);
assert_offset!(CRenderPhase, field_14, 0x14);
assert_offset!(CRenderPhase, field_17, 0x17);
assert_offset!(CRenderPhase, field_19, 0x19);
assert_offset!(CRenderPhase, field_1a, 0x1a);
assert_offset!(CRenderPhase, field_1b, 0x1b);
assert_offset!(CRenderPhase, field_1c, 0x1c);
assert_offset!(CRenderPhase, field_1d, 0x1d);
assert_offset!(CRenderPhase, field_1e, 0x1e);
assert_offset!(CRenderPhase, field_20, 0x20);
assert_offset!(CRenderPhase, field_34, 0x34);
assert_offset!(CRenderPhase, field_3c, 0x3c);
assert_offset!(CRenderPhase, field_40, 0x40);
assert_offset!(CRenderPhase, field_44, 0x44);
assert_offset!(CRenderPhase, field_50, 0x50);
assert_offset!(CRenderPhase, field_b0, 0xb0);
assert_offset!(CRenderPhase, field_4a0, 0x4a0);
assert_offset!(CRenderPhase, field_510, 0x510);
assert_offset!(CRenderPhase, field_514, 0x514);
assert_offset!(CRenderPhase, field_518, 0x518);
assert_offset!(CRenderPhase, field_890, 0x890);
assert_offset!(CRenderPhase, field_894, 0x894);
assert_offset!(CRenderPhase, field_898, 0x898);
assert_offset!(CRenderPhase, field_89c, 0x89c);
assert_offset!(CRenderPhase, field_8a0, 0x8a0);
assert_offset!(CRenderPhase, field_8a4, 0x8a4);
assert_offset!(CRenderPhase, field_8a8, 0x8a8);
assert_offset!(CRenderPhase, field_8ac, 0x8ac);
assert_offset!(CRenderPhase, field_8b0, 0x8b0);
assert_offset!(CRenderPhase, field_8b4, 0x8b4);
assert_offset!(CRenderPhase, field_8b8, 0x8b8);
assert_offset!(CRenderPhase, field_8bc, 0x8bc);
assert_offset!(CRenderPhase, field_8c0, 0x8c0);
assert_offset!(CRenderPhase, field_8c4, 0x8c4);
assert_offset!(CRenderPhase, field_8c8, 0x8c8);
assert_offset!(CRenderPhase, field_8cc, 0x8cc);
assert_offset!(CRenderPhase, field_8d0, 0x8d0);
assert_offset!(CRenderPhase, field_8dc, 0x8dc);
assert_offset!(CRenderPhase, field_8e8, 0x8e8);
assert_offset!(CRenderPhase, field_8f4, 0x8f4);
assert_offset!(CRenderPhase, field_8f8, 0x8f8);
assert_offset!(CRenderPhase, field_8fc, 0x8fc);
assert_offset!(CRenderPhase, field_900, 0x900);
assert_offset!(CRenderPhase, field_934, 0x934);
assert_offset!(CRenderPhase, field_938, 0x938);
assert_offset!(CRenderPhase, field_940, 0x940);
assert_offset!(CRenderPhase, field_944, 0x944);
assert_offset!(CRenderPhase, field_948, 0x948);
assert_offset!(CRenderPhase, field_94c, 0x94c);
assert_offset!(CRenderPhase, field_950, 0x950);
assert_offset!(CRenderPhase, field_954, 0x954);
assert_offset!(CRenderPhase, field_958, 0x958);
assert_offset!(CRenderPhase, field_95c, 0x95c);
assert_offset!(CRenderPhase, field_960, 0x960);

/// Merged layout for `CRenderPhaseHeight`.
///
/// Size: 0xa05 (low). Bases: CRenderPhase@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CRenderPhaseHeight {
    /// Unknown bytes (0x0..0x970).
    pub _pad_0000: [u8; 0x970],
    /// field_970 (confidence: high, kind: bool-or-byte, lanes: c-render).
    pub field_970: u8,
    /// Unknown bytes (0x971..0x980).
    pub _pad_0971: [u8; 0xf],
    /// field_980 (confidence: low, kind: pointer, lanes: c-render).
    pub field_980: Ptr32<u8>,
    /// field_984 (confidence: low, kind: pointer, lanes: c-render).
    pub field_984: Ptr32<u8>,
    /// field_988 (confidence: low, kind: pointer, lanes: c-render).
    pub field_988: Ptr32<u8>,
    /// field_98c (confidence: low, kind: pointer, lanes: c-render).
    pub field_98c: Ptr32<u8>,
    /// field_990 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_990: Ptr32<u8>,
    /// field_994 (confidence: low, kind: double, lanes: c-render).
    pub field_994: [u8; 8],
    /// field_99c (confidence: low, kind: double, lanes: c-render).
    pub field_99c: [u8; 8],
    /// Unknown bytes (0x9a4..0x9a8).
    pub _pad_09a4: [u8; 0x4],
    /// field_9a8 (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_9a8: u8,
    /// Unknown bytes (0x9a9..0x9f0).
    pub _pad_09a9: [u8; 0x47],
    /// field_9f0 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_9f0: Ptr32<u8>,
    /// field_9f4 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_9f4: Ptr32<u8>,
    /// field_9f8 (confidence: low, kind: pointer, lanes: c-render).
    pub field_9f8: Ptr32<u8>,
    /// field_9fc (confidence: low, kind: pointer, lanes: c-render).
    pub field_9fc: Ptr32<u8>,
    /// field_a00 (confidence: low, kind: pointer, lanes: c-render).
    pub field_a00: Ptr32<u8>,
    /// field_a04 (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_a04: u8,
    /// Unknown trailing bytes (0xa05..0xa08).
    pub _pad_end: [u8; 0x3],
}
assert_size!(CRenderPhaseHeight, 0xa08); // merged size 0xa05 rounded to 4
assert_offset!(CRenderPhaseHeight, field_970, 0x970);
assert_offset!(CRenderPhaseHeight, field_980, 0x980);
assert_offset!(CRenderPhaseHeight, field_984, 0x984);
assert_offset!(CRenderPhaseHeight, field_988, 0x988);
assert_offset!(CRenderPhaseHeight, field_98c, 0x98c);
assert_offset!(CRenderPhaseHeight, field_990, 0x990);
assert_offset!(CRenderPhaseHeight, field_994, 0x994);
assert_offset!(CRenderPhaseHeight, field_99c, 0x99c);
assert_offset!(CRenderPhaseHeight, field_9a8, 0x9a8);
assert_offset!(CRenderPhaseHeight, field_9f0, 0x9f0);
assert_offset!(CRenderPhaseHeight, field_9f4, 0x9f4);
assert_offset!(CRenderPhaseHeight, field_9f8, 0x9f8);
assert_offset!(CRenderPhaseHeight, field_9fc, 0x9fc);
assert_offset!(CRenderPhaseHeight, field_a00, 0xa00);
assert_offset!(CRenderPhaseHeight, field_a04, 0xa04);

/// Merged layout for `rage::ProceduralTextureRenderTargetDef`.
///
/// Size: 0x8c (low). Bases: none.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageProceduralTextureRenderTargetDef {
    /// Unknown bytes (0x0..0x4).
    pub _pad_0000: [u8; 0x4],
    /// field_4 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_4: Ptr32<u8>,
    /// Unknown bytes (0x8..0x44).
    pub _pad_0008: [u8; 0x3c],
    /// field_44 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_44: Ptr32<u8>,
    /// field_48 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_48: Ptr32<u8>,
    /// field_4c (confidence: medium, kind: pointer, lanes: c-render).
    pub field_4c: Ptr32<u8>,
    /// field_50 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_50: Ptr32<u8>,
    /// field_54 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_54: Ptr32<u8>,
    /// field_58 (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_58: u8,
    /// Unknown bytes (0x59..0x5c).
    pub _pad_0059: [u8; 0x3],
    /// field_5c (confidence: medium, kind: double, lanes: c-render).
    pub field_5c: [u8; 8],
    /// field_64 (confidence: medium, kind: double, lanes: c-render).
    pub field_64: [u8; 8],
    /// field_6c (confidence: medium, kind: double, lanes: c-render).
    pub field_6c: [u8; 8],
    /// field_74 (confidence: medium, kind: double, lanes: c-render).
    pub field_74: [u8; 8],
    /// field_7c (confidence: medium, kind: double, lanes: c-render).
    pub field_7c: [u8; 8],
    /// field_84 (confidence: medium, kind: double, lanes: c-render).
    pub field_84: [u8; 8],
}
assert_size!(RageProceduralTextureRenderTargetDef, 0x8c); // merged size 0x8c rounded to 4
assert_offset!(RageProceduralTextureRenderTargetDef, field_4, 0x4);
assert_offset!(RageProceduralTextureRenderTargetDef, field_44, 0x44);
assert_offset!(RageProceduralTextureRenderTargetDef, field_48, 0x48);
assert_offset!(RageProceduralTextureRenderTargetDef, field_4c, 0x4c);
assert_offset!(RageProceduralTextureRenderTargetDef, field_50, 0x50);
assert_offset!(RageProceduralTextureRenderTargetDef, field_54, 0x54);
assert_offset!(RageProceduralTextureRenderTargetDef, field_58, 0x58);
assert_offset!(RageProceduralTextureRenderTargetDef, field_5c, 0x5c);
assert_offset!(RageProceduralTextureRenderTargetDef, field_64, 0x64);
assert_offset!(RageProceduralTextureRenderTargetDef, field_6c, 0x6c);
assert_offset!(RageProceduralTextureRenderTargetDef, field_74, 0x74);
assert_offset!(RageProceduralTextureRenderTargetDef, field_7c, 0x7c);
assert_offset!(RageProceduralTextureRenderTargetDef, field_84, 0x84);

/// Merged layout for `rage::ProceduralTextureVerletWater`.
///
/// Size: 0xb0 (low). Bases: rage::ProceduralTexture@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageProceduralTextureVerletWater {
    /// Unknown bytes (0x0..0x34).
    pub _pad_0000: [u8; 0x34],
    /// field_34 (confidence: low, kind: pointer, lanes: c-render).
    pub field_34: Ptr32<u8>,
    /// field_38 (confidence: low, kind: pointer, lanes: c-render).
    pub field_38: Ptr32<u8>,
    /// field_3c (confidence: low, kind: pointer, lanes: c-render).
    pub field_3c: Ptr32<u8>,
    /// Unknown bytes (0x40..0x54).
    pub _pad_0040: [u8; 0x14],
    /// field_54 (confidence: low, kind: pointer, lanes: c-render).
    pub field_54: Ptr32<u8>,
    /// Unknown bytes (0x58..0x74).
    pub _pad_0058: [u8; 0x1c],
    /// field_74 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_74: Ptr32<u8>,
    /// field_78 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_78: Ptr32<u8>,
    /// field_7c (confidence: medium, kind: pointer, lanes: c-render).
    pub field_7c: Ptr32<u8>,
    /// field_80 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_80: Ptr32<u8>,
    /// field_84 (confidence: high, kind: pointer, lanes: c-render).
    pub field_84: Ptr32<u8>,
    /// field_88 (confidence: high, kind: pointer, lanes: c-render).
    pub field_88: Ptr32<u8>,
    /// field_8c (confidence: medium, kind: pointer, lanes: c-render).
    pub field_8c: Ptr32<u8>,
    /// field_90 (confidence: low, kind: double, lanes: c-render).
    pub field_90: [u8; 8],
    /// field_98 (confidence: low, kind: double, lanes: c-render).
    pub field_98: [u8; 8],
    /// field_a0 (confidence: low, kind: double, lanes: c-render).
    pub field_a0: [u8; 8],
    /// field_a8 (confidence: low, kind: double, lanes: c-render).
    pub field_a8: [u8; 8],
}
assert_size!(RageProceduralTextureVerletWater, 0xb0); // merged size 0xb0 rounded to 4
assert_offset!(RageProceduralTextureVerletWater, field_34, 0x34);
assert_offset!(RageProceduralTextureVerletWater, field_38, 0x38);
assert_offset!(RageProceduralTextureVerletWater, field_3c, 0x3c);
assert_offset!(RageProceduralTextureVerletWater, field_54, 0x54);
assert_offset!(RageProceduralTextureVerletWater, field_74, 0x74);
assert_offset!(RageProceduralTextureVerletWater, field_78, 0x78);
assert_offset!(RageProceduralTextureVerletWater, field_7c, 0x7c);
assert_offset!(RageProceduralTextureVerletWater, field_80, 0x80);
assert_offset!(RageProceduralTextureVerletWater, field_84, 0x84);
assert_offset!(RageProceduralTextureVerletWater, field_88, 0x88);
assert_offset!(RageProceduralTextureVerletWater, field_8c, 0x8c);
assert_offset!(RageProceduralTextureVerletWater, field_90, 0x90);
assert_offset!(RageProceduralTextureVerletWater, field_98, 0x98);
assert_offset!(RageProceduralTextureVerletWater, field_a0, 0xa0);
assert_offset!(RageProceduralTextureVerletWater, field_a8, 0xa8);

/// Merged layout for `rage::ShaderFragment`.
///
/// Size: 0xe8 (low). Bases: rage::datBase@0x0.
/// Lanes: c-misc-b, c-render, via:rage::AtmosphericScattering, via:rage::DayLighting, via:rage::EnviromentDomeLighting, via:rage::FogControl, via:rage::IntervalShadows, via:rage::SkyLightController, via:rage::SkyhatMiniNoise, via:rage::WaterFogControl.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageShaderFragment {
    /// Unknown bytes (0x0..0x4).
    pub _pad_0000: [u8; 0x4],
    /// field_4 (confidence: high, kind: flags, lanes: c-misc-b,via:rage::AtmosphericScattering,via:rage::DayLighting,via:rage::EnviromentDomeLighting,via:rage::FogControl,via:rage::IntervalShadows,via:rage::WaterFogControl moved from siblings:rage::AtmosphericScattering,rage::DayLighting,rage::EnviromentDomeLighting,rage::FogControl,rage::IntervalShadows,rage::WaterFogControl).
    pub field_4: u32,
    /// Unknown bytes (0x8..0x38).
    pub _pad_0008: [u8; 0x30],
    /// field_38 (confidence: high, kind: pointer, lanes: c-misc-b,c-render,via:rage::EnviromentDomeLighting,via:rage::FogControl,via:rage::SkyLightController,via:rage::SkyhatMiniNoise moved from siblings:rage::EnviromentDomeLighting,rage::FogControl,rage::SkyLightController,rage::SkyhatMiniNoise).
    pub field_38: Ptr32<u8>,
    /// field_3c (confidence: high, kind: pointer, lanes: c-misc-b,c-render,via:rage::FogControl,via:rage::SkyhatMiniNoise moved from siblings:rage::FogControl,rage::SkyhatMiniNoise).
    pub field_3c: Ptr32<u8>,
    /// Unknown bytes (0x40..0x54).
    pub _pad_0040: [u8; 0x14],
    /// field_54 (confidence: medium, kind: pointer, lanes: c-render,via:rage::SkyLightController,via:rage::SkyhatMiniNoise moved from siblings:rage::SkyLightController,rage::SkyhatMiniNoise).
    pub field_54: Ptr32<u8>,
    /// field_58 (confidence: medium, kind: pointer, lanes: c-render,via:rage::SkyLightController,via:rage::SkyhatMiniNoise moved from siblings:rage::SkyLightController,rage::SkyhatMiniNoise).
    pub field_58: Ptr32<u8>,
    /// Unknown bytes (0x5c..0x64).
    pub _pad_005c: [u8; 0x8],
    /// field_64 (confidence: medium, kind: pointer, lanes: c-render,via:rage::SkyLightController,via:rage::SkyhatMiniNoise moved from siblings:rage::SkyLightController,rage::SkyhatMiniNoise).
    pub field_64: Ptr32<u8>,
    /// Unknown bytes (0x68..0xe4).
    pub _pad_0068: [u8; 0x7c],
    /// field_e4 (confidence: medium, kind: flags, lanes: c-misc-b,c-render).
    pub field_e4: u32,
}
assert_size!(RageShaderFragment, 0xe8); // merged size 0xe8 rounded to 4
assert_offset!(RageShaderFragment, field_4, 0x4);
assert_offset!(RageShaderFragment, field_38, 0x38);
assert_offset!(RageShaderFragment, field_3c, 0x3c);
assert_offset!(RageShaderFragment, field_54, 0x54);
assert_offset!(RageShaderFragment, field_58, 0x58);
assert_offset!(RageShaderFragment, field_64, 0x64);
assert_offset!(RageShaderFragment, field_e4, 0xe4);

/// Merged layout for `rage::SkyDome`.
///
/// Size: 0x330 (low). Bases: rage::datBase@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageSkyDome {
    /// Unknown bytes (0x0..0x2c4).
    pub _pad_0000: [u8; 0x2c4],
    /// field_2c4 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_2c4: Ptr32<u8>,
    /// Unknown bytes (0x2c8..0x2cc).
    pub _pad_02c8: [u8; 0x4],
    /// field_2cc (confidence: medium, kind: pointer, lanes: c-render).
    pub field_2cc: Ptr32<u8>,
    /// Unknown bytes (0x2d0..0x2d4).
    pub _pad_02d0: [u8; 0x4],
    /// field_2d4 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_2d4: Ptr32<u8>,
    /// field_2d8 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_2d8: Ptr32<u8>,
    /// field_2dc (confidence: medium, kind: pointer, lanes: c-render).
    pub field_2dc: Ptr32<u8>,
    /// field_2e0 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_2e0: Ptr32<u8>,
    /// field_2e4 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_2e4: Ptr32<u8>,
    /// Unknown bytes (0x2e8..0x2ec).
    pub _pad_02e8: [u8; 0x4],
    /// field_2ec (confidence: low, kind: pointer, lanes: c-render).
    pub field_2ec: Ptr32<u8>,
    /// field_2f0 (confidence: low, kind: pointer, lanes: c-render).
    pub field_2f0: Ptr32<u8>,
    /// field_2f4 (confidence: low, kind: pointer, lanes: c-render).
    pub field_2f4: Ptr32<u8>,
    /// field_2f8 (confidence: low, kind: pointer, lanes: c-render).
    pub field_2f8: Ptr32<u8>,
    /// field_2fc (confidence: low, kind: pointer, lanes: c-render).
    pub field_2fc: Ptr32<u8>,
    /// field_300 (confidence: low, kind: pointer, lanes: c-render).
    pub field_300: Ptr32<u8>,
    /// field_304 (confidence: low, kind: pointer, lanes: c-render).
    pub field_304: Ptr32<u8>,
    /// Unknown bytes (0x308..0x314).
    pub _pad_0308: [u8; 0xc],
    /// field_314 (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_314: u8,
    /// Unknown bytes (0x315..0x328).
    pub _pad_0315: [u8; 0x13],
    /// field_328 (confidence: low, kind: int16, lanes: c-render).
    pub field_328: u16,
    /// Unknown trailing bytes (0x32a..0x330).
    pub _pad_end: [u8; 0x6],
}
assert_size!(RageSkyDome, 0x330); // merged size 0x330 rounded to 4
assert_offset!(RageSkyDome, field_2c4, 0x2c4);
assert_offset!(RageSkyDome, field_2cc, 0x2cc);
assert_offset!(RageSkyDome, field_2d4, 0x2d4);
assert_offset!(RageSkyDome, field_2d8, 0x2d8);
assert_offset!(RageSkyDome, field_2dc, 0x2dc);
assert_offset!(RageSkyDome, field_2e0, 0x2e0);
assert_offset!(RageSkyDome, field_2e4, 0x2e4);
assert_offset!(RageSkyDome, field_2ec, 0x2ec);
assert_offset!(RageSkyDome, field_2f0, 0x2f0);
assert_offset!(RageSkyDome, field_2f4, 0x2f4);
assert_offset!(RageSkyDome, field_2f8, 0x2f8);
assert_offset!(RageSkyDome, field_2fc, 0x2fc);
assert_offset!(RageSkyDome, field_300, 0x300);
assert_offset!(RageSkyDome, field_304, 0x304);
assert_offset!(RageSkyDome, field_314, 0x314);
assert_offset!(RageSkyDome, field_328, 0x328);

/// Merged layout for `rage::SkyDomeProceduralControl`.
///
/// Size: 0x104 (low). Bases: none.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageSkyDomeProceduralControl {
    /// Unknown bytes (0x0..0x10).
    pub _pad_0000: [u8; 0x10],
    /// field_10 (confidence: medium, kind: embedded-object, lanes: c-render).
    pub field_10: u32,
    /// Unknown bytes (0x14..0x70).
    pub _pad_0014: [u8; 0x5c],
    /// field_70 (confidence: medium, kind: embedded-object, lanes: c-render).
    pub field_70: u32,
    /// Unknown bytes (0x74..0xd0).
    pub _pad_0074: [u8; 0x5c],
    /// field_d0 (confidence: medium, kind: embedded-object, lanes: c-render).
    pub field_d0: u32,
    /// Unknown bytes (0xd4..0xf0).
    pub _pad_00d4: [u8; 0x1c],
    /// field_f0 (confidence: medium, kind: embedded-object, lanes: c-render).
    pub field_f0: u32,
    /// Unknown bytes (0xf4..0x100).
    pub _pad_00f4: [u8; 0xc],
    /// field_100 (confidence: medium, kind: embedded-object, lanes: c-render).
    pub field_100: u32,
}
assert_size!(RageSkyDomeProceduralControl, 0x104); // merged size 0x104 rounded to 4
assert_offset!(RageSkyDomeProceduralControl, field_10, 0x10);
assert_offset!(RageSkyDomeProceduralControl, field_70, 0x70);
assert_offset!(RageSkyDomeProceduralControl, field_d0, 0xd0);
assert_offset!(RageSkyDomeProceduralControl, field_f0, 0xf0);
assert_offset!(RageSkyDomeProceduralControl, field_100, 0x100);

/// Merged layout for `rage::SkyhatMiniNoise`.
///
/// Size: 0x231 (low). Bases: rage::ShaderFragment@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageSkyhatMiniNoise {
    /// Unknown bytes (0x0..0x28).
    pub _pad_0000: [u8; 0x28],
    /// field_28 (confidence: high, kind: pointer, lanes: c-render).
    pub field_28: Ptr32<u8>,
    /// Unknown bytes (0x2c..0x34).
    pub _pad_002c: [u8; 0x8],
    /// field_34 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_34: Ptr32<u8>,
    /// Unknown bytes (0x38..0x50).
    pub _pad_0038: [u8; 0x18],
    /// field_50 (confidence: low, kind: pointer, lanes: c-render).
    pub field_50: Ptr32<u8>,
    /// Unknown bytes (0x54..0x68).
    pub _pad_0054: [u8; 0x14],
    /// field_68 (confidence: low, kind: pointer, lanes: c-render).
    pub field_68: Ptr32<u8>,
    /// Unknown bytes (0x6c..0x70).
    pub _pad_006c: [u8; 0x4],
    /// field_70 (confidence: low, kind: pointer, lanes: c-render).
    pub field_70: Ptr32<u8>,
    /// field_74 (confidence: low, kind: pointer, lanes: c-render).
    pub field_74: Ptr32<u8>,
    /// field_78 (confidence: low, kind: pointer, lanes: c-render).
    pub field_78: Ptr32<u8>,
    /// Unknown bytes (0x7c..0x80).
    pub _pad_007c: [u8; 0x4],
    /// field_80 (confidence: low, kind: pointer, lanes: c-render).
    pub field_80: Ptr32<u8>,
    /// field_84 (confidence: low, kind: pointer, lanes: c-render).
    pub field_84: Ptr32<u8>,
    /// field_88 (confidence: low, kind: pointer, lanes: c-render).
    pub field_88: Ptr32<u8>,
    /// Unknown bytes (0x8c..0x90).
    pub _pad_008c: [u8; 0x4],
    /// field_90 (confidence: low, kind: pointer, lanes: c-render).
    pub field_90: Ptr32<u8>,
    /// field_94 (confidence: low, kind: pointer, lanes: c-render).
    pub field_94: Ptr32<u8>,
    /// Unknown bytes (0x98..0xa0).
    pub _pad_0098: [u8; 0x8],
    /// field_a0 (confidence: low, kind: pointer, lanes: c-render).
    pub field_a0: Ptr32<u8>,
    /// field_a4 (confidence: low, kind: pointer, lanes: c-render).
    pub field_a4: Ptr32<u8>,
    /// field_a8 (confidence: low, kind: pointer, lanes: c-render).
    pub field_a8: Ptr32<u8>,
    /// Unknown bytes (0xac..0xb0).
    pub _pad_00ac: [u8; 0x4],
    /// field_b0 (confidence: low, kind: pointer, lanes: c-render).
    pub field_b0: Ptr32<u8>,
    /// field_b4 (confidence: low, kind: pointer, lanes: c-render).
    pub field_b4: Ptr32<u8>,
    /// Unknown bytes (0xb8..0xc0).
    pub _pad_00b8: [u8; 0x8],
    /// field_c0 (confidence: low, kind: pointer, lanes: c-render).
    pub field_c0: Ptr32<u8>,
    /// field_c4 (confidence: low, kind: pointer, lanes: c-render).
    pub field_c4: Ptr32<u8>,
    /// field_c8 (confidence: low, kind: pointer, lanes: c-render).
    pub field_c8: Ptr32<u8>,
    /// field_cc (confidence: low, kind: pointer, lanes: c-render).
    pub field_cc: Ptr32<u8>,
    /// field_d0 (confidence: low, kind: pointer, lanes: c-render).
    pub field_d0: Ptr32<u8>,
    /// field_d4 (confidence: low, kind: pointer, lanes: c-render).
    pub field_d4: Ptr32<u8>,
    /// field_d8 (confidence: low, kind: pointer, lanes: c-render).
    pub field_d8: Ptr32<u8>,
    /// field_dc (confidence: low, kind: pointer, lanes: c-render).
    pub field_dc: Ptr32<u8>,
    /// field_e0 (confidence: low, kind: pointer, lanes: c-render).
    pub field_e0: Ptr32<u8>,
    /// Unknown bytes (0xe4..0xe8).
    pub _pad_00e4: [u8; 0x4],
    /// field_e8 (confidence: low, kind: pointer, lanes: c-render).
    pub field_e8: Ptr32<u8>,
    /// field_ec (confidence: low, kind: pointer, lanes: c-render).
    pub field_ec: Ptr32<u8>,
    /// field_f0 (confidence: low, kind: pointer, lanes: c-render).
    pub field_f0: Ptr32<u8>,
    /// field_f4 (confidence: low, kind: pointer, lanes: c-render).
    pub field_f4: Ptr32<u8>,
    /// field_f8 (confidence: low, kind: pointer, lanes: c-render).
    pub field_f8: Ptr32<u8>,
    /// field_fc (confidence: low, kind: pointer, lanes: c-render).
    pub field_fc: Ptr32<u8>,
    /// field_100 (confidence: low, kind: pointer, lanes: c-render).
    pub field_100: Ptr32<u8>,
    /// field_104 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_104: Ptr32<u8>,
    /// field_108 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_108: Ptr32<u8>,
    /// Unknown bytes (0x10c..0x160).
    pub _pad_010c: [u8; 0x54],
    /// field_160 (confidence: low, kind: pointer, lanes: c-render).
    pub field_160: Ptr32<u8>,
    /// Unknown bytes (0x164..0x1d0).
    pub _pad_0164: [u8; 0x6c],
    /// field_1d0 (confidence: low, kind: pointer, lanes: c-render).
    pub field_1d0: Ptr32<u8>,
    /// Unknown bytes (0x1d4..0x1e0).
    pub _pad_01d4: [u8; 0xc],
    /// field_1e0 (confidence: low, kind: pointer, lanes: c-render).
    pub field_1e0: Ptr32<u8>,
    /// Unknown trailing bytes (0x1e4..0x234).
    pub _pad_end: [u8; 0x50],
}
assert_size!(RageSkyhatMiniNoise, 0x234); // merged size 0x231 rounded to 4
assert_offset!(RageSkyhatMiniNoise, field_28, 0x28);
assert_offset!(RageSkyhatMiniNoise, field_34, 0x34);
assert_offset!(RageSkyhatMiniNoise, field_50, 0x50);
assert_offset!(RageSkyhatMiniNoise, field_68, 0x68);
assert_offset!(RageSkyhatMiniNoise, field_70, 0x70);
assert_offset!(RageSkyhatMiniNoise, field_74, 0x74);
assert_offset!(RageSkyhatMiniNoise, field_78, 0x78);
assert_offset!(RageSkyhatMiniNoise, field_80, 0x80);
assert_offset!(RageSkyhatMiniNoise, field_84, 0x84);
assert_offset!(RageSkyhatMiniNoise, field_88, 0x88);
assert_offset!(RageSkyhatMiniNoise, field_90, 0x90);
assert_offset!(RageSkyhatMiniNoise, field_94, 0x94);
assert_offset!(RageSkyhatMiniNoise, field_a0, 0xa0);
assert_offset!(RageSkyhatMiniNoise, field_a4, 0xa4);
assert_offset!(RageSkyhatMiniNoise, field_a8, 0xa8);
assert_offset!(RageSkyhatMiniNoise, field_b0, 0xb0);
assert_offset!(RageSkyhatMiniNoise, field_b4, 0xb4);
assert_offset!(RageSkyhatMiniNoise, field_c0, 0xc0);
assert_offset!(RageSkyhatMiniNoise, field_c4, 0xc4);
assert_offset!(RageSkyhatMiniNoise, field_c8, 0xc8);
assert_offset!(RageSkyhatMiniNoise, field_cc, 0xcc);
assert_offset!(RageSkyhatMiniNoise, field_d0, 0xd0);
assert_offset!(RageSkyhatMiniNoise, field_d4, 0xd4);
assert_offset!(RageSkyhatMiniNoise, field_d8, 0xd8);
assert_offset!(RageSkyhatMiniNoise, field_dc, 0xdc);
assert_offset!(RageSkyhatMiniNoise, field_e0, 0xe0);
assert_offset!(RageSkyhatMiniNoise, field_e8, 0xe8);
assert_offset!(RageSkyhatMiniNoise, field_ec, 0xec);
assert_offset!(RageSkyhatMiniNoise, field_f0, 0xf0);
assert_offset!(RageSkyhatMiniNoise, field_f4, 0xf4);
assert_offset!(RageSkyhatMiniNoise, field_f8, 0xf8);
assert_offset!(RageSkyhatMiniNoise, field_fc, 0xfc);
assert_offset!(RageSkyhatMiniNoise, field_100, 0x100);
assert_offset!(RageSkyhatMiniNoise, field_104, 0x104);
assert_offset!(RageSkyhatMiniNoise, field_108, 0x108);
assert_offset!(RageSkyhatMiniNoise, field_160, 0x160);
assert_offset!(RageSkyhatMiniNoise, field_1d0, 0x1d0);
assert_offset!(RageSkyhatMiniNoise, field_1e0, 0x1e0);

/// Merged layout for `rage::ptxDomain`.
///
/// Size: 0x138 (low). Bases: rage::datBase@0x0.
/// Lanes: c-render, via:rage::ptxDomainBox, via:rage::ptxDomainCylinder, via:rage::ptxDomainSphere, via:rage::ptxDomainVortex.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePtxDomain {
    /// Unknown bytes (0x0..0x4).
    pub _pad_0000: [u8; 0x4],
    /// field_4 (confidence: low, kind: pointer, lanes: c-render).
    pub field_4: Ptr32<u8>,
    /// Unknown bytes (0x8..0x28).
    pub _pad_0008: [u8; 0x20],
    /// field_28 (confidence: low, kind: pointer, lanes: c-render).
    pub field_28: Ptr32<u8>,
    /// Unknown bytes (0x2c..0x34).
    pub _pad_002c: [u8; 0x8],
    /// field_34 (confidence: low, kind: pointer, lanes: c-render).
    pub field_34: Ptr32<u8>,
    /// field_38 (confidence: high, kind: embedded-object, lanes: c-render).
    pub field_38: u32,
    /// Unknown bytes (0x3c..0x50).
    pub _pad_003c: [u8; 0x14],
    /// field_50 (confidence: low, kind: pointer, lanes: c-render).
    pub field_50: Ptr32<u8>,
    /// field_54 (confidence: low, kind: pointer, lanes: c-render).
    pub field_54: Ptr32<u8>,
    /// field_58 (confidence: low, kind: pointer, lanes: c-render).
    pub field_58: Ptr32<u8>,
    /// field_5c (confidence: low, kind: pointer, lanes: c-render).
    pub field_5c: Ptr32<u8>,
    /// Unknown bytes (0x60..0x78).
    pub _pad_0060: [u8; 0x18],
    /// field_78 (confidence: low, kind: pointer, lanes: c-render).
    pub field_78: Ptr32<u8>,
    /// field_7c (confidence: low, kind: pointer, lanes: c-render).
    pub field_7c: Ptr32<u8>,
    /// field_80 (confidence: low, kind: pointer, lanes: c-render).
    pub field_80: Ptr32<u8>,
    /// field_84 (confidence: low, kind: pointer, lanes: c-render).
    pub field_84: Ptr32<u8>,
    /// field_88 (confidence: high, kind: pointer, lanes: c-render).
    pub field_88: Ptr32<u8>,
    /// Unknown bytes (0x8c..0xa0).
    pub _pad_008c: [u8; 0x14],
    /// field_a0 (confidence: medium, kind: pointer, lanes: c-render,via:rage::ptxDomainBox,via:rage::ptxDomainCylinder moved from siblings:rage::ptxDomainBox,rage::ptxDomainCylinder).
    pub field_a0: Ptr32<u8>,
    /// field_a4 (confidence: medium, kind: int16, lanes: c-render,via:rage::ptxDomainBox,via:rage::ptxDomainCylinder moved from siblings:rage::ptxDomainBox,rage::ptxDomainCylinder).
    pub field_a4: u16,
    /// Unknown bytes (0xa6..0xb0).
    pub _pad_00a6: [u8; 0xa],
    /// field_b0 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxDomainBox,via:rage::ptxDomainCylinder,via:rage::ptxDomainSphere,via:rage::ptxDomainVortex moved from siblings:rage::ptxDomainBox,rage::ptxDomainCylinder,rage::ptxDomainSphere,rage::ptxDomainVortex).
    pub field_b0: Ptr32<u8>,
    /// field_b4 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxDomainBox,via:rage::ptxDomainCylinder,via:rage::ptxDomainSphere,via:rage::ptxDomainVortex moved from siblings:rage::ptxDomainBox,rage::ptxDomainCylinder,rage::ptxDomainSphere,rage::ptxDomainVortex).
    pub field_b4: Ptr32<u8>,
    /// field_b8 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxDomainBox,via:rage::ptxDomainCylinder,via:rage::ptxDomainSphere,via:rage::ptxDomainVortex moved from siblings:rage::ptxDomainBox,rage::ptxDomainCylinder,rage::ptxDomainSphere,rage::ptxDomainVortex).
    pub field_b8: Ptr32<u8>,
    /// Unknown bytes (0xbc..0xc0).
    pub _pad_00bc: [u8; 0x4],
    /// field_c0 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxDomainBox,via:rage::ptxDomainCylinder,via:rage::ptxDomainSphere,via:rage::ptxDomainVortex moved from siblings:rage::ptxDomainBox,rage::ptxDomainCylinder,rage::ptxDomainSphere,rage::ptxDomainVortex).
    pub field_c0: Ptr32<u8>,
    /// field_c4 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxDomainBox,via:rage::ptxDomainCylinder,via:rage::ptxDomainSphere,via:rage::ptxDomainVortex moved from siblings:rage::ptxDomainBox,rage::ptxDomainCylinder,rage::ptxDomainSphere,rage::ptxDomainVortex).
    pub field_c4: Ptr32<u8>,
    /// field_c8 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxDomainBox,via:rage::ptxDomainCylinder,via:rage::ptxDomainSphere,via:rage::ptxDomainVortex moved from siblings:rage::ptxDomainBox,rage::ptxDomainCylinder,rage::ptxDomainSphere,rage::ptxDomainVortex).
    pub field_c8: Ptr32<u8>,
    /// Unknown bytes (0xcc..0xd0).
    pub _pad_00cc: [u8; 0x4],
    /// field_d0 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxDomainBox,via:rage::ptxDomainCylinder,via:rage::ptxDomainSphere,via:rage::ptxDomainVortex moved from siblings:rage::ptxDomainBox,rage::ptxDomainCylinder,rage::ptxDomainSphere,rage::ptxDomainVortex).
    pub field_d0: Ptr32<u8>,
    /// field_d4 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxDomainBox,via:rage::ptxDomainCylinder,via:rage::ptxDomainSphere,via:rage::ptxDomainVortex moved from siblings:rage::ptxDomainBox,rage::ptxDomainCylinder,rage::ptxDomainSphere,rage::ptxDomainVortex).
    pub field_d4: Ptr32<u8>,
    /// field_d8 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxDomainBox,via:rage::ptxDomainCylinder,via:rage::ptxDomainSphere,via:rage::ptxDomainVortex moved from siblings:rage::ptxDomainBox,rage::ptxDomainCylinder,rage::ptxDomainSphere,rage::ptxDomainVortex).
    pub field_d8: Ptr32<u8>,
    /// Unknown bytes (0xdc..0xe0).
    pub _pad_00dc: [u8; 0x4],
    /// field_e0 (confidence: low, kind: pointer, lanes: c-render).
    pub field_e0: Ptr32<u8>,
    /// field_e4 (confidence: low, kind: pointer, lanes: c-render).
    pub field_e4: Ptr32<u8>,
    /// field_e8 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxDomainBox,via:rage::ptxDomainCylinder,via:rage::ptxDomainSphere,via:rage::ptxDomainVortex moved from siblings:rage::ptxDomainBox,rage::ptxDomainCylinder,rage::ptxDomainSphere,rage::ptxDomainVortex).
    pub field_e8: Ptr32<u8>,
    /// Unknown bytes (0xec..0xf0).
    pub _pad_00ec: [u8; 0x4],
    /// field_f0 (confidence: low, kind: pointer, lanes: c-render).
    pub field_f0: Ptr32<u8>,
    /// field_f4 (confidence: low, kind: pointer, lanes: c-render).
    pub field_f4: Ptr32<u8>,
    /// field_f8 (confidence: medium, kind: pointer, lanes: c-render,via:rage::ptxDomainBox,via:rage::ptxDomainCylinder,via:rage::ptxDomainSphere,via:rage::ptxDomainVortex moved from siblings:rage::ptxDomainBox,rage::ptxDomainCylinder,rage::ptxDomainSphere,rage::ptxDomainVortex).
    pub field_f8: Ptr32<u8>,
    /// Unknown bytes (0xfc..0x100).
    pub _pad_00fc: [u8; 0x4],
    /// field_100 (confidence: low, kind: pointer, lanes: c-render).
    pub field_100: Ptr32<u8>,
    /// field_104 (confidence: low, kind: pointer, lanes: c-render).
    pub field_104: Ptr32<u8>,
    /// field_108 (confidence: low, kind: pointer, lanes: c-render).
    pub field_108: Ptr32<u8>,
    /// field_10c (confidence: low, kind: pointer, lanes: c-render).
    pub field_10c: Ptr32<u8>,
    /// Unknown bytes (0x110..0x114).
    pub _pad_0110: [u8; 0x4],
    /// field_114 (confidence: low, kind: pointer, lanes: c-render).
    pub field_114: Ptr32<u8>,
    /// field_118 (confidence: low, kind: pointer, lanes: c-render).
    pub field_118: Ptr32<u8>,
    /// field_11c (confidence: medium, kind: pointer, lanes: c-render).
    pub field_11c: Ptr32<u8>,
    /// Unknown bytes (0x120..0x124).
    pub _pad_0120: [u8; 0x4],
    /// field_124 (confidence: low, kind: int16, lanes: c-render).
    pub field_124: u16,
    /// Unknown bytes (0x126..0x134).
    pub _pad_0126: [u8; 0xe],
    /// field_134 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxDomainBox,via:rage::ptxDomainCylinder,via:rage::ptxDomainSphere,via:rage::ptxDomainVortex moved from siblings:rage::ptxDomainBox,rage::ptxDomainCylinder,rage::ptxDomainSphere,rage::ptxDomainVortex).
    pub field_134: Ptr32<u8>,
}
assert_size!(RagePtxDomain, 0x138); // merged size 0x138 rounded to 4
assert_offset!(RagePtxDomain, field_4, 0x4);
assert_offset!(RagePtxDomain, field_28, 0x28);
assert_offset!(RagePtxDomain, field_34, 0x34);
assert_offset!(RagePtxDomain, field_38, 0x38);
assert_offset!(RagePtxDomain, field_50, 0x50);
assert_offset!(RagePtxDomain, field_54, 0x54);
assert_offset!(RagePtxDomain, field_58, 0x58);
assert_offset!(RagePtxDomain, field_5c, 0x5c);
assert_offset!(RagePtxDomain, field_78, 0x78);
assert_offset!(RagePtxDomain, field_7c, 0x7c);
assert_offset!(RagePtxDomain, field_80, 0x80);
assert_offset!(RagePtxDomain, field_84, 0x84);
assert_offset!(RagePtxDomain, field_88, 0x88);
assert_offset!(RagePtxDomain, field_a0, 0xa0);
assert_offset!(RagePtxDomain, field_a4, 0xa4);
assert_offset!(RagePtxDomain, field_b0, 0xb0);
assert_offset!(RagePtxDomain, field_b4, 0xb4);
assert_offset!(RagePtxDomain, field_b8, 0xb8);
assert_offset!(RagePtxDomain, field_c0, 0xc0);
assert_offset!(RagePtxDomain, field_c4, 0xc4);
assert_offset!(RagePtxDomain, field_c8, 0xc8);
assert_offset!(RagePtxDomain, field_d0, 0xd0);
assert_offset!(RagePtxDomain, field_d4, 0xd4);
assert_offset!(RagePtxDomain, field_d8, 0xd8);
assert_offset!(RagePtxDomain, field_e0, 0xe0);
assert_offset!(RagePtxDomain, field_e4, 0xe4);
assert_offset!(RagePtxDomain, field_e8, 0xe8);
assert_offset!(RagePtxDomain, field_f0, 0xf0);
assert_offset!(RagePtxDomain, field_f4, 0xf4);
assert_offset!(RagePtxDomain, field_f8, 0xf8);
assert_offset!(RagePtxDomain, field_100, 0x100);
assert_offset!(RagePtxDomain, field_104, 0x104);
assert_offset!(RagePtxDomain, field_108, 0x108);
assert_offset!(RagePtxDomain, field_10c, 0x10c);
assert_offset!(RagePtxDomain, field_114, 0x114);
assert_offset!(RagePtxDomain, field_118, 0x118);
assert_offset!(RagePtxDomain, field_11c, 0x11c);
assert_offset!(RagePtxDomain, field_124, 0x124);
assert_offset!(RagePtxDomain, field_134, 0x134);

/// Merged layout for `rage::ptxEffectRule`.
///
/// Size: 0x118 (low). Bases: rage::atReferenceCounter@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePtxEffectRule {
    /// Unknown bytes (0x0..0x20).
    pub _pad_0000: [u8; 0x20],
    /// field_20 (confidence: low, kind: pointer, lanes: c-render).
    pub field_20: Ptr32<u8>,
    /// Unknown bytes (0x24..0x58).
    pub _pad_0024: [u8; 0x34],
    /// field_58 (confidence: high, kind: embedded-object, lanes: c-render).
    pub field_58: u32,
    /// Unknown bytes (0x5c..0xa8).
    pub _pad_005c: [u8; 0x4c],
    /// field_a8 (confidence: high, kind: pointer, lanes: c-render).
    pub field_a8: Ptr32<u8>,
    /// Unknown bytes (0xac..0xd0).
    pub _pad_00ac: [u8; 0x24],
    /// field_d0 (confidence: high, kind: pointer, lanes: c-render).
    pub field_d0: Ptr32<u8>,
    /// Unknown bytes (0xd4..0xfc).
    pub _pad_00d4: [u8; 0x28],
    /// field_fc (confidence: low, kind: pointer, lanes: c-render).
    pub field_fc: Ptr32<u8>,
    /// Unknown bytes (0x100..0x10c).
    pub _pad_0100: [u8; 0xc],
    /// field_10c (confidence: low, kind: int16, lanes: c-render).
    pub field_10c: u16,
    /// field_10e (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_10e: u8,
    /// Unknown trailing bytes (0x10f..0x118).
    pub _pad_end: [u8; 0x9],
}
assert_size!(RagePtxEffectRule, 0x118); // merged size 0x118 rounded to 4
assert_offset!(RagePtxEffectRule, field_20, 0x20);
assert_offset!(RagePtxEffectRule, field_58, 0x58);
assert_offset!(RagePtxEffectRule, field_a8, 0xa8);
assert_offset!(RagePtxEffectRule, field_d0, 0xd0);
assert_offset!(RagePtxEffectRule, field_fc, 0xfc);
assert_offset!(RagePtxEffectRule, field_10c, 0x10c);
assert_offset!(RagePtxEffectRule, field_10e, 0x10e);

/// Merged layout for `rage::ptxEffectRuleStd`.
///
/// Size: 0x19c (low). Bases: rage::ptxEffectRule@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePtxEffectRuleStd {
    /// Unknown bytes (0x0..0x134).
    pub _pad_0000: [u8; 0x134],
    /// field_134 (confidence: high, kind: int16, lanes: c-render).
    pub field_134: u16,
    /// Unknown bytes (0x136..0x15c).
    pub _pad_0136: [u8; 0x26],
    /// field_15c (confidence: high, kind: pointer, lanes: c-render).
    pub field_15c: Ptr32<u8>,
    /// Unknown bytes (0x160..0x16c).
    pub _pad_0160: [u8; 0xc],
    /// field_16c (confidence: low, kind: pointer, lanes: c-render).
    pub field_16c: Ptr32<u8>,
    /// Unknown bytes (0x170..0x174).
    pub _pad_0170: [u8; 0x4],
    /// field_174 (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_174: u8,
    /// field_175 (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_175: u8,
    /// field_176 (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_176: u8,
    /// field_177 (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_177: u8,
    /// field_178 (confidence: low, kind: int16, lanes: c-render).
    pub field_178: u16,
    /// Unknown trailing bytes (0x17a..0x19c).
    pub _pad_end: [u8; 0x22],
}
assert_size!(RagePtxEffectRuleStd, 0x19c); // merged size 0x19c rounded to 4
assert_offset!(RagePtxEffectRuleStd, field_134, 0x134);
assert_offset!(RagePtxEffectRuleStd, field_15c, 0x15c);
assert_offset!(RagePtxEffectRuleStd, field_16c, 0x16c);
assert_offset!(RagePtxEffectRuleStd, field_174, 0x174);
assert_offset!(RagePtxEffectRuleStd, field_175, 0x175);
assert_offset!(RagePtxEffectRuleStd, field_176, 0x176);
assert_offset!(RagePtxEffectRuleStd, field_177, 0x177);
assert_offset!(RagePtxEffectRuleStd, field_178, 0x178);

/// Merged layout for `rage::ptxEmitRuleStd`.
///
/// Size: 0x1ed (low). Bases: rage::ptxEmitRule@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePtxEmitRuleStd {
    /// Unknown bytes (0x0..0x9c).
    pub _pad_0000: [u8; 0x9c],
    /// field_9c (confidence: low, kind: pointer, lanes: c-render).
    pub field_9c: Ptr32<u8>,
    /// Unknown bytes (0xa0..0xc4).
    pub _pad_00a0: [u8; 0x24],
    /// field_c4 (confidence: low, kind: pointer, lanes: c-render).
    pub field_c4: Ptr32<u8>,
    /// Unknown bytes (0xc8..0xec).
    pub _pad_00c8: [u8; 0x24],
    /// field_ec (confidence: low, kind: pointer, lanes: c-render).
    pub field_ec: Ptr32<u8>,
    /// Unknown bytes (0xf0..0x13c).
    pub _pad_00f0: [u8; 0x4c],
    /// field_13c (confidence: low, kind: pointer, lanes: c-render).
    pub field_13c: Ptr32<u8>,
    /// Unknown bytes (0x140..0x18c).
    pub _pad_0140: [u8; 0x4c],
    /// field_18c (confidence: medium, kind: pointer, lanes: c-render).
    pub field_18c: Ptr32<u8>,
    /// Unknown bytes (0x190..0x1b4).
    pub _pad_0190: [u8; 0x24],
    /// field_1b4 (confidence: low, kind: pointer, lanes: c-render).
    pub field_1b4: Ptr32<u8>,
    /// field_1b8 (confidence: low, kind: pointer, lanes: c-render).
    pub field_1b8: Ptr32<u8>,
    /// Unknown bytes (0x1bc..0x1e0).
    pub _pad_01bc: [u8; 0x24],
    /// field_1e0 (confidence: high, kind: pointer, lanes: c-render).
    pub field_1e0: Ptr32<u8>,
    /// Unknown bytes (0x1e4..0x1e8).
    pub _pad_01e4: [u8; 0x4],
    /// field_1e8 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_1e8: Ptr32<u8>,
    /// field_1ec (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_1ec: u8,
    /// Unknown trailing bytes (0x1ed..0x1f0).
    pub _pad_end: [u8; 0x3],
}
assert_size!(RagePtxEmitRuleStd, 0x1f0); // merged size 0x1ed rounded to 4
assert_offset!(RagePtxEmitRuleStd, field_9c, 0x9c);
assert_offset!(RagePtxEmitRuleStd, field_c4, 0xc4);
assert_offset!(RagePtxEmitRuleStd, field_ec, 0xec);
assert_offset!(RagePtxEmitRuleStd, field_13c, 0x13c);
assert_offset!(RagePtxEmitRuleStd, field_18c, 0x18c);
assert_offset!(RagePtxEmitRuleStd, field_1b4, 0x1b4);
assert_offset!(RagePtxEmitRuleStd, field_1b8, 0x1b8);
assert_offset!(RagePtxEmitRuleStd, field_1e0, 0x1e0);
assert_offset!(RagePtxEmitRuleStd, field_1e8, 0x1e8);
assert_offset!(RagePtxEmitRuleStd, field_1ec, 0x1ec);

/// Merged layout for `rage::ptxEvent`.
///
/// Size: 0xa0 (low). Bases: none.
/// Lanes: c-render, via:rage::ptxEventEffect, via:rage::ptxEventEmitter.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePtxEvent {
    /// Unknown bytes (0x0..0x4).
    pub _pad_0000: [u8; 0x4],
    /// field_4 (confidence: low, kind: pointer, lanes: c-render).
    pub field_4: Ptr32<u8>,
    /// field_8 (confidence: low, kind: pointer, lanes: c-render).
    pub field_8: Ptr32<u8>,
    /// field_c (confidence: high, kind: pointer, lanes: c-render).
    pub field_c: Ptr32<u8>,
    /// field_10 (confidence: low, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: low, kind: pointer, lanes: c-render).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: low, kind: pointer, lanes: c-render).
    pub field_18: Ptr32<u8>,
    /// field_1c (confidence: low, kind: pointer, lanes: c-render).
    pub field_1c: Ptr32<u8>,
    /// field_20 (confidence: medium, kind: pointer, lanes: c-render,via:rage::ptxEventEffect,via:rage::ptxEventEmitter moved from siblings:rage::ptxEventEffect,rage::ptxEventEmitter).
    pub field_20: Ptr32<u8>,
    /// field_24 (confidence: medium, kind: pointer, lanes: c-render,via:rage::ptxEventEffect,via:rage::ptxEventEmitter moved from siblings:rage::ptxEventEffect,rage::ptxEventEmitter).
    pub field_24: Ptr32<u8>,
    /// field_28 (confidence: medium, kind: pointer, lanes: c-render,via:rage::ptxEventEffect,via:rage::ptxEventEmitter moved from siblings:rage::ptxEventEffect,rage::ptxEventEmitter).
    pub field_28: Ptr32<u8>,
    /// Unknown bytes (0x2c..0x30).
    pub _pad_002c: [u8; 0x4],
    /// field_30 (confidence: medium, kind: pointer, lanes: c-render,via:rage::ptxEventEffect,via:rage::ptxEventEmitter moved from siblings:rage::ptxEventEffect,rage::ptxEventEmitter).
    pub field_30: Ptr32<u8>,
    /// field_34 (confidence: medium, kind: pointer, lanes: c-render,via:rage::ptxEventEffect,via:rage::ptxEventEmitter moved from siblings:rage::ptxEventEffect,rage::ptxEventEmitter).
    pub field_34: Ptr32<u8>,
    /// field_38 (confidence: medium, kind: pointer, lanes: c-render,via:rage::ptxEventEffect,via:rage::ptxEventEmitter moved from siblings:rage::ptxEventEffect,rage::ptxEventEmitter).
    pub field_38: Ptr32<u8>,
    /// Unknown bytes (0x3c..0x40).
    pub _pad_003c: [u8; 0x4],
    /// field_40 (confidence: low, kind: pointer, lanes: c-render).
    pub field_40: Ptr32<u8>,
    /// field_44 (confidence: low, kind: pointer, lanes: c-render).
    pub field_44: Ptr32<u8>,
    /// field_48 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxEventEffect,via:rage::ptxEventEmitter moved from siblings:rage::ptxEventEffect,rage::ptxEventEmitter).
    pub field_48: Ptr32<u8>,
    /// field_4c (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxEventEffect,via:rage::ptxEventEmitter moved from siblings:rage::ptxEventEffect,rage::ptxEventEmitter).
    pub field_4c: Ptr32<u8>,
    /// Unknown bytes (0x50..0x90).
    pub _pad_0050: [u8; 0x40],
    /// field_90 (confidence: low, kind: pointer, lanes: c-render).
    pub field_90: Ptr32<u8>,
    /// Unknown bytes (0x94..0x9c).
    pub _pad_0094: [u8; 0x8],
    /// field_9c (confidence: low, kind: pointer, lanes: c-render).
    pub field_9c: Ptr32<u8>,
}
assert_size!(RagePtxEvent, 0xa0); // merged size 0xa0 rounded to 4
assert_offset!(RagePtxEvent, field_4, 0x4);
assert_offset!(RagePtxEvent, field_8, 0x8);
assert_offset!(RagePtxEvent, field_c, 0xc);
assert_offset!(RagePtxEvent, field_10, 0x10);
assert_offset!(RagePtxEvent, field_14, 0x14);
assert_offset!(RagePtxEvent, field_18, 0x18);
assert_offset!(RagePtxEvent, field_1c, 0x1c);
assert_offset!(RagePtxEvent, field_20, 0x20);
assert_offset!(RagePtxEvent, field_24, 0x24);
assert_offset!(RagePtxEvent, field_28, 0x28);
assert_offset!(RagePtxEvent, field_30, 0x30);
assert_offset!(RagePtxEvent, field_34, 0x34);
assert_offset!(RagePtxEvent, field_38, 0x38);
assert_offset!(RagePtxEvent, field_40, 0x40);
assert_offset!(RagePtxEvent, field_44, 0x44);
assert_offset!(RagePtxEvent, field_48, 0x48);
assert_offset!(RagePtxEvent, field_4c, 0x4c);
assert_offset!(RagePtxEvent, field_90, 0x90);
assert_offset!(RagePtxEvent, field_9c, 0x9c);

/// Merged layout for `rage::ptxRule`.
///
/// Size: 0x695 (low). Bases: rage::atReferenceCounter@0x0.
/// Lanes: c-render, via:rage::ptxModel, via:rage::ptxSprite.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePtxRule {
    /// Unknown bytes (0x0..0x20).
    pub _pad_0000: [u8; 0x20],
    /// field_20 (confidence: low, kind: pointer, lanes: c-render).
    pub field_20: Ptr32<u8>,
    /// Unknown bytes (0x24..0xf0).
    pub _pad_0024: [u8; 0xcc],
    /// field_f0 (confidence: low, kind: pointer, lanes: c-render).
    pub field_f0: Ptr32<u8>,
    /// Unknown bytes (0xf4..0x10c).
    pub _pad_00f4: [u8; 0x18],
    /// field_10c (confidence: low, kind: pointer, lanes: c-render).
    pub field_10c: Ptr32<u8>,
    /// Unknown bytes (0x110..0x118).
    pub _pad_0110: [u8; 0x8],
    /// field_118 (confidence: medium, kind: pointer, lanes: c-render,via:rage::ptxModel,via:rage::ptxSprite moved from siblings:rage::ptxModel,rage::ptxSprite).
    pub field_118: Ptr32<u8>,
    /// field_11c (confidence: high, kind: pointer, lanes: c-render).
    pub field_11c: Ptr32<u8>,
    /// Unknown bytes (0x120..0x128).
    pub _pad_0120: [u8; 0x8],
    /// field_128 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_128: Ptr32<u8>,
    /// Unknown bytes (0x12c..0x174).
    pub _pad_012c: [u8; 0x48],
    /// field_174 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxModel,via:rage::ptxSprite moved from siblings:rage::ptxModel,rage::ptxSprite).
    pub field_174: Ptr32<u8>,
    /// Unknown bytes (0x178..0x690).
    pub _pad_0178: [u8; 0x518],
    /// field_690 (confidence: medium, kind: pointer, lanes: c-render,via:rage::ptxModel,via:rage::ptxSprite moved from siblings:rage::ptxModel,rage::ptxSprite).
    pub field_690: Ptr32<u8>,
    /// field_694 (confidence: medium, kind: bool-or-byte, lanes: c-render,via:rage::ptxModel,via:rage::ptxSprite moved from siblings:rage::ptxModel,rage::ptxSprite).
    pub field_694: u8,
    /// Unknown trailing bytes (0x695..0x698).
    pub _pad_end: [u8; 0x3],
}
assert_size!(RagePtxRule, 0x698); // merged size 0x695 rounded to 4
assert_offset!(RagePtxRule, field_20, 0x20);
assert_offset!(RagePtxRule, field_f0, 0xf0);
assert_offset!(RagePtxRule, field_10c, 0x10c);
assert_offset!(RagePtxRule, field_118, 0x118);
assert_offset!(RagePtxRule, field_11c, 0x11c);
assert_offset!(RagePtxRule, field_128, 0x128);
assert_offset!(RagePtxRule, field_174, 0x174);
assert_offset!(RagePtxRule, field_690, 0x690);
assert_offset!(RagePtxRule, field_694, 0x694);

/// Merged layout for `rage::ptxRulePropList`.
///
/// Size: 0x424 (low). Bases: rage::datBase@0x0.
/// Lanes: c-render, via:rage::ptxModelRulePropList, via:rage::ptxSpriteRulePropList.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePtxRulePropList {
    /// Unknown bytes (0x0..0x4).
    pub _pad_0000: [u8; 0x4],
    /// field_4 (confidence: high, kind: pointer, lanes: c-render).
    pub field_4: Ptr32<u8>,
    /// Unknown bytes (0x8..0x78).
    pub _pad_0008: [u8; 0x70],
    /// field_78 (confidence: high, kind: pointer, lanes: c-render).
    pub field_78: Ptr32<u8>,
    /// Unknown bytes (0x7c..0xac).
    pub _pad_007c: [u8; 0x30],
    /// field_ac (confidence: high, kind: pointer, lanes: c-render).
    pub field_ac: Ptr32<u8>,
    /// Unknown bytes (0xb0..0xe0).
    pub _pad_00b0: [u8; 0x30],
    /// field_e0 (confidence: high, kind: pointer, lanes: c-render).
    pub field_e0: Ptr32<u8>,
    /// Unknown bytes (0xe4..0x114).
    pub _pad_00e4: [u8; 0x30],
    /// field_114 (confidence: high, kind: pointer, lanes: c-render).
    pub field_114: Ptr32<u8>,
    /// Unknown bytes (0x118..0x1e4).
    pub _pad_0118: [u8; 0xcc],
    /// field_1e4 (confidence: high, kind: pointer, lanes: c-render).
    pub field_1e4: Ptr32<u8>,
    /// Unknown bytes (0x1e8..0x2b4).
    pub _pad_01e8: [u8; 0xcc],
    /// field_2b4 (confidence: high, kind: pointer, lanes: c-render).
    pub field_2b4: Ptr32<u8>,
    /// Unknown bytes (0x2b8..0x3b8).
    pub _pad_02b8: [u8; 0x100],
    /// field_3b8 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxModelRulePropList,via:rage::ptxSpriteRulePropList moved from siblings:rage::ptxModelRulePropList,rage::ptxSpriteRulePropList).
    pub field_3b8: Ptr32<u8>,
    /// Unknown bytes (0x3bc..0x3ec).
    pub _pad_03bc: [u8; 0x30],
    /// field_3ec (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxModelRulePropList,via:rage::ptxSpriteRulePropList moved from siblings:rage::ptxModelRulePropList,rage::ptxSpriteRulePropList).
    pub field_3ec: Ptr32<u8>,
    /// Unknown bytes (0x3f0..0x420).
    pub _pad_03f0: [u8; 0x30],
    /// field_420 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxModelRulePropList,via:rage::ptxSpriteRulePropList moved from siblings:rage::ptxModelRulePropList,rage::ptxSpriteRulePropList).
    pub field_420: Ptr32<u8>,
}
assert_size!(RagePtxRulePropList, 0x424); // merged size 0x424 rounded to 4
assert_offset!(RagePtxRulePropList, field_4, 0x4);
assert_offset!(RagePtxRulePropList, field_78, 0x78);
assert_offset!(RagePtxRulePropList, field_ac, 0xac);
assert_offset!(RagePtxRulePropList, field_e0, 0xe0);
assert_offset!(RagePtxRulePropList, field_114, 0x114);
assert_offset!(RagePtxRulePropList, field_1e4, 0x1e4);
assert_offset!(RagePtxRulePropList, field_2b4, 0x2b4);
assert_offset!(RagePtxRulePropList, field_3b8, 0x3b8);
assert_offset!(RagePtxRulePropList, field_3ec, 0x3ec);
assert_offset!(RagePtxRulePropList, field_420, 0x420);

/// Merged layout for `rage::ptxSprite`.
///
/// Size: 0x69d (low). Bases: rage::ptxRule@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePtxSprite {
    /// Unknown bytes (0x0..0x178).
    pub _pad_0000: [u8; 0x178],
    /// field_178 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_178: Ptr32<u8>,
    /// Unknown bytes (0x17c..0x1b4).
    pub _pad_017c: [u8; 0x38],
    /// field_1b4 (confidence: low, kind: int16, lanes: c-render).
    pub field_1b4: u16,
    /// Unknown bytes (0x1b6..0x1e8).
    pub _pad_01b6: [u8; 0x32],
    /// field_1e8 (confidence: low, kind: int16, lanes: c-render).
    pub field_1e8: u16,
    /// Unknown bytes (0x1ea..0x320).
    pub _pad_01ea: [u8; 0x136],
    /// field_320 (confidence: low, kind: int16, lanes: c-render).
    pub field_320: u16,
    /// Unknown bytes (0x322..0x335).
    pub _pad_0322: [u8; 0x13],
    /// field_335 (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_335: u8,
    /// Unknown bytes (0x336..0x4c0).
    pub _pad_0336: [u8; 0x18a],
    /// field_4c0 (confidence: low, kind: int16, lanes: c-render).
    pub field_4c0: u16,
    /// Unknown bytes (0x4c2..0x4cc).
    pub _pad_04c2: [u8; 0xa],
    /// field_4cc (confidence: low, kind: pointer, lanes: c-render).
    pub field_4cc: Ptr32<u8>,
    /// Unknown bytes (0x4d0..0x4d5).
    pub _pad_04d0: [u8; 0x5],
    /// field_4d5 (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_4d5: u8,
    /// Unknown bytes (0x4d6..0x678).
    pub _pad_04d6: [u8; 0x1a2],
    /// field_678 (confidence: high, kind: embedded-object, lanes: c-render).
    pub field_678: u32,
    /// field_67c (confidence: high, kind: pointer, lanes: c-render).
    pub field_67c: Ptr32<u8>,
    /// field_680 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_680: Ptr32<u8>,
    /// field_684 (confidence: high, kind: pointer, lanes: c-render).
    pub field_684: Ptr32<u8>,
    /// field_688 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_688: Ptr32<u8>,
    /// field_68c (confidence: low, kind: pointer, lanes: c-render).
    pub field_68c: Ptr32<u8>,
    /// Unknown bytes (0x690..0x698).
    pub _pad_0690: [u8; 0x8],
    /// field_698 (confidence: low, kind: pointer, lanes: c-render).
    pub field_698: Ptr32<u8>,
    /// field_69c (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_69c: u8,
    /// Unknown trailing bytes (0x69d..0x6a0).
    pub _pad_end: [u8; 0x3],
}
assert_size!(RagePtxSprite, 0x6a0); // merged size 0x69d rounded to 4
assert_offset!(RagePtxSprite, field_178, 0x178);
assert_offset!(RagePtxSprite, field_1b4, 0x1b4);
assert_offset!(RagePtxSprite, field_1e8, 0x1e8);
assert_offset!(RagePtxSprite, field_320, 0x320);
assert_offset!(RagePtxSprite, field_335, 0x335);
assert_offset!(RagePtxSprite, field_4c0, 0x4c0);
assert_offset!(RagePtxSprite, field_4cc, 0x4cc);
assert_offset!(RagePtxSprite, field_4d5, 0x4d5);
assert_offset!(RagePtxSprite, field_678, 0x678);
assert_offset!(RagePtxSprite, field_67c, 0x67c);
assert_offset!(RagePtxSprite, field_680, 0x680);
assert_offset!(RagePtxSprite, field_684, 0x684);
assert_offset!(RagePtxSprite, field_688, 0x688);
assert_offset!(RagePtxSprite, field_68c, 0x68c);
assert_offset!(RagePtxSprite, field_698, 0x698);
assert_offset!(RagePtxSprite, field_69c, 0x69c);

/// Merged layout for `rage::ptxTimeLine`.
///
/// Size: 0x20 (low). Bases: none.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePtxTimeLine {
    /// Unknown bytes (0x0..0x4).
    pub _pad_0000: [u8; 0x4],
    /// field_4 (confidence: low, kind: pointer, lanes: c-render).
    pub field_4: Ptr32<u8>,
    /// field_8 (confidence: low, kind: pointer, lanes: c-render).
    pub field_8: Ptr32<u8>,
    /// field_c (confidence: low, kind: pointer, lanes: c-render).
    pub field_c: Ptr32<u8>,
    /// field_10 (confidence: high, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_18: Ptr32<u8>,
    /// field_1c (confidence: low, kind: pointer, lanes: c-render).
    pub field_1c: Ptr32<u8>,
}
assert_size!(RagePtxTimeLine, 0x20); // merged size 0x20 rounded to 4
assert_offset!(RagePtxTimeLine, field_4, 0x4);
assert_offset!(RagePtxTimeLine, field_8, 0x8);
assert_offset!(RagePtxTimeLine, field_c, 0xc);
assert_offset!(RagePtxTimeLine, field_10, 0x10);
assert_offset!(RagePtxTimeLine, field_14, 0x14);
assert_offset!(RagePtxTimeLine, field_18, 0x18);
assert_offset!(RagePtxTimeLine, field_1c, 0x1c);

/// Merged layout for `rage::rmPtfxManager`.
///
/// Size: 0x4c0 (low). Bases: rage::datBase@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageRmPtfxManager {
    /// Unknown bytes (0x0..0x4).
    pub _pad_0000: [u8; 0x4],
    /// field_4 (confidence: high, kind: pointer, lanes: c-misc-b,c-render).
    pub field_4: Ptr32<u8>,
    /// Unknown bytes (0x8..0x20).
    pub _pad_0008: [u8; 0x18],
    /// field_20 (confidence: medium, kind: pointer, lanes: c-misc-b,c-render).
    pub field_20: Ptr32<u8>,
    /// Unknown bytes (0x24..0x38).
    pub _pad_0024: [u8; 0x14],
    /// field_38 (confidence: low, kind: double, lanes: c-render).
    pub field_38: [u8; 8],
    /// Unknown bytes (0x40..0x50).
    pub _pad_0040: [u8; 0x10],
    /// field_50 (confidence: low, kind: double, lanes: c-render).
    pub field_50: [u8; 8],
    /// field_58 (confidence: low, kind: double, lanes: c-render).
    pub field_58: [u8; 8],
    /// Unknown bytes (0x60..0x68).
    pub _pad_0060: [u8; 0x8],
    /// field_68 (confidence: low, kind: int16, lanes: c-render).
    pub field_68: u16,
    /// Unknown bytes (0x6a..0x70).
    pub _pad_006a: [u8; 0x6],
    /// field_70 (confidence: low, kind: pointer, lanes: c-render).
    pub field_70: Ptr32<u8>,
    /// Unknown bytes (0x74..0x78).
    pub _pad_0074: [u8; 0x4],
    /// field_78 (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_78: u8,
    /// Unknown bytes (0x79..0x80).
    pub _pad_0079: [u8; 0x7],
    /// field_80 (confidence: high, kind: pointer, lanes: c-misc-b,c-render).
    pub field_80: Ptr32<u8>,
    /// Unknown bytes (0x84..0xe0).
    pub _pad_0084: [u8; 0x5c],
    /// field_e0 (confidence: low, kind: pointer, lanes: c-render).
    pub field_e0: Ptr32<u8>,
    /// Unknown bytes (0xe4..0x100).
    pub _pad_00e4: [u8; 0x1c],
    /// field_100 (confidence: low, kind: pointer, lanes: c-render).
    pub field_100: Ptr32<u8>,
    /// Unknown bytes (0x104..0x160).
    pub _pad_0104: [u8; 0x5c],
    /// field_160 (confidence: low, kind: pointer, lanes: c-render).
    pub field_160: Ptr32<u8>,
    /// Unknown bytes (0x164..0x210).
    pub _pad_0164: [u8; 0xac],
    /// field_210 (confidence: low, kind: pointer, lanes: c-render).
    pub field_210: Ptr32<u8>,
    /// field_214 (confidence: low, kind: pointer, lanes: c-render).
    pub field_214: Ptr32<u8>,
    /// Unknown bytes (0x218..0x21c).
    pub _pad_0218: [u8; 0x4],
    /// field_21c (confidence: low, kind: pointer, lanes: c-render).
    pub field_21c: Ptr32<u8>,
    /// field_220 (confidence: low, kind: pointer, lanes: c-render).
    pub field_220: Ptr32<u8>,
    /// field_224 (confidence: low, kind: pointer, lanes: c-render).
    pub field_224: Ptr32<u8>,
    /// field_228 (confidence: low, kind: pointer, lanes: c-render).
    pub field_228: Ptr32<u8>,
    /// field_22c (confidence: low, kind: pointer, lanes: c-render).
    pub field_22c: Ptr32<u8>,
    /// Unknown bytes (0x230..0x238).
    pub _pad_0230: [u8; 0x8],
    /// field_238 (confidence: high, kind: pointer, lanes: c-misc-b,c-render).
    pub field_238: Ptr32<u8>,
    /// field_23c (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_23c: u8,
    /// Unknown bytes (0x23d..0x240).
    pub _pad_023d: [u8; 0x3],
    /// field_240 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_240: Ptr32<u8>,
    /// field_244 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_244: Ptr32<u8>,
    /// field_248 (confidence: high, kind: bool-or-byte, lanes: c-misc-b,c-render).
    pub field_248: u8,
    /// Unknown bytes (0x249..0x254).
    pub _pad_0249: [u8; 0xb],
    /// field_254 (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_254: u8,
    /// Unknown bytes (0x255..0x258).
    pub _pad_0255: [u8; 0x3],
    /// field_258 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_258: Ptr32<u8>,
    /// field_25c (confidence: medium, kind: pointer, lanes: c-render).
    pub field_25c: Ptr32<u8>,
    /// field_260 (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_260: u8,
    /// Unknown bytes (0x261..0x264).
    pub _pad_0261: [u8; 0x3],
    /// field_264 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_264: Ptr32<u8>,
    /// Unknown bytes (0x268..0x26c).
    pub _pad_0268: [u8; 0x4],
    /// field_26c (confidence: medium, kind: pointer, lanes: c-render).
    pub field_26c: Ptr32<u8>,
    /// field_270 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_270: Ptr32<u8>,
    /// field_274 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_274: Ptr32<u8>,
    /// Unknown bytes (0x278..0x294).
    pub _pad_0278: [u8; 0x1c],
    /// field_294 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_294: Ptr32<u8>,
    /// field_298 (confidence: low, kind: pointer, lanes: c-render).
    pub field_298: Ptr32<u8>,
    /// Unknown bytes (0x29c..0x4a0).
    pub _pad_029c: [u8; 0x204],
    /// field_4a0 (confidence: low, kind: pointer, lanes: c-render).
    pub field_4a0: Ptr32<u8>,
    /// field_4a4 (confidence: low, kind: pointer, lanes: c-render).
    pub field_4a4: Ptr32<u8>,
    /// field_4a8 (confidence: low, kind: pointer, lanes: c-render).
    pub field_4a8: Ptr32<u8>,
    /// Unknown bytes (0x4ac..0x4b0).
    pub _pad_04ac: [u8; 0x4],
    /// field_4b0 (confidence: low, kind: pointer, lanes: c-render).
    pub field_4b0: Ptr32<u8>,
    /// field_4b4 (confidence: low, kind: pointer, lanes: c-render).
    pub field_4b4: Ptr32<u8>,
    /// field_4b8 (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_4b8: u8,
    /// Unknown trailing bytes (0x4b9..0x4c0).
    pub _pad_end: [u8; 0x7],
}
assert_size!(RageRmPtfxManager, 0x4c0); // merged size 0x4c0 rounded to 4
assert_offset!(RageRmPtfxManager, field_4, 0x4);
assert_offset!(RageRmPtfxManager, field_20, 0x20);
assert_offset!(RageRmPtfxManager, field_38, 0x38);
assert_offset!(RageRmPtfxManager, field_50, 0x50);
assert_offset!(RageRmPtfxManager, field_58, 0x58);
assert_offset!(RageRmPtfxManager, field_68, 0x68);
assert_offset!(RageRmPtfxManager, field_70, 0x70);
assert_offset!(RageRmPtfxManager, field_78, 0x78);
assert_offset!(RageRmPtfxManager, field_80, 0x80);
assert_offset!(RageRmPtfxManager, field_e0, 0xe0);
assert_offset!(RageRmPtfxManager, field_100, 0x100);
assert_offset!(RageRmPtfxManager, field_160, 0x160);
assert_offset!(RageRmPtfxManager, field_210, 0x210);
assert_offset!(RageRmPtfxManager, field_214, 0x214);
assert_offset!(RageRmPtfxManager, field_21c, 0x21c);
assert_offset!(RageRmPtfxManager, field_220, 0x220);
assert_offset!(RageRmPtfxManager, field_224, 0x224);
assert_offset!(RageRmPtfxManager, field_228, 0x228);
assert_offset!(RageRmPtfxManager, field_22c, 0x22c);
assert_offset!(RageRmPtfxManager, field_238, 0x238);
assert_offset!(RageRmPtfxManager, field_23c, 0x23c);
assert_offset!(RageRmPtfxManager, field_240, 0x240);
assert_offset!(RageRmPtfxManager, field_244, 0x244);
assert_offset!(RageRmPtfxManager, field_248, 0x248);
assert_offset!(RageRmPtfxManager, field_254, 0x254);
assert_offset!(RageRmPtfxManager, field_258, 0x258);
assert_offset!(RageRmPtfxManager, field_25c, 0x25c);
assert_offset!(RageRmPtfxManager, field_260, 0x260);
assert_offset!(RageRmPtfxManager, field_264, 0x264);
assert_offset!(RageRmPtfxManager, field_26c, 0x26c);
assert_offset!(RageRmPtfxManager, field_270, 0x270);
assert_offset!(RageRmPtfxManager, field_274, 0x274);
assert_offset!(RageRmPtfxManager, field_294, 0x294);
assert_offset!(RageRmPtfxManager, field_298, 0x298);
assert_offset!(RageRmPtfxManager, field_4a0, 0x4a0);
assert_offset!(RageRmPtfxManager, field_4a4, 0x4a4);
assert_offset!(RageRmPtfxManager, field_4a8, 0x4a8);
assert_offset!(RageRmPtfxManager, field_4b0, 0x4b0);
assert_offset!(RageRmPtfxManager, field_4b4, 0x4b4);
assert_offset!(RageRmPtfxManager, field_4b8, 0x4b8);

/// Merged layout for `rage::rmPtfxShaderVar`.
///
/// Size: 0x2c (low). Bases: rage::datBase@0x0.
/// Lanes: c-render, via:rage::rmPtfxShaderVar_Float, via:rage::rmPtfxShaderVar_Float2, via:rage::rmPtfxShaderVar_Float3, via:rage::rmPtfxShaderVar_Float4, via:rage::rmPtfxShaderVar_Int, via:rage::rmPtfxShaderVar_Texture.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageRmPtfxShaderVar {
    /// Unknown bytes (0x0..0x4).
    pub _pad_0000: [u8; 0x4],
    /// field_4 (confidence: high, kind: pointer, lanes: c-render).
    pub field_4: Ptr32<u8>,
    /// Unknown bytes (0x8..0x10).
    pub _pad_0008: [u8; 0x8],
    /// field_10 (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_10: u8,
    /// field_11 (confidence: high, kind: bool-or-byte, lanes: c-render).
    pub field_11: u8,
    /// field_12 (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_12: u8,
    /// Unknown bytes (0x13..0x20).
    pub _pad_0013: [u8; 0xd],
    /// field_20 (confidence: high, kind: pointer, lanes: c-render,via:rage::rmPtfxShaderVar_Float,via:rage::rmPtfxShaderVar_Float2,via:rage::rmPtfxShaderVar_Float3,via:rage::rmPtfxShaderVar_Float4,via:rage::rmPtfxShaderVar_Int,via:rage::rmPtfxShaderVar_Texture moved from siblings:rage::rmPtfxShaderVar_Float,rage::rmPtfxShaderVar_Float2,rage::rmPtfxShaderVar_Float3,rage::rmPtfxShaderVar_Float4,rage::rmPtfxShaderVar_Int,rage::rmPtfxShaderVar_Texture).
    pub field_20: Ptr32<u8>,
    /// Unknown bytes (0x24..0x28).
    pub _pad_0024: [u8; 0x4],
    /// field_28 (confidence: low, kind: pointer, lanes: c-render).
    pub field_28: Ptr32<u8>,
}
assert_size!(RageRmPtfxShaderVar, 0x2c); // merged size 0x2c rounded to 4
assert_offset!(RageRmPtfxShaderVar, field_4, 0x4);
assert_offset!(RageRmPtfxShaderVar, field_10, 0x10);
assert_offset!(RageRmPtfxShaderVar, field_11, 0x11);
assert_offset!(RageRmPtfxShaderVar, field_12, 0x12);
assert_offset!(RageRmPtfxShaderVar, field_20, 0x20);
assert_offset!(RageRmPtfxShaderVar, field_28, 0x28);

/// Merged layout for `rmcInstanceWheelData`.
///
/// Size: 0x70 (low). Bases: rage::rmcInstanceDataBase@0x0.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RmcInstanceWheelData {
    /// vftable (confidence: high, kind: vtable_ptr, lanes: c-misc-b).
    pub vftable: Ptr32<()>,
    /// Unknown bytes (0x4..0x10).
    pub _pad_0004: [u8; 0xc],
    /// field_10 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_10: u32,
    /// Unknown bytes (0x14..0x50).
    pub _pad_0014: [u8; 0x3c],
    /// field_50 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_50: u32,
    /// field_54 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_54: u32,
    /// field_58 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_58: u32,
    /// field_5c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_5c: u32,
    /// field_60 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_60: u32,
    /// field_64 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_64: u32,
    /// field_68 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_68: u32,
    /// field_6c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_6c: u32,
}
assert_size!(RmcInstanceWheelData, 0x70); // merged size 0x70 rounded to 4
assert_offset!(RmcInstanceWheelData, vftable, 0x0);
assert_offset!(RmcInstanceWheelData, field_10, 0x10);
assert_offset!(RmcInstanceWheelData, field_50, 0x50);
assert_offset!(RmcInstanceWheelData, field_54, 0x54);
assert_offset!(RmcInstanceWheelData, field_58, 0x58);
assert_offset!(RmcInstanceWheelData, field_5c, 0x5c);
assert_offset!(RmcInstanceWheelData, field_60, 0x60);
assert_offset!(RmcInstanceWheelData, field_64, 0x64);
assert_offset!(RmcInstanceWheelData, field_68, 0x68);
assert_offset!(RmcInstanceWheelData, field_6c, 0x6c);
