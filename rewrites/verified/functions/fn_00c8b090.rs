// original: 0x00c8b090 audio_voices_mute (proposed)
///
/// Zeroes the flag word at +0x14 of each of the first `n` voice slots,
/// where `n` is the SIGNED half-word at `this+8` (zero or negative mutes
/// nothing) and the slot array pointer is at `this+4`; slots are 0x20
/// bytes apart. Returns the sign-extended count when the loop runs, else 0
/// (the original's eax residue, kept exact). Thiscall, no stack arguments.

lf_checker_rt::export!(thiscall, rw_00c8b090(this: u32) -> u32 {
    unsafe {
    #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
    #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
    #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        const SLOT: u32 = 0x20;
        const FLAG_OFF: u32 = 0x14;
        let n = rd16(this.wrapping_add(8)) as i16 as i32;
        if n <= 0 {
            return 0;
        }
        let arr = rd32(this.wrapping_add(4));
        let mut i: i32 = 0;
        while i < n {
            wr32(arr.wrapping_add((i as u32).wrapping_mul(SLOT)).wrapping_add(FLAG_OFF), 0);
            i += 1;
        }
        n as u32
    }
});
