// original: 0x00BDF9B0 CTaskSimpleSitDown::~CTaskSimpleSitDown

/// Detaches the registered animation and timing-node objects when they are present, clears both task fields, replaces the vtable, and calls the base simple-task destructor.
lf_checker_rt::export!(thiscall, rw_00bdf9b0(this: u32) -> u32 {
    unsafe {
        const REGISTER_ANIMATION: u32 = 1;
        const DETACH_TIMING_NODE: u32 = 2;
        const BASE_TASK_DTOR: u32 = 3;
        const TASK_VTABLE: u32 = 0x00EB8D7C;
        const ANIMATION_FIELD: u32 = 0x60;
        const TIMING_NODE_FIELD: u32 = 0x64;

        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(TASK_VTABLE));
        let animation_field = this.wrapping_add(ANIMATION_FIELD);
        let animation = (animation_field as *const u32).read_unaligned();
        if animation != 0 {
            let _ = lf_checker_rt::callee_thiscall!(REGISTER_ANIMATION, u32, animation, this);
            (animation_field as *mut u32).write_unaligned(0);
        }

        let node_field = this.wrapping_add(TIMING_NODE_FIELD);
        let node = (node_field as *const u32).read_unaligned();
        if node != 0 {
            let _ = lf_checker_rt::callee_thiscall!(DETACH_TIMING_NODE, u32, node, node_field);
            (node_field as *mut u32).write_unaligned(0);
        }
        lf_checker_rt::callee_thiscall!(BASE_TASK_DTOR, u32, this)
    }
});
