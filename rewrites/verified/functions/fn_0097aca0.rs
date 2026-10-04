// original: 0x0097aca0 audio_listeners_destroy_all
/// Destroy all three global audio listener records.
///
/// Frees each present listener through the global allocator and clears
/// its table slot.
export!(cdecl, rw_0097aca0() -> u32 {
    unsafe {
        let table = global::<u32>(0x12312D4);
        for i in 0..3usize {
            let e = *table.add(i);
            if e != 0 {
                callee_cdecl!(1, u32, e);
                *table.add(i) = 0;
            }
        }
        0
    }
});
