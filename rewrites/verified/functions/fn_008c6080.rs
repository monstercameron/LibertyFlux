// original: 0x008C6080 stream_flag_fe_get
/// Read one streaming-slot flag byte selected by a caller-supplied index.
///
/// `this` points at the streaming control block and `idx` selects a slot.
/// Returns the byte at `this + idx + 0xFE` (a per-slot state flag) with
/// the upper 24 bits clear. The original leaves the index's own upper bits
/// in the result; only the low byte is behaviour. Takes no heap lock and
/// makes no calls. Original: thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_008c6080(this: u32, idx: u32) -> u32 {
    unsafe {
        const FLAG_BASE: u32 = 0xFE;
        let slot = this.wrapping_add(idx).wrapping_add(FLAG_BASE);
        (slot as *const u8).read() as u32
    }
});
