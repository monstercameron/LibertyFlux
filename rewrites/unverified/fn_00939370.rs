// original: 0x00939370 stream_list_find_or_default (proposed)

/// Find a streaming node by id, else answer a shared default or null.
///
/// Searches the global node list (link at `+0x00`, id at `+0x0C`) for
/// `wanted`. A match returns the node address plus `0x20E` (pointer
/// arithmetic only). With no match, a zero `use_default` flag answers 0
/// and a nonzero flag answers the shared default record. Reads only.
lf_checker_rt::export!(cdecl, rw_00939370(wanted: u32, use_default: u32) -> u32 {
    unsafe {
        const HEAD_GLOBAL: u32 = 0x11A4EE0;
        const NEXT: u32 = 0x00;
        const ID: u32 = 0x0C;
        const RECORD_SKIP: u32 = 0x20E;
        const DEFAULT_RECORD: u32 = 0xE87D28;
        let mut node = lf_checker_rt::global::<u32>(HEAD_GLOBAL).read_unaligned();
        while node != 0 {
            if ((node + ID) as *const u32).read_unaligned() == wanted {
                return node.wrapping_add(RECORD_SKIP);
            }
            node = ((node + NEXT) as *const u32).read_unaligned();
        }
        if (use_default & 0xFF) == 0 {
            0
        } else {
            lf_checker_rt::relocated(DEFAULT_RECORD)
        }
    }
});
