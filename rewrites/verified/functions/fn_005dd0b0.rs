// original: 0x005dd0b0 CTaskSimpleAssessInjuredPed_ctor

/// Construct an assess-injured-ped task around the argument.
///
/// Base-constructs `this`, sets bit 0 and clears bit 1 of the flag
/// byte at `+0x14`, stamps `VTABLE`, stores -1 at `+0x18` and the
/// argument at `+0x1c`, zeroes `+0x20..+0x32`, constructs the member at
/// `+0x34`, reference-counts a non-null argument, and returns `this`.
///
/// Original: 0x005dd0b0 (thiscall).
lf_checker_rt::export!(thiscall, rw_005dd0b0(this: u32, arg: u32) -> u32 {
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
        const VTABLE: u32 = 0xfe0c5c;
        const BASE_CTOR: u32 = 1;
        const SUB_CTOR: u32 = 2;
        const ADDREF: u32 = 3;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        let b = rd8(this + 0x14);
        wr8(this + 0x14, (b & 0xfd) | 1);
        wr32(this, lf_checker_rt::relocated(VTABLE));
        wr32(this + 0x18, 0xffffffff);
        wr32(this + 0x1c, arg);
        wr32(this + 0x20, 0);
        wr32(this + 0x24, 0);
        wr32(this + 0x28, 0);
        wr32(this + 0x2c, 0);
        wr16(this + 0x30, 0);
        lf_checker_rt::callee_thiscall!(SUB_CTOR, u32, this + 0x34);
        if arg != 0 {
            lf_checker_rt::callee_thiscall!(ADDREF, u32, arg, this + 0x1c);
        }
        wr8(this + 0x30, 0);
        this
    }
});
