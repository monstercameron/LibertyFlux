// original: 0x00953230 chain_search_u32
/// Search the node chain for the first entry covering `key`.
///
/// Each node holds a data pointer at +4 and the next node at +8. The walk
/// snapshots each data block's first 32 bytes and compares its fourth word
/// against the key; it stops with the head's data when the chain ends and
/// with the current node's data when the next block overshoots the key.
export!(cdecl, rw_00953230(key: u32) -> u32 {
    unsafe {
        let head = *global::<u32>(0x11FF028);
        let mut node = head;
        loop {
            let next = *((node.wrapping_add(8)) as *const u32);
            if next == 0 {
                return *((head.wrapping_add(4)) as *const u32);
            }
            let data = *((node.wrapping_add(4)) as *const u32);
            let mut window = [0u32; 8];
            core::ptr::copy_nonoverlapping(
                data as *const u32,
                window.as_mut_ptr(),
                8,
            );
            core::hint::black_box(window);
            if window[3] > key {
                node = next;
                continue;
            }
            let next_data = *((next.wrapping_add(4)) as *const u32);
            if *((next_data.wrapping_add(12)) as *const u32) > key {
                return *((node.wrapping_add(4)) as *const u32);
            }
            node = next;
        }
    }
});
