// original: 0x009aa200 conv_table_lookup_b
/// Look up the sub-table for `key` and return its live entry list.
///
/// Asks the conversation hub (stubbed, thiscall/1) for the record of
/// `key`; a null record means no entry. Otherwise the count byte at
/// record `+0x0a` selects a group at `record + count*8`, the live byte
/// at `group+0x0b` is stored into `*out`, and when it is non-zero the
/// list at `group+0x0c` is returned, else null. Stdcall, two words.
export!(stdcall, rw_009AA200(key: u32, out: u32) -> u32 {
    unsafe {
        const HUB: u32 = 0x115d9a0;
        const COUNT_OFF: u32 = 0x0a;
        const LIVE_OFF: u32 = 0x0b;
        const LIST_OFF: u32 = 0x0c;
        const GROUP_STRIDE: u32 = 8;
        let rec: u32 = callee_thiscall!(1, u32, relocated(HUB), key);
        if rec == 0 {
            return 0;
        }
        let n = ((rec + COUNT_OFF) as *const u8).read() as u32;
        let group = rec + n * GROUP_STRIDE;
        let live = ((group + LIVE_OFF) as *const u8).read();
        (out as *mut u8).write(live);
        if live == 0 {
            return 0;
        }
        group + LIST_OFF
    }
});
