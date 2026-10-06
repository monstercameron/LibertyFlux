// original: 0x00908630 blip_record_attach
/// Attach a blip record to the current thread's engine state.
///
/// Returns the record pointer unchanged when the record has no own data.
/// Otherwise runs the register step and the refresh step against the thread
/// anchor (the TLS block named by the index global, plus 0x78), then sets
/// flag bit 6 on the record, falling back to the default blip's record when
/// the flag went clear in between. Returns the flagged record pointer.
export!(cdecl, rw_00908630(id: u32, arg2: u32) -> u32 {
    unsafe {
        let table = global::<u32>(BLIP_TABLE);
        let entry = *table.add(id as usize) as *mut u8;
        if *entry.add(8) == 0 {
            return entry as u32;
        }
        let idx = *global::<u32>(TLS_QUEUE_INDEX);
        let anchor = tls_slot(idx as usize).wrapping_add(THREAD_ANCHOR_OFF);
        callee_cdecl!(1, u32, arg2, anchor);
        callee_cdecl!(2, u32, id, anchor);
        let entry2 = *table.add(id as usize) as *mut u8;
        let tgt = if *entry2.add(8) != 0 {
            entry2
        } else {
            let d = *global::<u32>(BLIP_DEFAULT);
            *table.add(d as usize) as *mut u8
        };
        let flags = tgt.add(0x20) as *mut u16;
        *flags |= 0x40;
        tgt as u32
    }
});
