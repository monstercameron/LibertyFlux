// original: 0x00e5dfb0 F_Y_MPLATIN_02
/// Resolve model `F_Y_MPLATIN_02` to its runtime id and cache it.
///
/// Passes the model's name string and a zero flag to the game's name-lookup
/// helper, stores the returned id in this model's global slot, and returns it.
/// (The helper itself lives in the game's encrypted code range, so only the
/// call shape is known: caller-cleaned stack, two arguments.)
lf_k2_rt::export!(cdecl, rw_00e5dfb0() -> u32 {
    let name = lf_k2_rt::relocated(0x00F8A828);
    let id = lf_k2_rt::callee_cdecl!(1, u32, name, 0);
    unsafe { *lf_k2_rt::global::<u32>(0x019D2FD8) = id; }
    id
});

