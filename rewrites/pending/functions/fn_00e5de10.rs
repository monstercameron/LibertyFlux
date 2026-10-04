// original: 0x00e5de10 M_Y_GIRI_LO_01_GANG_01
/// Resolve model `M_Y_GIRI_LO_01_GANG_01` to its runtime id and cache it.
///
/// Passes the model's name string and a zero flag to the game's name-lookup
/// helper, stores the returned id in this model's global slot, and returns it.
/// (The helper itself lives in the game's encrypted code range, so only the
/// call shape is known: caller-cleaned stack, two arguments.)
checker_rt::export!(cdecl, rw_00e5de10() -> u32 {
    let name = checker_rt::relocated(0x00F8A9A4);
    let id = checker_rt::callee_cdecl!(1, u32, name, 0);
    unsafe { *checker_rt::global::<u32>(0x019D2F54) = id; }
    id
});

