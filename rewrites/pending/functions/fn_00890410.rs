// original: 0x00890410 audio_slot_node_lookup
/// Look up the node for slot `idx` of this object, or null for an empty slot.
///
/// Reads the slot byte at `this+0x48+idx`; 0xff means empty. Otherwise the
/// node address is `stride_a * slot + row_a[variant]`, where `stride_a` is a
/// global and `row_a` comes from the global row-pointer table indexed by the
/// object's variant byte at `this+0x40`.
export!(thiscall, rw_00890410(this: u32, idx: u32) -> u32 {
    unsafe {
        let slot = ((this.wrapping_add(idx).wrapping_add(0x48)) as *const u8).read();
        if slot == 0xff {
            return 0;
        }
        let stride = *global::<u32>(0x0115D964);
        let variant = ((this + 0x40) as *const u8).read() as u32;
        let base = *global::<u32>(0x0115D988);
        let row = ((base
            .wrapping_add(variant.wrapping_mul(0x6f40))
            .wrapping_add(0x6f10)) as *const u32)
            .read();
        stride.wrapping_mul(slot as u32).wrapping_add(row)
    }
});
