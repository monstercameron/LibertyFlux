// original: 0x008C5EC0 stream_dispatch_indexed
/// Dispatch streaming request `idx` through the indexed handler table.
///
/// Reads the handler row for `idx` from the table at `[this + 0x264]`
/// (16 bytes per row, dispatch word at row offset 4) and calls the
/// dispatch callee with that word, the request block `req` and mode 0.
/// Returns the callee's answer. Original: thiscall, two stack words.
lf_checker_rt::export!(thiscall, rw_008c5ec0(this: u32, idx: u32,
                                              req: u32) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 0x264;
        const ROW_STRIDE: u32 = 16;
        const ROW_WORD: u32 = 4;
        const DISPATCH_CALLEE: u32 = 1;
        let table = ((this + TABLE_OFF) as *const u32).read_unaligned();
        let word = (table.wrapping_add(idx.wrapping_mul(ROW_STRIDE))
                    .wrapping_add(ROW_WORD) as *const u32).read_unaligned();
        lf_checker_rt::callee_stdcall!(DISPATCH_CALLEE, u32, word, req, 0)
    }
});
