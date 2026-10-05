// original: 0x00ab5dd0 stream_append_handle (proposed)

/// Resolve a handle pair and append it to the global handle table.
///
/// Calls the resolver callee with `(a, b)` and returns its answer. A
/// negative answer ends the call; otherwise the global slot count is read,
/// bumped by one, and the answer is stored at that slot of the global
/// table. Returns the resolver's answer either way.
///
/// Callees: 1 = handle resolver (stdcall, two words).
///
/// Original: 0x00ab5dd0 (stdcall, two stack words).
lf_checker_rt::export!(stdcall, rw_00ab5dd0(a: u32, b: u32) -> u32 {
    unsafe {
        const RESOLVER: u32 = 1;
        const TABLE: u32 = 0x0150_E100;
        const COUNT: u32 = 0x0150_E128;
        let ans = lf_checker_rt::callee_stdcall!(RESOLVER, u32, a, b);
        if (ans as i32) < 0 {
            return ans;
        }
        let slot = (lf_checker_rt::global::<u32>(COUNT) as *const u32).read_unaligned();
        (lf_checker_rt::global::<u32>(COUNT) as *mut u32)
            .write_unaligned(slot.wrapping_add(1));
        let cell = (lf_checker_rt::global::<u32>(TABLE) as u32)
            .wrapping_add(slot.wrapping_mul(4));
        (cell as *mut u32).write_unaligned(ans);
        ans
    }
});
