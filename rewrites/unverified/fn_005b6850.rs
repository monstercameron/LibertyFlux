// original: 0x005B6850 find_with_temp_key (proposed)

/// Look up `key` through a temporary on-stack search key.
///
/// Builds a 0x88-byte search key on the stack with the key at +0x80 and -1
/// at +0x84 (the rest is uninitialised scratch), aligns the stack to 8, and
/// calls the row-search callee with a pointer to the key. The callee is
/// stubbed by the proof: the frame pointer itself is skipped in the call
/// comparison while the two initialised words are snapshotted (the scratch
/// words are never compared), and ECX passes whatever the caller left
/// through to the stub uncompared. Returns the callee's answer with the
/// stack restored. Stdcall: key on the stack.
lf_checker_rt::export!(stdcall, rw_005B6850(key: u32) -> u32 {
    unsafe {
        const KEY_WORDS: usize = 0x88 / 4;
        const KEY_OFF: usize = 0x80 / 4;
        let mut buf = [0u32; KEY_WORDS];
        buf[KEY_OFF] = key;
        buf[KEY_OFF + 1] = 0xFFFF_FFFF;
        lf_checker_rt::callee_stdcall!(1, u32, buf.as_mut_ptr() as u32)
    }
});
