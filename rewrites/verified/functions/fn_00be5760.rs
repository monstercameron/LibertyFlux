// original: 0x00BE5760 task_type_tag_after_lazy_init

/// Initialize the task type field when its unsigned value is all bits set,
/// then map values 0 and 1 to one tag, 2 and 3 to their respective tags, and
/// values above 3 to the invalid sentinel. A state byte selects the alternate
/// tag in each valid group. The initializer writes the type field through the
/// task pointer and is intercepted with an explicit out-parameter script. The
/// function also pops one ignored 32-bit stack word before returning.
lf_checker_rt::export!(thiscall, rw_00be5760(this: u32, _unused: u32) -> u32 {
    unsafe {
        const TASK_TYPE_FIELD: u32 = 0x38;
        const TASK_STATE: u32 = 0x85;
        let mut index = ((this + TASK_TYPE_FIELD) as *const u32).read_unaligned();
        if index == u32::MAX {
            let _initialized = lf_checker_rt::callee_thiscall!(1, u32, this, 1);
            index = ((this + TASK_TYPE_FIELD) as *const u32).read_unaligned();
        }
        let has_state = ((this + TASK_STATE) as *const u8).read() != 0;
        match index {
            0 | 1 => if has_state { 0xa4 } else { 0x9d },
            2 => if has_state { 0xa6 } else { 0xa1 },
            3 => if has_state { 0xa5 } else { 0x9f },
            _ => u32::MAX,
        }
    }
});
