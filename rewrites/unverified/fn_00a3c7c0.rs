// original: 0x00a3c7c0 vehicle_float_slot_store (proposed)

/// Store a float into slot `+0x10ac` of the record reached through the
/// object's first word: `*obj` is a pointer to the record.
///
/// Takes the object in ECX and the float bits as one stack word (thiscall/1,
/// callee cleans 4). Returns the record pointer (`*obj`).
lf_checker_rt::export!(thiscall, rw_00a3c7c0(obj: u32, val_bits: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x10AC;
        let rec = core::ptr::read_unaligned(obj as *const u32);
        core::ptr::write_unaligned((rec + SLOT) as *mut u32, val_bits);
        rec
    }
});
