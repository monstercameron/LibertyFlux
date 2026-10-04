// original: 0x009fbad0 select_hashed_slot
/// Picks one of `count` 32-byte records by a hash and copies it to the
/// output slots at `this + 0x104`.
///
/// Clears the 16 output bytes, then returns 0 when `count` (the word at
/// `this + 0x100`) is 0. Otherwise it folds four gathered bytes with the
/// shared step's answer, mixes with a multiply/xor-shift hash, reduces
/// modulo `count`, and copies the selected 16 bytes. Returns 1. The second
/// argument of the gather call is an uninitialised register word in the
/// original; the rewrite passes 0 and the contract does not compare it.
export!(thiscall, rw_rs227_009fbad0(this_ptr: u32) -> u8 {
    /// Multiplier of the index hash.
    const HASH_MUL: u32 = 0x2323_0559;
    unsafe {
        ((this_ptr + 0x104) as *mut u32).write_unaligned(0);
        ((this_ptr + 0x108) as *mut u32).write_unaligned(0);
        ((this_ptr + 0x10C) as *mut u32).write_unaligned(0);
        ((this_ptr + 0x110) as *mut u32).write_unaligned(0);
        let count = *((this_ptr + 0x100) as *const u32);
        if count == 0 {
            return 0;
        }
        let mut buf = [0u8; 4];
        callee_cdecl!(0, u32, buf.as_mut_ptr() as u32, 0);
        let gathered = ((buf[0] as u32) << 24)
            | ((buf[1] as u32) << 16)
            | ((buf[2] as u32) << 8)
            | (buf[3] as u32);
        let x = callee_cdecl!(1, u32,) ^ gathered;
        let spread = x.wrapping_add((x == 0) as u32).wrapping_mul(HASH_MUL);
        let h = (x ^ x.rotate_left(16)).wrapping_sub(spread) & 0x7FFF_FFFF;
        let rem = (h as i32).wrapping_rem(count as i32);
        let src = this_ptr.wrapping_add(rem.wrapping_shl(5) as u32);
        let w0 = (src as *const u64).read_unaligned();
        let w1 = ((src + 8) as *const u64).read_unaligned();
        ((this_ptr + 0x104) as *mut u64).write_unaligned(w0);
        ((this_ptr + 0x10C) as *mut u64).write_unaligned(w1);
        1
    }
});
