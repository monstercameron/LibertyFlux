// original: 0x0093F3C0 stream_slots_rank (proposed)

/// Rank the 32 streaming slots and return the first inactive index.
///
/// Builds a 32-entry array of `SLOT_BASE + 4*i` and passes it to the
/// sort callee (which is stubbed, so the array keeps its order), then
/// scans it: the index is recovered as `(entry - SLOT_BASE) / 4` and the
/// slot looked up in the slot table. A null slot, or one whose word at
/// `OBJ_STATE` equals `STATE_READY`, triggers the poll callee: a zero
/// answer, or a zero gate word at `GATE + 28*i`, returns the index,
/// otherwise the scan continues. A slot in any other state is skipped
/// silently. Exhausting all 32 returns -1.
///
/// Original: 0x0093F3C0 (cdecl, no arguments; two direct callees).
lf_checker_rt::export!(cdecl, rw_0093f3c0() -> u32 {
    const SLOT_BASE: u32 = 0x11A8888;
    const SLOT_COUNT: u32 = 32;
    const SLOT_TABLE: u32 = 0x11A8808;
    const OBJ_STATE: u32 = 0x4E8;
    const STATE_READY: u32 = 6;
    const GATE: u32 = 0x19392E0;
    const GATE_STRIDE: u32 = 0x1C;
    const SORT: u32 = 1;
    const POLL: u32 = 2;
    unsafe {
        let base = lf_checker_rt::relocated(SLOT_BASE);
        let mut arr = [0u32; 32];
        for (i, slot) in arr.iter_mut().enumerate() {
            *slot = base.wrapping_add(i as u32 * 4);
        }
        let begin = arr.as_ptr() as u32;
        lf_checker_rt::callee_cdecl!(SORT, u32, begin, begin.wrapping_add(128), 0u32);
        let table = lf_checker_rt::global::<u32>(SLOT_TABLE) as *const u32;
        for i in 0..SLOT_COUNT {
            let entry = (begin.wrapping_add(i * 4) as *const u32).read_unaligned();
            let idx = ((entry.wrapping_sub(base)) as i32 >> 2) as u32;
            let slot = table.add(idx as usize).read_unaligned();
            let mut poll = slot == 0;
            if !poll {
                poll = ((slot + OBJ_STATE) as *const u32).read_unaligned() == STATE_READY;
            }
            if poll {
                let ans: u32 = lf_checker_rt::callee_cdecl!(POLL, u32,);
                if (ans & 0xFF) == 0 {
                    return idx;
                }
                let gate = lf_checker_rt::relocated(GATE).wrapping_add(i * GATE_STRIDE);
                if ((gate as *const u32).read_unaligned()) == 0 {
                    return idx;
                }
            }
        }
        u32::MAX
    }
});
