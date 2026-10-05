// original: 0x00AF8590 veh_handle_copy_into (proposed)

/// Copy one embedded handle over another within the same object.
///
/// Runs the handle copy (callee 1) with the destination at `this + 0xA0`
/// and the source at `this + 0x74`. Nothing is returned.
///
/// Original: 0x00AF8590 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00AF8590(this: u32) -> u32 {
    unsafe {
        const COPY: u32 = 1;
        const SRC: u32 = 0x74;
        const DST: u32 = 0xA0;
        lf_checker_rt::callee_thiscall!(COPY, u32, this + DST, this + SRC);
        0
    }
});
