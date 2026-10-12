// original: 0x008A86D0 audio_cycle_thread_slot

/// Step the audio thread slot cursor back by one, modulo four.
///
/// Reads the slot count at `SLOT_COUNT`, subtracts one, keeps the low two
/// bits, stores the cursor at `SLOT_CURSOR` and also at `ENTRY_CURSOR` past
/// the thread's audio entry. The entry comes from the thread-local audio
/// table (`tls_slot(0)`, the word the original reads through FS:[0x2c])
/// indexed by the slot index word `TLS_INDEX`. Returns the entry pointer.
/// Thiscall, object in ECX.
lf_checker_rt::export!(thiscall, rw_008A86D0(this: u32) -> u32 {
    unsafe {
        const SLOT_COUNT: u32 = 0x1704;
        const SLOT_CURSOR: u32 = 0x1708;
        const TLS_INDEX: u32 = 0x017A_BA14;
        const ENTRY_CURSOR: u32 = 0x70;
        const SLOT_MASK: u32 = 3;
        let count = (this.wrapping_add(SLOT_COUNT) as *const u32).read_unaligned();
        let tls = lf_checker_rt::tls_slot(0);
        let cursor = count.wrapping_sub(1) & SLOT_MASK;
        (this.wrapping_add(SLOT_CURSOR) as *mut u32).write_unaligned(cursor);
        let index = (lf_checker_rt::relocated(TLS_INDEX) as *const u32).read_unaligned();
        let entry = (tls.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        (entry.wrapping_add(ENTRY_CURSOR) as *mut u32).write_unaligned(cursor);
        entry
    }
});
