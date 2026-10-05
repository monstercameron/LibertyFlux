// original: 0x0093E6A0 stream_publish_scene (proposed)

/// Publish a scene update through a chain of subsystem callees.
///
/// Calls the two slot publishers with (`ptr`, `level`, ...) words, then
/// a chain of engine callees forwarding `ptr`, `level` and the mode
/// words; two calls forward the caller's entry `esi` bits as a float
/// word (unreadable from Rust, so those two call arguments are skipped
/// and zero is passed). When the low byte of `flagword` is nonzero, two
/// extra zero-argument calls run between the seventh and eighth engine
/// calls. The `this` pointers are constants selected by the branch.
///
/// Original: 0x0093E6A0 (cdecl, six stack words; twelve direct callees).
lf_checker_rt::export!(cdecl, rw_0093e6a0(ptr: u32, level: u32, flagword: u32, w3: u32, w4: u32, w5: u32) -> u32 {
    const THIS_A_FILE: u32 = 0x12E2420;
    const THIS_B_FILE: u32 = 0x1668DB0;
    unsafe {
        lf_checker_rt::callee_cdecl!(1, u32, ptr, level);
        lf_checker_rt::callee_cdecl!(2, u32, ptr, level, w4, 1u32, 0u32);
        lf_checker_rt::callee_cdecl!(3, u32, ptr, 0u32);
        lf_checker_rt::callee_cdecl!(4, u32, ptr, level, w3, w5, 0u32);
        let this_a = lf_checker_rt::relocated(THIS_A_FILE);
        lf_checker_rt::callee_thiscall!(5, u32, this_a, ptr, 0u32);
        lf_checker_rt::callee_cdecl!(6, u32, ptr, level);
        lf_checker_rt::callee_cdecl!(7, u32, 0xFFFF_FFFFu32, ptr);
        if (flagword & 0xFF) != 0 {
            lf_checker_rt::callee_cdecl!(8, u32,);
            lf_checker_rt::callee_thiscall!(9, u32, lf_checker_rt::relocated(THIS_B_FILE));
        }
        lf_checker_rt::callee_cdecl!(10, u32, ptr, level);
        lf_checker_rt::callee_cdecl!(11, u32,);
        lf_checker_rt::callee_cdecl!(12, u32, 0u32, 1u32)
    }
});
