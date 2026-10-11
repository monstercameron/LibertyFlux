// original: 0x00c45380 CCamFinal::CCamFinal
/// Initialize a final-camera object by calling the camera base initializer,
/// installing its vtable word, then invoking the second initializer. Returns
/// the original object pointer. Both helpers are thiscall with no stack args.
lf_checker_rt::export!(thiscall, rw_00c45380(this: u32) -> u32 {
    const BASE_INITIALIZER: u32 = 1;
    const FINAL_INITIALIZER: u32 = 2;
    const VTABLE_FILE_VA: u32 = 0x00ec8fa0;
    lf_checker_rt::callee_thiscall!(BASE_INITIALIZER, u32, this);
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_FILE_VA));
    }
    lf_checker_rt::callee_thiscall!(FINAL_INITIALIZER, u32, this);
    this
});
