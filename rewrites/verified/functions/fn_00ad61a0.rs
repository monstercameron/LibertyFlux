// original: 0x00AD61A0 audio_slot_ptr_from_base (proposed)

/// Address a stride-0x10c4 audio slot table upwards from its base.
///
/// Returns `base + slot * stride` where `slot` is the current-slot global
/// (cdecl/0; the multiplication and the addition wrap). The base address is
/// an relocated image address.
lf_checker_rt::export!(cdecl, rw_00ad61a0() -> u32 {
    unsafe {
        const BASE: u32 = 0x0154EC60;
        const STRIDE: u32 = 0x10C4;
        const SLOT: u32 = 0x01550DF8;
        let slot = lf_checker_rt::global::<u32>(SLOT).read();
        (slot.wrapping_mul(STRIDE)).wrapping_add(lf_checker_rt::relocated(BASE))
    }
});
