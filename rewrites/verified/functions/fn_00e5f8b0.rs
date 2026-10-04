// original: 0x00e5f8b0 timing_slot_bind_f8b0
/// Initialise one static timing slot, then register its callback stub.
///
/// The original calls the slot's tiny initializer (a zero-argument routine
/// that only writes that slot's absolute globals) and then registers the
/// callback stub at `0x00E6F4A0` with the shared registrar, returning the
/// registrar's answer. The pushed stub address carries a HIGHLOW fixup and
/// is relocated to the worker's image base. Both callees are intercepted
/// and answered by the checker.
lf_checker_rt::export!(cdecl, rw_00e5f8b0() -> u32 {
    unsafe {
        /// Callback stub registered for this slot (file VA, relocated).
        const CALLBACK: u32 = 0x00E6F4A0;
        let _: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(CALLBACK))
    }
});
