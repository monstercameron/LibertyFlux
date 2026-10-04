// original: 0x0097B2B0 audio_table_float_at
/// Load one float from the indexed coefficient table (x87 return).
///
/// Returns the table entry at `base + index * 4`. thiscall(obj, index).
export!(thiscall, rw_s103_97b2b0(obj: *const u8, idx: u32) -> f32 {
    unsafe {
        *((((obj as usize).wrapping_add((idx as usize).wrapping_mul(4))).wrapping_add(0xC4))
            as *const f32)
    }
});
