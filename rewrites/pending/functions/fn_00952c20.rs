// original: 0x00952c20 record_seed_from_clock
/// Seed a 12-byte record from the shared clock words, then fold in a tick.
///
/// Copies the two clock words and the generation byte-word into `dst`. When
/// the source flags carry bit 1, the tick replaces the low word, the counter
/// byte advances, and the high word is reloaded from the table slot the new
/// counter selects; otherwise the tick is added to the low word.
export!(cdecl, rw_00952c20(dst: u32, src: u32) -> u32 {
    unsafe {
        core::ptr::copy_nonoverlapping(
            global::<u8>(0x120CA50),
            dst as *mut u8,
            12,
        );
        let flags = *((src.wrapping_add(4)) as *const u8);
        if flags & 2 != 0 {
            let tick: u32 = callee_cdecl!(0, u32,);
            let counter = (dst.wrapping_add(8)) as *mut u8;
            *counter = (*counter).wrapping_add(1);
            *(dst as *mut u32) = tick;
            let slot = *counter as usize;
            let entry = *(relocated(0x11F6F7C).wrapping_add(slot as u32 * 4) as *const u32);
            *((dst.wrapping_add(4)) as *mut u32) = entry;
            entry
        } else {
            let tick: u32 = callee_cdecl!(0, u32,);
            let low = dst as *mut u32;
            *low = (*low).wrapping_add(tick);
            tick
        }
    }
});
