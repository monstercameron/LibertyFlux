// original: 0x00c084b0 stream_state_save_a (proposed)

/// Pass every field of the streaming state to the writer, in order.
///
/// `this` points to the state and `arg` is handed to each call; the writer
/// (callee 1, three arguments: the caller's word, a field pointer, a byte
/// size) sees the fields at `+0x04`, `+0x08`, `+0x0c` (256 bytes), `+0x10c`,
/// `+0x110` (8 bytes), `+0x00` and `+0x118`, each with its size. Returns the
/// writer's last answer.
///
/// Original: 0x00c084b0 (thiscall, one stack word; writer is cdecl).
lf_checker_rt::export!(thiscall, rw_00c084b0(this: u32, arg: u32) -> u32 {
    unsafe {
        const WRITER: u32 = 1;
        const FIELDS: [(u32, u32); 7] =
            [(0x04, 4), (0x08, 4), (0x0c, 0x100), (0x10c, 4), (0x110, 8), (0x00, 4), (0x118, 4)];
        let mut r = 0u32;
        let mut k = 0usize;
        while k < FIELDS.len() {
            let (off, size) = FIELDS[k];
            r = lf_checker_rt::callee_cdecl!(WRITER, u32, arg, this.wrapping_add(off), size);
            k += 1;
        }
        r
    }
});
