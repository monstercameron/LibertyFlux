// original: 0x00a91e70 stream_slot_word1

/// Loads the second word of a streaming slot through the manager.
///
/// Same as rw_00a91dd0 but reads four bytes past the slot start (and from
/// address 4 on the faulting path). No calls.
/// Original: 0x00A91E70 (cdecl, one stack word), 37 bytes.
lf_checker_rt::export!(cdecl, rw_00a91e70(ptr: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x12FB258;
        let mgr = lf_checker_rt::global::<u32>(MGR).read();
        let off = (mgr.wrapping_add(4) as *const u32).read_unaligned();
        let probe = (ptr.wrapping_add(off) as *const u8).read();
        if (probe & 0x80) != 0 {
            core::hint::black_box(4 as *const u32).read_unaligned()
        } else {
            let stride = (mgr.wrapping_add(0xC) as *const u32).read_unaligned();
            let base = (mgr as *const u32).read_unaligned();
            (base.wrapping_add(stride.wrapping_mul(ptr)).wrapping_add(4) as *const u32)
                .read_unaligned()
        }
    }
});
