// original: 0x00975030 audio_pose_snapshot_set (proposed)

/// Snapshot a 40-byte pose block selected through a provider object.
///
/// A null provider clears the flag at +0xA8 (and the original returns its
/// untouched incoming accumulator, so no return channel is compared).
/// Otherwise the flag is set, the provider's slot at vtable +0xC yields a
/// block pointer, and 40 bytes are copied to +0x4 as five 8-byte moves.
/// Original: 0x00975030 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00975030(this: u32, provider: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0xA8;
        const DST: u32 = 4;
        const SLOT: u32 = 0xC;
        const WORDS: u32 = 5;
        if provider == 0 {
            ((this.wrapping_add(FLAG)) as *mut u8).write(0);
            return 0;
        }
        ((this.wrapping_add(FLAG)) as *mut u8).write(1);
        let vt = (provider as *const u32).read_unaligned();
        let target = ((vt.wrapping_add(SLOT)) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
        let src = f(provider);
        for i in 0..WORDS {
            let w = ((src.wrapping_add(i * 8)) as *const u64).read_unaligned();
            ((this.wrapping_add(DST).wrapping_add(i * 8)) as *mut u64).write_unaligned(w);
        }
        0
    }
});
