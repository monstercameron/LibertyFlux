// original: 0x005dd4e0 CTaskComplexTreatAccident_ctor

/// Construct a treat-accident task holding the argument at +0x14.
///
/// Base-constructs `this`, stamps `VTABLE`, stores the argument at
/// `+0x14`, reference-counts it when non-null, and returns `this`.
///
/// Original: 0x005dd4e0 (thiscall).
lf_checker_rt::export!(thiscall, rw_005dd4e0(this: u32, arg: u32) -> u32 {
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
        const VTABLE: u32 = 0xfe0dd4;
        const BASE_CTOR: u32 = 1;
        const ADDREF: u32 = 2;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        wr32(this, lf_checker_rt::relocated(VTABLE));
        wr32(this + 0x14, arg);
        if arg != 0 {
            lf_checker_rt::callee_thiscall!(ADDREF, u32, arg, this + 0x14);
        }
        this
    }
});
