// original: 0x005ddbe0 TaskFE0E2C_ctor_vec

/// Construct a vtable-FE0E2C task from five arguments including a vector.
///
/// Base-constructs `this`, stores the two words and two low bytes at
/// `+0x14`/`+0x18`/`+0x1c`/`+0x42`, stamps `VTABLE`, copies three words
/// from the vector argument to `+0x30`, zeroes the rest (`+0x54` gets
/// -1), then reference-counts each non-null stored word and clears its
/// slot; returns `this`. The check of `+0x20` always sees zero.
///
/// Original: 0x005ddbe0 (thiscall).
lf_checker_rt::export!(thiscall, rw_005ddbe0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        const VTABLE: u32 = 0xfe0e2c;
        const BASE_CTOR: u32 = 1;
        const ADDREF: u32 = 2;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        wr32(this + 0x14, a0);
        wr32(this + 0x18, a1);
        wr8(this + 0x1c, a2 as u8);
        wr32(this + 0x20, 0);
        wr32(this, lf_checker_rt::relocated(VTABLE));
        wr32(this + 0x30, rd32(a3));
        wr32(this + 0x34, rd32(a3 + 4));
        wr32(this + 0x38, rd32(a3 + 8));
        wr16(this + 0x40, 0);
        wr8(this + 0x42, a4 as u8);
        wr32(this + 0x44, 0);
        wr32(this + 0x48, 0);
        wr32(this + 0x4c, 0);
        wr16(this + 0x50, 0);
        wr32(this + 0x54, 0xffffffff);
        // The slot at +0x20 was just zeroed, so this call never fires.
        if rd32(this + 0x20) != 0 {
            let z = rd32(this + 0x20);
            lf_checker_rt::callee_thiscall!(ADDREF, u32, z, this + 0x20);
            wr32(this + 0x20, 0);
        }
        let c0 = rd32(this + 0x14);
        if c0 != 0 {
            lf_checker_rt::callee_thiscall!(ADDREF, u32, c0, this + 0x14);
            wr32(this + 0x14, 0);
        }
        let c1 = rd32(this + 0x18);
        if c1 != 0 {
            lf_checker_rt::callee_thiscall!(ADDREF, u32, c1, this + 0x18);
            wr32(this + 0x18, 0);
        }
        this
    }
});
