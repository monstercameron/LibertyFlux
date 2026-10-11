// original: 0x00DA6D80 CTaskComplexEscapeBlast::vf1

/// Ask the global task manager to create an escape-blast task. A missing
/// manager returns zero. On success, forward the task identifier, pointer to
/// this task's embedded point, its two coordinate words, and its selector
/// byte (zero-extended) to the factory in that order. Float values are passed
/// as their stored 32-bit words so their payloads are preserved exactly.
lf_checker_rt::export!(thiscall, rw_00DA6D80(this: u32) -> u32 {
    unsafe {
        const MANAGER_GLOBAL_VA: u32 = 0x0167E2A0;
        const TASK_TYPE: u32 = 1;
        const TASK_MANAGER_LOOKUP: u32 = 1;
        const TASK_FACTORY: u32 = 2;
        const TASK_IDENTIFIER: u32 = 0x14;
        const POINT: u32 = 0x20;
        const X_COORDINATE: u32 = 0x30;
        const Y_COORDINATE: u32 = 0x34;
        const SELECTOR: u32 = 0x38;

        let manager_context = lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read_unaligned();
        let manager = lf_checker_rt::callee_thiscall!(TASK_MANAGER_LOOKUP, u32, manager_context);
        if manager == 0 {
            return 0;
        }

        let task_identifier = (this.wrapping_add(TASK_IDENTIFIER) as *const u32).read_unaligned();
        let point = this.wrapping_add(POINT);
        let x_bits = (this.wrapping_add(X_COORDINATE) as *const u32).read_unaligned();
        let selector = (this.wrapping_add(SELECTOR) as *const u8).read() as u32;
        let y_bits = (this.wrapping_add(Y_COORDINATE) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(
            TASK_FACTORY, u32, manager, task_identifier, point, x_bits, selector, y_bits
        )
    }
});
