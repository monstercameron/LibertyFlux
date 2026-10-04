// original: 0x008b4170 binding_record_init
/// Binding record initialiser.
///
/// Tags the record at `this_ptr` with its type marker, mixes the previous word
/// at offset 4 with a global sequence counter (masked to 14 bits, counter then
/// incremented), stores `value` at offset 8 and the word found at `slot_ptr`
/// at offset 12. Returns the record pointer.
export!(thiscall, rw_008b4170(this_ptr: u32, value: u32, slot_ptr: u32) -> u32 {
    unsafe {
        // Both markers are relocated image addresses (type tags); the worker maps
        // the image away from its preferred base, so derive them like globals.
        const MARKER_PROBE: u32 = 0x00E7E048;
        const MARKER_FINAL: u32 = 0x00E7E064;
        const COUNTER: u32 = 0x010327A0;
        const MIX_MASK: u32 = 0x3FFF;
        let base = this_ptr;
        let saved = ((base.wrapping_add(4)) as *const u32).read();
        ((base) as *mut u32).write(relocated(MARKER_PROBE));
        let counter = global::<u32>(COUNTER);
        let mix = (saved ^ counter.read()) & MIX_MASK;
        ((base.wrapping_add(4)) as *mut u32).write(saved ^ mix);
        counter.write(counter.read().wrapping_add(1));
        ((base) as *mut u32).write(relocated(MARKER_FINAL));
        ((base.wrapping_add(0xC)) as *mut u32).write(0xFFFF_FFFF);
        ((base.wrapping_add(8)) as *mut u32).write(value);
        let slot = (slot_ptr as *const u32).read();
        ((base.wrapping_add(0xC)) as *mut u32).write(slot);
        base
    }
});
