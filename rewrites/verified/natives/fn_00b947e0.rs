// original: 0x00b947e0 IS_AREA_OCCUPIED
/// Script native `IS_AREA_OCCUPIED` (hash 0x5BE1238D).
///
/// Passes a fixed engine predicate address and the call context itself to a
/// shared area-test dispatcher. The predicate address is an immediate in the
/// code with a HIGHLOW relocation entry (Verified: the checker observes the
/// original passing the relocated value), so it is derived with
/// `relocated()` like any other reference into the original image, never
/// hard-coded. No return slot is written.
export!(cdecl, rw_00b947e0(ctx: *const u8) -> u32 {
    unsafe {
        /// Engine area-test predicate passed to the dispatcher (file VA).
        const AREA_PREDICATE: u32 = 0x00b96810;
        callee_cdecl!(1, u32, lf_k2_rt::relocated(AREA_PREDICATE), ctx as u32)
    }
});
