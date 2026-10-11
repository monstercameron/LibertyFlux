// original: 0x00D39950 CDummyTask_Wander::vf0

/// CDummyTask_Wander::vf0 calls its base destructor, then deletes the object when bit 0 of `delete_flag` is set. The allocator service pointer is read from the documented global slot. ECX holds `this`; one stack word is callee-cleaned; the return value is the original object pointer.
lf_checker_rt::export!(thiscall, rw_00d39950(this: u32, delete_flag: u32) -> u32 {
    unsafe {
        const DELETE_FLAG: u32 = 0x01;
        const ALLOCATOR_GLOBAL: u32 = 0x0171FAF4;
        const BASE_DESTRUCTOR: u32 = 1;
        const DELETE_OBJECT: u32 = 2;
        let _ = lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this);
        if delete_flag & DELETE_FLAG != 0 {
            let allocator = lf_checker_rt::global::<u32>(ALLOCATOR_GLOBAL).read_unaligned();
            let _ = lf_checker_rt::callee_thiscall!(DELETE_OBJECT, u32, allocator, this);
        }
        this
    }
});
