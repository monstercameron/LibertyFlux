// original: 0x00925E70 input_shifted_limit
/// Compute `min(0x40 << shift, 0x400)` gated on two globals.
///
/// Returns 0 unless both `0x01160EAC` and `0x01160EA8` are positive. The
/// shift uses x86 `shl` semantics (count masked to 5 bits) via `wrapping_shl`.
export!(cdecl, rw_00925E70() -> u32 {
    unsafe {
        let shift = *global::<i32>(0x1160EAC);
        if shift <= 0 {
            return 0;
        }
        if *global::<i32>(0x1160EA8) <= 0 {
            return 0;
        }
        let v = 0x40u32.wrapping_shl(shift as u32);
        if v > 0x400 {
            0x400
        } else {
            v
        }
    }
});
