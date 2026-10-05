// original: 0x009aa140 conv_table_lookup_a
/// Look up the sub-table for `key` and return its entry list, if live.
///
/// Asks the conversation hub (stubbed, thiscall/1 with the hub pointer)
/// for the record of `key`; a null record means no entry. Otherwise the
/// count byte at record `+0x0a` selects a slot at `record + count*8 + 0x0b`,
/// whose byte selects a group at `slot + group*8`; the live byte at
/// `group+1` is stored into `*out`, and when it is non-zero the list at
/// `group+2` is returned, else null. Stdcall, two stack words.
export!(stdcall, rw_009AA140(key: u32, out: u32) -> u32 {
    unsafe {
        const HUB: u32 = 0x115d9a0;
        const COUNT_OFF: u32 = 0x0a;
        const SLOT_BASE: u32 = 0x0b;
        const LIVE_OFF: u32 = 1;
        const LIST_OFF: u32 = 2;
        const GROUP_STRIDE: u32 = 8;
        let rec: u32 = callee_thiscall!(1, u32, relocated(HUB), key);
        if rec == 0 {
            return 0;
        }
        let n = ((rec + COUNT_OFF) as *const u8).read() as u32;
        let slot = rec + n * GROUP_STRIDE + SLOT_BASE;
        let m = (slot as *const u8).read() as u32;
        let group = slot + m * GROUP_STRIDE;
        let live = ((group + LIVE_OFF) as *const u8).read();
        (out as *mut u8).write(live);
        if live == 0 {
            return 0;
        }
        group + LIST_OFF
    }
});
