// original: 0x00A0D640 streaming_emit_ui_record_gated
/// Checks the mode-specific streaming flag and the entity UI bit, then
/// forwards the object, raw f32 bits, bit mask, and zero to the UI helper.
lf_checker_rt::export!(cdecl, rw_00A0D640(entity: u32, amount_bits: u32, bit_index: u32, mode_state: u32) -> () {
    unsafe {
        const MODE: u32 = 0x3C;
        const ENTITY_FLAGS: u32 = 0x24;
        const MODE_ONE_BLOCK: u32 = 0x0001_0000;
        const MODE_TWO_BLOCK: u32 = 0x0002_0000;
        const EMIT_UI_BIT: u32 = 0x20;
        let mode = (mode_state.wrapping_add(MODE) as *const u32).read_unaligned();
        let flags = (entity.wrapping_add(ENTITY_FLAGS) as *const u32).read_unaligned();
        let blocking = if mode == 2 { MODE_TWO_BLOCK } else { MODE_ONE_BLOCK };
        if flags & blocking != 0 || flags & EMIT_UI_BIT == 0 {
            return;
        }
        let mask = 1u32.wrapping_shl(bit_index & 0x1F);
        let _ = lf_checker_rt::callee_cdecl!(1, u32, entity, amount_bits, mask, 0u32);
    }
});
