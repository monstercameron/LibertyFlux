// original: 0x00937D20 stream_list_flag_value_clear (proposed)

/// Walk the streaming node list, keeping the value of the last flagged node.
///
/// Reads the list head from its global; each node links through dword at
/// `+0x00`, carries a flag byte at `+0x108` and a value byte at `+0x109`.
/// Starts with 1 and, for each node whose flag is zero,
/// replaces the answer with that node's value. An empty list answers 1.
/// Returns the byte in AL (upper bits of EAX are the entry value's).
lf_checker_rt::export!(cdecl, rw_00937d20() -> u32 {
    unsafe {
        const HEAD_GLOBAL: u32 = 0x11A4EE4;
        const NEXT: u32 = 0x00;
        const FLAG: u32 = 0x108;
        const VALUE: u32 = 0x109;
        let mut node = lf_checker_rt::global::<u32>(HEAD_GLOBAL).read_unaligned();
        let mut answer: u8 = 1;
        while node != 0 {
            if ((node + FLAG) as *const u8).read() == 0 {
                answer = ((node + VALUE) as *const u8).read();
            }
            node = ((node + NEXT) as *const u32).read_unaligned();
        }
        answer as u32
    }
});
