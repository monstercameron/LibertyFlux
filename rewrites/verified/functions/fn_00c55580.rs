// original: 0x00c55580 CVehicleScanner::destroy_member_then_base
/// Stamp the scanner vtable, pass the reference slot at +0x4 to the shared slot cleanup, then forward `this` and the option word to the common base teardown. The thiscall consumes one stack word and returns the base teardown result.
lf_checker_rt::export!(thiscall, rw_00c55580(this: u32, options: u32) -> u32 {
    const FIELD_POINTER: u32 = 0x04;
    const CLEAN_MEMBER_SLOT: u32 = 1;
    const SHARED_TEARDOWN: u32 = 2;
    unsafe {
        core::ptr::write_unaligned(this as *mut u32, lf_checker_rt::relocated(0x00ecb020));
        let field = this.wrapping_add(FIELD_POINTER);
        let _: u32 = lf_checker_rt::callee_cdecl!(CLEAN_MEMBER_SLOT, u32, field);
    }
    lf_checker_rt::callee_thiscall!(SHARED_TEARDOWN, u32, this, options)
});
