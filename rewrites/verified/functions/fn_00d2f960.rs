// original: 0x00D2F960 path_node_count (proposed)

/// Return the first 32-bit word of the path record referenced by the task's
/// pointer field at offset 0x70. A null pointer returns zero. The method is a
/// thiscall leaf with no stack arguments; its result is the EAX word.
lf_checker_rt::export!(thiscall, rw_00D2F960(this: u32) -> u32 {
    const PATH_POINTER: u32 = 0x70;
    let path = unsafe { crate::read_u32(this.wrapping_add(PATH_POINTER)) };
    if path == 0 {
        0
    } else {
        unsafe { crate::read_u32(path) }
    }
});
