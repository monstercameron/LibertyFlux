// original: 0x005e7320 GET_NUMBER_OF_INJURED_PEDS_IN_RANGE
/// Count the injured characters within range of a point.
///
/// Builds a position vector from script arguments 0-2, forwards its address
/// in EDX with the incoming ECX word to the engine implementation, and stores
/// the returned count in the return slot. A fourth script word is loaded and
/// dropped.
///
/// The contract fixes the incoming ECX word (the rewrite cannot observe it),
/// skips the EDX address and snapshots the three pointed-to words.
export!(cdecl, rw_005e7320(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let vec = [*args, *args.add(1), *args.add(2)];
        let count = callee_fastcall!(1, u32, 0, vec.as_ptr() as u32, 0x12345678);
        let slot = *(ctx as *const u32);
        *(slot as *mut u32) = count;
        count
    }
});
