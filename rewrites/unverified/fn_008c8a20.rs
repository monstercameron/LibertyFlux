// original: 0x008C8A20 stream_key_unpack
/// Unpack a packed sort key into the six words of a streaming record.
///
/// Asks the unpack callee to decode the packed pair at `packed` into a
/// scratch block, then spreads six of its words into `this` (offsets 0,
/// 4, 8, 12, 16 and 20), adding one to the word that lands at `+4`.
/// Reports 1 in the low byte. Original: thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_008c8a20(this: u32, packed: u32) -> u32 {
    unsafe {
        const UNPACK_CALLEE: u32 = 1;
        const COOKIE_CALLEE: u32 = 2;
        let mut block = [0u16; 8];
        let lo = (packed as *const u32).read_unaligned();
        let hi = ((packed + 4) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(
            UNPACK_CALLEE, u32, lo, hi, block.as_mut_ptr() as u32);
        ((this + 4) as *mut u32)
            .write_unaligned(block[1] as u32 + 1);
        ((this + 8) as *mut u32).write_unaligned(block[3] as u32);
        (this as *mut u32).write_unaligned(block[0] as u32);
        ((this + 12) as *mut u32).write_unaligned(block[4] as u32);
        ((this + 16) as *mut u32).write_unaligned(block[5] as u32);
        ((this + 20) as *mut u32).write_unaligned(block[6] as u32);
        lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
        1
    }
});
