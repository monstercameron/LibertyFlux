// original: 0x00afe020 is_current_selection
/// Report whether the given value is the current selection.
export!(cdecl, rw_00afe020(arg: u32) -> u32 {
    unsafe {
        if *global::<u32>(0x1600184) == arg { 1 } else { 0 }
    }
});
