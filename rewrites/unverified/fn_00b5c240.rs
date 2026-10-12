// original: 0x00b5c240 add_saturate_word (proposed)

/// Add the stack argument to the counter word at `+0x60`, saturating into
/// the range 0..=0x61a8, and store the result back. Only the low 16 bits
/// of the sum matter: if either input word already equals 0x61a8 the result
/// is 0x61a8; otherwise a sum whose low word has the sign bit set yields 0
/// and any other sum is clamped to 0x61a8 from above. Returns the stored
/// value.
lf_checker_rt::export!(thiscall, rw_00b5c240(this: u32, add: u32) -> u32 {
    unsafe {
        const COUNTER: u32 = 0x60;
        const SAT: u32 = 0x61a8;
        let p = this.wrapping_add(COUNTER) as *mut u32;
        let cur = p.read_unaligned();
        let out = if cur == SAT || add == SAT {
            SAT
        } else {
            let s = cur.wrapping_add(add) & 0xffff;
            if s & 0x8000 != 0 {
                0
            } else if s > SAT {
                SAT
            } else {
                s
            }
        };
        p.write_unaligned(out);
        out
    }
});
