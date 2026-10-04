// original: 0x008901e0 audio_walk_cache_chain
/// Walks the audio cache chain to the terminal value.
///
/// Follows entity-table links from this node until a node with an empty
/// (`0xFF`) selector is found. Nodes in mode 1 resolve their word at
/// `0x7E` through the cache helper on first visit. Returns the resolved
/// value carried along the chain (or the argument when no mode-1 node
/// contributed one).
export!(thiscall, rw_008901e0(this_ptr: *mut u8, default: u32) -> u32 {
    unsafe {
        let mut node = this_ptr;
        let mut acc = default;
        loop {
            let mode = ((*(node.add(0x70) as *const u32) >> 12) & 3) as u8;
            if mode == 1 {
                let cached = *(node.add(0x7E) as *const u16);
                if cached != 0 {
                    acc = cached as u32;
                } else {
                    let sel = *node.add(0x40) as u32;
                    let ans: u32 = callee_thiscall!(
                        1,
                        u32,
                        relocated(0x0115D8A0),
                        sel
                    );
                    *(node.add(0x7E) as *mut u16) = ans as u16;
                    acc = (ans & 0xFFFF) as u32;
                }
            }
            let next_sel = *node.add(5);
            if next_sel == 0xFF {
                return acc;
            }
            let stride = *(relocated(0x0115D964) as *const u32);
            let base = *(relocated(0x0115D988) as *const u32) as *const u8;
            let row_sel = *node.add(0x40) as u32;
            let row = base.add(row_sel.wrapping_mul(0x6F40) as usize);
            node = stride
                .wrapping_mul(next_sel as u32)
                .wrapping_add(*(row.add(0x6F10) as *const u32))
                as *mut u8;
        }
    }
});
