// original: 0x00e68590 veh_slot_bind_8590

/// Initialise one static vehicle object, then register its default callback.
///
/// First calls the tiny object initializer (a thiscall taking only the
/// object address) with the static object at `VEH_OBJ`, then registers the
/// default callback stub at `CALLBACK` with the registrar (cdecl/1) and
/// returns the registrar's answer. Both callees are intercepted and answered
/// by the checker, so this function is fully described by the two outgoing
/// calls and the returned answer.
///
/// Original: 0x00E68590 (cdecl/0, two direct calls).
lf_checker_rt::export!(cdecl, rw_00e68590() -> u32 {
    unsafe {
        /// Static vehicle object this instance initialises (file VA).
        const VEH_OBJ: u32 = 0x01394D60;
        /// Default callback stub registered for it (file VA).
        const CALLBACK: u32 = 0x00E72370;
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(VEH_OBJ));
        lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(CALLBACK))
    }
});
