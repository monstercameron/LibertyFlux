// original: 0x009f0890 playerped_timed_trigger
/// Fire a timed trigger when the state at `0x7b8` is 6 and stale.
///
/// Returns 0 unless field `0x7b8` equals 6 (its `eax` on that path is
/// whatever the caller left there; the contract fixes entry `eax` to 0
/// since it is not an argument). Otherwise subtracts the stamp at `0x7bc`
/// from the global tick counter; when the elapsed time exceeds 1000 it
/// calls the trigger entry with (1, 1, 1) and returns its answer, else it
/// returns the elapsed time itself.
export!(thiscall, rw_009f0890(this_ptr: u32) -> u32 {
    unsafe {
        if *((this_ptr + 0x7b8) as *const u32) != 6 {
            return 0;
        }
        let now = *global::<u32>(0x11735b4);
        let stamp = *((this_ptr + 0x7bc) as *const u32);
        let elapsed = now.wrapping_sub(stamp);
        if elapsed > 0x3e8 {
            callee_thiscall!(2, u32, this_ptr, 1, 1, 1)
        } else {
            elapsed
        }
    }
});
