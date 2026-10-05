// original: 0x009aa3b0 conv_table_lookup_c
/// Look up the sub-table for `key` and return its entry list, if live.
///
/// Asks the conversation hub (stubbed, thiscall/1) for the record of
/// `key`; a null record means no entry. Otherwise the count byte at
/// record `+0x0a` is stored into `*out`, and when it is non-zero the
/// list at `record+0x0b` is returned, else null. Stdcall, two words.
export!(stdcall, rw_009AA3B0(key: u32, out: u32) -> u32 {
    unsafe {
        const HUB: u32 = 0x115d9a0;
        const COUNT_OFF: u32 = 0x0a;
        const LIST_OFF: u32 = 0x0b;
        let rec: u32 = callee_thiscall!(1, u32, relocated(HUB), key);
        if rec == 0 {
            return 0;
        }
        let n = ((rec + COUNT_OFF) as *const u8).read();
        (out as *mut u8).write(n);
        if n == 0 {
            return 0;
        }
        rec + LIST_OFF
    }
});
