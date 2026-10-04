// original: 0x00ca2bd0 CEventDeadPed::~CEventDeadPed

/// Destroy a dead-ped event: release the handle, run the base destructor.
///
/// Installs the vtable, releases the subject handle at `+0x18` (callee 1)
/// when non-null, then tail-calls the base destructor (callee 2) whose
/// answer is returned.
///
/// Original: 0x00ca2bd0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00ca2bd0(this: u32) -> u32 {
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe {
        const VT_MAIN: u32 = 0x00ed_74dc;
        wr32(this, lf_checker_rt::relocated(VT_MAIN));
        let h = rd32(this + 0x18);
        if h != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, h, this + 0x18);
        }
        lf_checker_rt::callee_thiscall!(2, u32, this)
    }
});
