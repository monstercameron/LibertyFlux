// original: 0x00ca2b90 CEventDamage::~CEventDamage

/// Destroy a damage event: release the handle and the payload, run base.
///
/// Installs the vtable, releases the subject handle at `+0x18` (callee 1)
/// when non-null, releases the payload at `+0x34` through its vtable slot 0
/// (callee 2) when non-null, installs the sub vtable, then tail-calls the
/// base destructor (callee 3) whose answer is returned.
///
/// Original: 0x00ca2b90 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00ca2b90(this: u32) -> u32 {
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe {
        const VT_MAIN: u32 = 0x00ed_7424;
        const VT_SUB: u32 = 0x00ed_7418;
        wr32(this, lf_checker_rt::relocated(VT_MAIN));
        let h = rd32(this + 0x18);
        if h != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, h, this + 0x18);
        }
        let o = rd32(this + 0x34);
        if o != 0 {
            let slot = rd32(rd32(o));
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            f(o, 1);
        }
        wr32(this + 0x24, lf_checker_rt::relocated(VT_SUB));
        lf_checker_rt::callee_thiscall!(3, u32, this)
    }
});
