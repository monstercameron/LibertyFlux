// original: 0x00e686f0 veh_thunk_86f0

/// Tail-thunk into the shared vehicle routine with a static object.
///
/// The original loads `VEH_OBJ` into ECX and jumps to the shared routine;
/// the checker patches the jump to a recorder stub. The rewrite forwards
/// the same object address through a call and returns the answer. Takes no
/// stack arguments.
///
/// Original: 0x00E686F0 (cdecl/0, one tail jump; 10 bytes of code).
lf_checker_rt::export!(cdecl, rw_00e686f0() -> u32 {
    unsafe {
        /// Static vehicle object forwarded to the shared routine (file VA).
        const VEH_OBJ: u32 = 0x013BABA0;
        lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(VEH_OBJ))
    }
});
