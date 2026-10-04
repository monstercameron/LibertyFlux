// original: 0x009086a0 blip_refresh_clear_bit6
/// Refresh a blip's trailing record block, then clear flag bit 6 when set.
///
/// Resolves the record and bails out with its pointer when it has no own
/// data. Otherwise runs the engine refresh step over the block at +0x60,
/// re-resolves (falling back to the default blip when the flag went clear)
/// and clears bit 6 of the flag word when set. Returns 0xFFBF on the clearing
/// path, else the refresh answer with its low byte replaced by the tested bit.
export!(cdecl, rw_009086A0(id: u32, arg2: u32) -> u32 {
    unsafe {
        let entry = blip(id);
        if *entry.add(0x08) == 0 {
            return entry as u32;
        }
        let ans: u32 = callee_cdecl!(1, u32, (entry as u32).wrapping_add(0x60), arg2);
        let entry2 = blip(id);
        let tgt = if *entry2.add(0x08) != 0 {
            entry2
        } else {
            blip(*global::<u32>(BLIP_DEFAULT))
        };
        let bit = (*tgt.add(0x20) >> 6) & 1;
        if bit == 0 {
            ans & 0xFFFFFF00
        } else {
            *(tgt.add(0x20) as *mut u16) &= 0xFFBF;
            0xFFBF
        }
    }
});
