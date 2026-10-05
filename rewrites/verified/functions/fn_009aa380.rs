// original: 0x009aa380 conv_table_first
/// Return the head word of the sub-table for `key`, if it is non-empty.
///
/// Asks the conversation hub (stubbed, thiscall/1) for the record of
/// `key`. When the record exists and its count byte at `+0x0a` is
/// non-zero, returns the unaligned dword at record `+0x0b`; otherwise
/// returns null. Stdcall, one stack word.
export!(stdcall, rw_009AA380(key: u32) -> u32 {
    unsafe {
        const HUB: u32 = 0x115d9a0;
        const COUNT_OFF: u32 = 0x0a;
        const HEAD_OFF: u32 = 0x0b;
        let rec: u32 = callee_thiscall!(1, u32, relocated(HUB), key);
        if rec == 0 {
            return 0;
        }
        if ((rec + COUNT_OFF) as *const u8).read() == 0 {
            return 0;
        }
        ((rec + HEAD_OFF) as *const u32).read_unaligned()
    }
});
