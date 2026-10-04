// original: 0x00ae0110 ui_dispatch_table_call
/// Dispatch a table record with this object's derived arguments.
///
/// Looks the record up by the u16 at 0x1a, derives a span pointer from the
/// byte at 0x27 (or null) and a slot address like `ui_slot_address_or_null`,
/// then calls the worker with (record, span, word at 0x30, slot, arg1, arg2,
/// 0). Returns the worker's answer.
export!(thiscall, rw_00ae0110(this: *const u8, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x01295CD8;
        let sel = *(this.add(0x1a) as *const u16) as u32;
        let record = *(global::<u32>(TABLE).add(sel as usize));
        let span_byte = *(this.add(0x27));
        let span = if span_byte != 0 {
            (this as u32).wrapping_add((span_byte as u32).wrapping_shl(4))
        } else {
            0
        };
        let flag_a = *(this.add(0x28)) as u32;
        let flag_b = *(this.add(0x26)) as u32;
        let base = *(this.add(0x18) as *const u16) as u32;
        let index = flag_b
            .wrapping_sub((flag_a & 1).wrapping_mul(3))
            .wrapping_add(4)
            .wrapping_shl(4);
        let slot = if index == base {
            0
        } else {
            (this as u32).wrapping_add(index)
        };
        let extra = *(this.add(0x30) as *const u32);
        callee_cdecl!(1, u32, record, span, extra, slot, arg1, arg2, 0)
    }
});
