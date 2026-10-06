// original: 0x0090a360 input_capture_store (proposed)
/// Store one capture frame into the bank selected by the static mode.
///
/// The mode byte (clamped to 3) picks one of four banks; the bank's counter
/// picks the frame slot. Sixteen bytes from `p0`, sixteen from `p1` and one
/// word through `p2` are copied into the slot, the counter is incremented,
/// and when it reaches `0xAA` the flush callee runs with the bank number.
/// Returns the new counter, or the flush answer when flushed. Cdecl, three
/// stack words.
export!(cdecl, rw_0090a360(p0: u32, p1: u32, p2: u32) -> u32 {
    unsafe {
        /// Static mode byte (file VA; read as the low byte of this word).
        const MODE: u32 = 0x010366C1;
        /// Per-bank frame counters, four words (file VA).
        const COUNTERS: u32 = 0x01195EA0;
        /// Frame bank base (file VA).
        const BANK: u32 = 0x01195F70;
        /// Frames per bank; reaching it flushes.
        const FRAMES: u32 = 0xAA;
        /// Bank stride in frames and frame size in bytes.
        const BANK_STRIDE: u32 = 0xAA;
        const FRAME_BYTES: u32 = 36;
        const FLUSH_ID: u32 = 1;
        let m = (global::<u8>(MODE)).read();
        let bank = if m < 3 { m as u32 } else { 3 };
        let cntp = relocated(COUNTERS).wrapping_add(bank.wrapping_mul(4)) as *mut u32;
        let cnt = cntp.read_unaligned();
        let slot = (relocated(BANK) as usize)
            .wrapping_add(((bank.wrapping_mul(BANK_STRIDE).wrapping_add(cnt)) as usize)
                .wrapping_mul(FRAME_BYTES as usize)) as u32;
        for i in 0..2u32 {
            let q = ((p0.wrapping_add(i.wrapping_mul(8))) as *const u64).read_unaligned();
            ((slot.wrapping_add(i.wrapping_mul(8))) as *mut u64).write_unaligned(q);
        }
        for i in 0..2u32 {
            let q = ((p1.wrapping_add(i.wrapping_mul(8))) as *const u64).read_unaligned();
            ((slot.wrapping_add(16).wrapping_add(i.wrapping_mul(8))) as *mut u64)
                .write_unaligned(q);
        }
        let w = (p2 as *const u32).read_unaligned();
        ((slot.wrapping_add(32)) as *mut u32).write_unaligned(w);
        let new = cnt.wrapping_add(1);
        cntp.write_unaligned(new);
        if new >= FRAMES {
            callee_cdecl!(FLUSH_ID, u32, bank)
        } else {
            new
        }
    }
});
