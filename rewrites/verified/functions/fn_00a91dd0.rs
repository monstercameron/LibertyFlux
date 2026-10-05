// original: 0x00a91dd0 stream_slot_word0

/// Loads the head word of a streaming slot through the manager.
///
/// Same manager layout as rw_00a91c40. When the probed byte's top bit is
/// set, reads a dword from address 0, faulting exactly like the original;
/// otherwise returns the dword at `base + stride * ptr`. No calls.
/// Original: 0x00A91DD0 (cdecl, one stack word), 35 bytes.
lf_checker_rt::export!(cdecl, rw_00a91dd0(ptr: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x12FB258;
        let mgr = lf_checker_rt::global::<u32>(MGR).read();
        let off = (mgr.wrapping_add(4) as *const u32).read_unaligned();
        let probe = (ptr.wrapping_add(off) as *const u8).read();
        if (probe & 0x80) != 0 {
            core::hint::black_box(0 as *const u32).read_unaligned()
        } else {
            let stride = (mgr.wrapping_add(0xC) as *const u32).read_unaligned();
            let base = (mgr as *const u32).read_unaligned();
            (base.wrapping_add(stride.wrapping_mul(ptr)) as *const u32).read_unaligned()
        }
    }
});
