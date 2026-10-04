// original: 0x00e604c0 reset_timing_state
/// Resets the global timing state block to its start-up values.
///
/// Zeroes the gauge words, stamps the inactive marker (`0xFFFFFFFF`) into
/// the slot words, keeps two pad bytes untouched, and masks two flag
/// bytes down to their preserved bits. Returns the masked mode byte.
export!(cdecl, rw_00e604c0() -> u8 {
    use core::ptr::write_unaligned;
    const INACTIVE: u32 = 0xFFFF_FFFF;
    unsafe {
        let flag = global::<u8>(0x019F32AC);
        *flag &= 0x80;
        let mode = *global::<u8>(0x019F329D) & 0xFC;
        // All offsets below are relative to the block base.
        let b = relocated(0x019F3230);
        let w64 = |off: u32| write_unaligned((b + off) as *mut u64, 0);
        let w32 = |off: u32, v: u32| write_unaligned((b + off) as *mut u32, v);
        let w16 = |off: u32| write_unaligned((b + off) as *mut u16, 0);
        w64(0x000); w64(0x008); w64(0x050); w64(0x048);
        w64(0x05C); w64(0x064);
        w32(0x010, 0); w32(0x014, INACTIVE);
        w16(0x018); w32(0x01C, INACTIVE);
        w16(0x020); w32(0x024, INACTIVE);
        w16(0x028); w32(0x02C, 0);
        w64(0x030);
        w32(0x038, INACTIVE); w32(0x03C, INACTIVE);
        w32(0x040, INACTIVE); w32(0x044, INACTIVE);
        w64(0x048);
        w16(0x050); w32(0x058, INACTIVE);
        w64(0x05C); w64(0x064);
        write_unaligned((b + 0x06C) as *mut u8, 0);
        write_unaligned((b + 0x06D) as *mut u8, mode);
        let c = relocated(0x019F32A0);
        write_unaligned(c as *mut u32, INACTIVE);
        write_unaligned((c + 4) as *mut u32, INACTIVE);
        write_unaligned((c + 8) as *mut u32, 0);
        mode
    }
});
