// original: 0x00C553F0 initialize_vehicle_scanner
/// Initialize the entity-scanner base with pool kind one, store the
/// vehicle-scanner type tag, register the three-word scanner field beginning
/// at `this + 4`, and return `this`. The registry helper receives a pointer
/// to those three words; both helper calls are intercepted in the proof.
lf_checker_rt::export!(thiscall, rw_c553f0(this: u32) -> u32 {
    unsafe {
        const POOL_KIND: u32 = 1;
        const TYPE_TAG_VA: u32 = 0x00ec_b020;
        const SCANNER_FIELDS: u32 = 4;

        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, POOL_KIND);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(TYPE_TAG_VA));
        let fields = this.wrapping_add(SCANNER_FIELDS);
        let _ = lf_checker_rt::callee_cdecl!(2, u32, fields);
        this
    }
});
