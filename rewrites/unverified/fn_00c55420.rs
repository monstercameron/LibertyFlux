// original: 0x00C55420 destroy_entity_scanner
/// Store the entity-scanner type tag, release its event-reference slots, and
/// tail-call the audio notification helper with `this + 4` in ECX. Return the
/// tail callee's EAX result. There are no stack arguments; both callees are
/// intercepted by the proof contract.
lf_checker_rt::export!(thiscall, rw_c55420(this: u32) -> u32 {
    unsafe {
        const TYPE_TAG_VA: u32 = 0x00ec_b014;
        const SCANNER_BASE: u32 = 4;

        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(TYPE_TAG_VA));
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        lf_checker_rt::callee_thiscall!(2, u32, this.wrapping_add(SCANNER_BASE))
    }
});
