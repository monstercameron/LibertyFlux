// original: 0x00984200 audio_wrap_spatial_params

/// Copies parameter words at +0x50, +0x54 and +0x58 into a three-word local
/// vector, then passes the vector with the block fields at +0x40 and +0x4c,
/// the byte at +0x85 and integer 0 to helper 1. The contract skips stack
/// argument 1 (the vector address) and snapshots its three initialized words
/// at offsets 0, 4 and 8. EAX is compared to the helper answer.
///
/// Scope: one straight-line call; helper 1 uses scripted edge returns, so its
/// native implementation is outside this proof. The fourth local vector word
/// is not snapshotted.
lf_checker_rt::export!(thiscall, rw_00984200(this: u32, params: u32) -> u32 {
    unsafe {
        let p = params as *const u8;
        let mut vector = [0u32; 4];
        vector[0] = (p.add(0x50) as *const u32).read_unaligned();
        vector[1] = (p.add(0x54) as *const u32).read_unaligned();
        vector[2] = (p.add(0x58) as *const u32).read_unaligned();

        let first = (p.add(0x40) as *const u32).read_unaligned();
        let second = (p.add(0x4c) as *const u32).read_unaligned();
        let mode = p.add(0x85).read() as u32;
        lf_checker_rt::callee_thiscall!(
            1,
            u32,
            this,
            first,
            vector.as_ptr() as u32,
            second,
            mode,
            0
        )
    }
});
