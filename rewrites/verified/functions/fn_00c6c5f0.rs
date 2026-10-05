// original: 0x00c6c5f0 anim_dict_init (proposed)

/// Initialise a dictionary object in place and stamp its slot bytes.
///
/// `this` receives: `+0xC` the multiplier, `+0` the first allocation of
/// `count * mult` bytes, `+4` the second allocation of `count` bytes,
/// `+8` the count, `+0x10` set to -1, `+0x14` cleared, `+0x18` set to 1.
/// The first `count` bytes of the second allocation are then stamped to
/// `(old bit 0) | 0x81`: bit 7 set, low six bits cleared, bit 0 kept.
/// The middle stack word is never read. Returns `this`.
///
/// Original: thiscall with three stack words, two calls to one callee.
lf_checker_rt::export!(thiscall, rw_00c6c5f0(this: u32, count: u32, _unused: u32, mult: u32) -> u32 {
    unsafe {
        const MULT_OFF: u32 = 0xC;
        const MAIN_OFF: u32 = 0x0;
        const SLOTS_OFF: u32 = 0x4;
        const COUNT_OFF: u32 = 0x8;
        const HEAD_OFF: u32 = 0x10;
        const TAIL_OFF: u32 = 0x14;
        const READY_OFF: u32 = 0x18;
        const ALLOC: u32 = 1;

        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        wr32(this.wrapping_add(MULT_OFF), mult);
        let main: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, count.wrapping_mul(mult));
        wr32(this.wrapping_add(MAIN_OFF), main);
        let slots: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, count);
        wr32(this.wrapping_add(SLOTS_OFF), slots);
        unsafe { ((this.wrapping_add(READY_OFF)) as *mut u8).write(1) };
        wr32(this.wrapping_add(COUNT_OFF), count);
        wr32(this.wrapping_add(HEAD_OFF), 0xFFFF_FFFF);
        wr32(this.wrapping_add(TAIL_OFF), 0);
        if (count as i32) > 0 {
            let mut i = 0u32;
            while i < count {
                let p = slots.wrapping_add(i);
                let v = unsafe { (p as *const u8).read() };
                unsafe { (p as *mut u8).write(v & 1 | 0x81) };
                i = i.wrapping_add(1);
            }
        }
        this
    }
});
