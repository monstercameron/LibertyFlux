// original: 0x00CBCA20 CPedMoveBlendBase::vf0
/// Set the object's base blend vtable. When option bit zero is set, call
/// the registered blend manager with the object as its single stack
/// argument. The method is thiscall, returns the object pointer, and reads
/// the manager pointer from the global slot at `0x018B6F18`.
lf_checker_rt::export!(thiscall, rw_00cbca20(this: *mut u8, options: u32) -> u32 {
    const BASE_VTABLE: u32 = 0x00EAC454;
    const GLOBAL_MANAGER: u32 = 0x018B6F18;
    const UPDATE_MANAGER_OPTION: u32 = 1;

    unsafe {
        this.cast::<u32>()
            .write_unaligned(lf_checker_rt::relocated(BASE_VTABLE));
    }
    if options & UPDATE_MANAGER_OPTION != 0 {
        let manager = unsafe { lf_checker_rt::global::<u32>(GLOBAL_MANAGER).read_unaligned() };
        let _ = lf_checker_rt::callee_thiscall!(1, u32, manager, this as u32);
    }
    this as u32
});
