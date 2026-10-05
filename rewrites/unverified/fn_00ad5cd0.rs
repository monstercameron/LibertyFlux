// original: 0x00AD5CD0 audio_slot_ptr_from_top (proposed)

/// Address a stride-0x10c4 audio slot table downwards from its top.
///
/// Returns `top - slot * stride` where `slot` is the current-slot global and
/// `top` is the table's top address (cdecl/0; the multiplication and the
/// subtraction wrap). The top address is an relocated image address.
lf_checker_rt::export!(cdecl, rw_00ad5cd0() -> u32 {
    unsafe {
        const TOP: u32 = 0x0154FD24;
        const STRIDE: u32 = 0x10C4;
        const SLOT: u32 = 0x01550DF8;
        let slot = lf_checker_rt::global::<u32>(SLOT).read();
        lf_checker_rt::relocated(TOP).wrapping_sub(slot.wrapping_mul(STRIDE))
    }
});
