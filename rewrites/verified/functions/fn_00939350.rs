// original: 0x00939350 stream_list_find_by_id (proposed)

/// Find the first node in the streaming list whose id matches.
///
/// Reads the list head from its global; each node links through dword at
/// `+0x00` and carries its id at `+0x0C`. Returns the matching node's
/// address, or 0 when the list is empty or no id matches. Reads only.
lf_checker_rt::export!(cdecl, rw_00939350(wanted: u32) -> u32 {
    unsafe {
        const HEAD_GLOBAL: u32 = 0x11A4EE0;
        const NEXT: u32 = 0x00;
        const ID: u32 = 0x0C;
        let mut node = lf_checker_rt::global::<u32>(HEAD_GLOBAL).read_unaligned();
        while node != 0 {
            if ((node + ID) as *const u32).read_unaligned() == wanted {
                return node;
            }
            node = ((node + NEXT) as *const u32).read_unaligned();
        }
        0
    }
});
