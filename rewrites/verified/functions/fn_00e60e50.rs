// original: 0x00e60e50 zero_global_pair
/// zero_global_pair: zero a pair of adjacent globals.
///
/// The original preserves EAX, which the contract leaves unchecked
/// (no Rust code can observe the caller's EAX).
lf_checker_rt::export!(cdecl, rw_00e60e50() -> u32 {
    unsafe {
        lf_checker_rt::global::<u32>(0x019FD9B0).write(0);
        lf_checker_rt::global::<u32>(0x019FD9B4).write(0);
        0
    }
});
