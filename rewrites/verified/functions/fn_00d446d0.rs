// original: 0x00d446d0 task_state_set_kind_and_arg (proposed)

/// Set a task object's kind tag and its argument slot.
///
/// `this` points to the task object: dword at `+0x78` is a kind tag, dword at
/// `+0x1c` carries one argument word. Writes tag value 2 and the incoming
/// argument, and returns the argument (the original leaves it in eax as a
/// leftover of loading it first; reproduced for bit-identity).
///
/// Original: thiscall, one stack word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00d446d0(this: u32, arg: u32) -> u32 {
    unsafe {
        const KIND_SLOT: u32 = 0x78;
        const ARG_SLOT: u32 = 0x1c;
        const KIND_VALUE: u32 = 2;
        ((this + KIND_SLOT) as *mut u32).write_unaligned(KIND_VALUE);
        ((this + ARG_SLOT) as *mut u32).write_unaligned(arg);
        arg
    }
});
