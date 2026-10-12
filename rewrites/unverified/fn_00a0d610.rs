// original: 0x00A0D610 entity_emit_ui_bit_if_flagged
/// Emits a four-word UI record only when the entity's UI bit is set. The
/// mask shifts by the low five bits of the requested bit index.
lf_checker_rt::export!(cdecl, rw_00A0D610(entity: u32, bit_index: u32) -> () {
    unsafe {
        const ENTITY_FLAGS: u32 = 0x24;
        const EMIT_UI_BIT: u8 = 0x20;
        let flags = (entity.wrapping_add(ENTITY_FLAGS) as *const u8).read();
        if flags & EMIT_UI_BIT != 0 {
            let mask = 1u32.wrapping_shl(bit_index & 0x1F);
            let _ = lf_checker_rt::callee_cdecl!(1, u32, entity, 0u32, mask, mask);
        }
    }
});
