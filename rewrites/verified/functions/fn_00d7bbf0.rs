// original: 0x00D7BBF0 index_to_scale_factor (proposed)

/// Map a small index to a scale factor, returned in ST0.
///
/// `idx` selects: -1 or 0 gives 0.5, 1 gives 1.0, 2 gives 1.6
/// (0x3fcccccd), 3 gives 2.2 (0x400ccccd), anything else gives 1.0. The
/// original dispatches through a jump table; the result is loaded with
/// `fld`, so the return channel is ST0. Cdecl, one stack word.
use lf_checker_rt::export;

export!(cdecl, rw_00d7bbf0(idx: u32) -> f32 {
    match idx as i32 {
        -1 | 0 => f32::from_bits(0x3f00_0000),
        1 => f32::from_bits(0x3f80_0000),
        2 => f32::from_bits(0x3fcc_cccd),
        3 => f32::from_bits(0x400c_cccd),
        _ => f32::from_bits(0x3f80_0000),
    }
});
