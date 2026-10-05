// original: 0x00a91c40 stream_slot_flag

/// Flags a streaming slot selected through the manager.
///
/// `mgr` (global at file VA 0x12FB258) holds an array base at `+0`, a probe
/// offset at `+4` and a stride at `+0xC`. Probes the byte at `ptr + offset`:
/// when its top bit is clear, writes `(arg2 == 0)` to
/// `base + stride * ptr + 0x54`. When the top bit is set, computes the same
/// byte but stores it at absolute address 0x54, faulting exactly like the
/// original. Returns the untouched upper bits of the offset with the low
/// byte replaced by the flag. No calls.
/// Original: 0x00A91C40 (cdecl, two stack words), 55 bytes.
lf_checker_rt::export!(cdecl, rw_00a91c40(ptr: u32, arg2: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x12FB258;
        const BASE_OFF: u32 = 0x00;
        const PROBE_OFF: u32 = 0x04;
        const STRIDE_OFF: u32 = 0x0C;
        const FLAG_OFF: u32 = 0x54;
        const TOP_BIT: u8 = 0x80;
        let mgr = lf_checker_rt::global::<u32>(MGR).read();
        let off = (mgr.wrapping_add(PROBE_OFF) as *const u32).read_unaligned();
        let probe = (ptr.wrapping_add(off) as *const u8).read();
        let flag = ((arg2 & 0xFF) == 0) as u32;
        if (probe & TOP_BIT) != 0 {
            (FLAG_OFF as *mut u8).write(flag as u8);
        } else {
            let stride = (mgr.wrapping_add(STRIDE_OFF) as *const u32).read_unaligned();
            let base = (mgr.wrapping_add(BASE_OFF) as *const u32).read_unaligned();
            let slot = base.wrapping_add(stride.wrapping_mul(ptr));
            (slot.wrapping_add(FLAG_OFF) as *mut u8).write(flag as u8);
        }
        (off & 0xFFFF_FF00) | flag
    }
});
