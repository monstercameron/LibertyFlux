// original: 0x00BDFA00 CTaskSimpleStandUp::~CTaskSimpleStandUp

/// Replaces the task vtable, unregisters a non-null animation pointer, clears
/// that field, and tail-calls the base simple-task destructor. The thiscall
/// receiver is in ECX and there are no stack arguments.
lf_checker_rt::export!(thiscall, rw_bdfa00(this: u32) -> u32 {
    unsafe {
        const UNREGISTER_ANIMATION: u32 = 1;
        const BASE_TASK_DTOR: u32 = 2;
        const TASK_VTABLE: u32 = 0x00EB8E34;
        const ANIMATION_FIELD: u32 = 0x14;

        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(TASK_VTABLE));
        let animation_field = this.wrapping_add(ANIMATION_FIELD);
        let animation = (animation_field as *const u32).read_unaligned();
        if animation != 0 {
            let _ = lf_checker_rt::callee_thiscall!(UNREGISTER_ANIMATION, u32, animation, this);
            (animation_field as *mut u32).write_unaligned(0);
        }
        lf_checker_rt::callee_thiscall!(BASE_TASK_DTOR, u32, this)
    }
});
