// original: 0x00b2b550 obj_flag_set

/// Sets or clears one mode bit on an object from a boolean byte.
///
/// `this` points to the object; only the low byte of the stack word `b` is
/// read. Writes 0x200000 to the flag word at `this + 0xF90` when that byte is
/// nonzero, else 0, and returns the value written. Thiscall: object in ECX,
/// one stack word, callee pops it (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_00b2b550(this: u32, b: u32) -> u32 {
    unsafe {
        const FLAG_WORD: u32 = 0x0F90;
        const MODE_BIT: u32 = 0x0020_0000;
        let v = if (b as u8) != 0 { MODE_BIT } else { 0 };
        ((this + FLAG_WORD) as *mut u32).write_unaligned(v);
        v
    }
});
