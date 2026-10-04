// original: 0x0097ad60 audio_listeners_apply_all
/// Apply an update to all three global audio listener records.
///
/// Calls the per-listener update on each table slot, including null ones.
export!(cdecl, rw_0097ad60(arg: u32) -> u32 {
    unsafe {
        let table = global::<u32>(0x12312D4);
        for i in 0..3usize {
            callee_thiscall!(1, u32, *table.add(i), arg);
        }
        0
    }
});
