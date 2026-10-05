// original: 0x008CA540 stream_request_pending (proposed)

/// Tests whether a streaming request slot needs no more work: 1 when the
/// slot is inactive (flag byte at `+FLAG` is 0), when its state dword at
/// `+STATE` is not `BUSY_STATE`, or when its progress dword at `+PROGRESS`
/// has reached `DONE_AT` or more; 0 only for a busy slot below the mark.
///
/// Thiscall on the slot pointer in ECX; the byte result is returned in AL.
lf_checker_rt::export!(thiscall, rw_008CA540(this: u32) -> u32 {
    unsafe {
        /// Offset of the active flag byte.
        const FLAG: u32 = 4;
        /// Offset of the state dword.
        const STATE: u32 = 8;
        /// State value meaning the slot is busy.
        const BUSY_STATE: u32 = 2;
        /// Progress value meaning done.
        const DONE_AT: u32 = 0xBB8;
        let active = ((this + FLAG) as *const u8).read();
        if active == 0 {
            return 1;
        }
        let state = ((this + STATE) as *const u32).read_unaligned();
        if state != BUSY_STATE {
            return 1;
        }
        let progress = (this as *const u32).read_unaligned();
        u32::from(progress >= DONE_AT)
    }
});
