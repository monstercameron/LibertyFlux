// original: 0x005B5B20 guarded_handle_release
/// Releases the shared handle through the release helper and clears the slot,
/// unless the slot is already empty.
export!(cdecl, rw_005B5B20() -> u32 {
    unsafe {
        let slot = global::<u32>(0x018B6DF8);
        let h = slot.read();
        if h != 0 {
            callee_cdecl!(1, u32, h);
            slot.write(0);
        }
        0
    }
});
