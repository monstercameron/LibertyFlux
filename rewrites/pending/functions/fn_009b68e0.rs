// original: 0x009B68E0 CViewportScripted deleting destructor (symbols: CViewportScripted::~CViewportScripted__deleting)
/// Destroy the scripted viewport and free it when the flags ask.
///
/// Writes the base-class table address over the object, runs the base
/// destructor, and when bit 0 of `flags` is set releases the object memory.
/// Returns the object pointer in all cases. thiscall, object in ecx.
lf_checker_rt::export!(thiscall, rw_009B68E0(this: u32, flags: u32) -> u32 {
    unsafe {
        const BASE_TABLE: u32 = 0x00E93F40;
        const FREE_FLAG: u32 = 1;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(BASE_TABLE));
        lf_checker_rt::callee_thiscall!(1, u32, this);
        if flags & FREE_FLAG != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(2, u32, this);
        }
        this
    }
});
