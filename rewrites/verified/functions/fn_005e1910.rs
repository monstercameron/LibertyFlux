// original: 0x005E1910 CRenderPhasePlayerSettings::vf4
/// When the relocated settings guard byte is nonzero, call the update helper
/// with this object and the source pointer, then copy three 32-bit source
/// fields at offsets `0x50c`, `0x510` and `0x514` into destination offsets
/// `0x940`, `0x944` and `0x948`. With a zero guard, leave the destination
/// unchanged. The method returns zero on the guarded-off path under the
/// contract's zero-eax input; on the copy path its incidental eax is the last
/// copied word. It consumes the source pointer stack argument.
lf_checker_rt::export!(thiscall, rw_005e1910(this: u32, source: u32) -> u32 {
    const SETTINGS_GUARD_FILE_VA: u32 = 0x0116_0c39;
    const SOURCE_FIELDS: [u32; 3] = [0x50c, 0x510, 0x514];
    const DESTINATION_FIELDS: [u32; 3] = [0x940, 0x944, 0x948];

    let guard = lf_checker_rt::global::<u8>(SETTINGS_GUARD_FILE_VA);
    if unsafe { guard.read_unaligned() } == 0 {
        return 0;
    }
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this, source);
    let mut final_word = 0;
    for index in 0..SOURCE_FIELDS.len() {
        let value = unsafe {
            ((source + SOURCE_FIELDS[index]) as *const u32).read_unaligned()
        };
        unsafe {
            ((this + DESTINATION_FIELDS[index]) as *mut u32).write_unaligned(value);
        }
        final_word = value;
    }
    final_word
});
