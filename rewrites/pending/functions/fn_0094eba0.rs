// original: 0x0094eba0 guarded_flag_store
/// Store a flag byte at a validated index.
///
/// Writes the low byte of `value` to `obj + 0xAF20 + index` when `index` is
/// non-negative (signed); a negative index stores nothing.
export!(thiscall, rw_0094eba0(obj: *mut u8, index: u32, value: u32) -> u32 {
    unsafe {
        if (index as i32) >= 0 {
            *obj.add(0xAF20).add(index as usize) = value as u8;
        }
        0
    }
});
