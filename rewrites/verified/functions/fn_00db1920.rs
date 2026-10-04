// original: 0x00db1920 reset_ui_mode_state
/// Reset the two global UI mode words to their idle values.
///
/// Writes the mode selector and its companion counter; touches no object
/// state and takes no arguments.
export!(cdecl, rw_00db1920() -> u32 {
    unsafe {
        const MODE_SELECTOR: u32 = 0x017a65a8;
        const MODE_COUNTER: u32 = 0x017a65ac;
        *global::<u32>(MODE_SELECTOR) = 3;
        *global::<u32>(MODE_COUNTER) = 0;
        0
    }
});
