// original: 0x00A7F470 CTargetTaskInfo::vf6

/// Serialize the 16-bit task field at `this + 0x18` and the task-info base.
/// The field is zero-extended before the first stream call, which also
/// receives a zero mode word. The second call serializes the base using the
/// same stream pointer. The function returns the bitwise OR of both low
/// result bytes. This is a 32-bit thiscall method with one stack argument.
lf_checker_rt::export!(thiscall, rw_00a7f470(this: u32, stream: u32) -> u32 {
    unsafe {
        const TASK_KIND: u32 = 0x18;
        const WRITE_TASK_KIND: u32 = 1;
        const WRITE_BASE: u32 = 2;
        const MODE: u32 = 0;

        let task_kind = (this.wrapping_add(TASK_KIND) as *const u16).read_unaligned() as u32;
        let value_result = lf_checker_rt::callee_thiscall!(
            WRITE_TASK_KIND,
            u32,
            stream,
            task_kind,
            MODE
        );
        let base_result = lf_checker_rt::callee_thiscall!(WRITE_BASE, u32, this, stream);
        (value_result & 0xff) | (base_result & 0xff)
    }
});
