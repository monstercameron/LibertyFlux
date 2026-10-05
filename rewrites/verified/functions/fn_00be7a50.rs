// original: 0x00be7a50 ped_subobject_reset (proposed)

/// Reset the ped sub-object at `+0x2b0`, propagating the callee's result.
///
/// Takes the ped pointer (its only stack word), addresses the embedded
/// sub-object 0x2b0 bytes in and invokes callee 1 (thiscall, no stack words)
/// on it. Returns whatever the callee returned. Ignores `ecx` on entry.
///
/// Original: stdcall, one stack word, the callee pops 4 bytes, returns `eax`.
lf_checker_rt::export!(stdcall, rw_00be7a50(ped: u32) -> u32 {
    unsafe {
        const SUBOBJECT: u32 = 0x2b0;
        const RESETTER: u32 = 1;

        lf_checker_rt::callee_thiscall!(RESETTER, u32, ped.wrapping_add(SUBOBJECT))
    }
});
