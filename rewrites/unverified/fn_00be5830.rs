// original: 0x00BE5830 CTaskSimpleMoveSwim::vf20

/// Copy the task's three movement words at offsets 0x20, 0x24, and 0x28 to
/// the caller's output buffer. Return that same buffer pointer. The output
/// pointer is a thiscall stack argument, and the caller retains its ownership.
lf_checker_rt::export!(thiscall, rw_00be5830(this: u32, output: u32) -> u32 {
    unsafe {
        const FIELD_OFFSETS: [u32; 3] = [0x20, 0x24, 0x28];
        for (index, offset) in FIELD_OFFSETS.into_iter().enumerate() {
            let value = ((this + offset) as *const u32).read_unaligned();
            ((output as *mut u32).add(index)).write_unaligned(value);
        }
        output
    }
});
