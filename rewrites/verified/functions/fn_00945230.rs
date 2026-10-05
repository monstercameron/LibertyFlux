// original: 0x00945230 streaming_node_find (proposed)

/// Find a streaming item by ordinal, or by id and occurrence.
///
/// Resolves the list head through the list lookup (one stack argument: the
/// key); a null head returns 0. With a zero id the argument's low word is
/// an ordinal: nodes are walked via the next link at `+0x3b`, subtracting
/// each node's item count at `+0x3f`, until the ordinal falls inside a
/// node, whose item slot address (`node + (ordinal + 8) * 8`) is returned.
/// With a non-zero id the nodes' 8-byte items from `+0x40` are scanned for
/// the occurrence-th match of the id (occurrence from the low word of the
/// third argument) and the matching item address is returned, or 0.
///
/// Original: 0x00945230 (stdcall, three stack arguments; callee pops 12).
lf_checker_rt::export!(stdcall, rw_00945230(key: u32, id: u32, occ: u32) -> u32 {
    unsafe {
        const NEXT: u32 = 0x3B;
        const COUNT: u32 = 0x3F;
        const ITEMS: u32 = 0x40;
        const ITEM_STRIDE: u32 = 8;
        const ORDINAL_BIAS: u32 = 8;
        const CALLEE: u32 = 1;
        let mut node: u32 = lf_checker_rt::callee_stdcall!(CALLEE, u32, key);
        if node == 0 {
            return 0;
        }
        if id == 0 {
            let mut ordinal = occ as u16 as u32;
            loop {
                let count = ((node + COUNT) as *const u8).read() as u32;
                if ordinal < count {
                    return node + (ordinal + ORDINAL_BIAS) * ITEM_STRIDE;
                }
                ordinal -= count;
                node = ((node + NEXT) as *const u32).read_unaligned();
                if node == 0 {
                    return 0;
                }
            }
        } else {
            let want = occ as u16 as u32;
            let mut matched = 0u32;
            loop {
                let count = ((node + COUNT) as *const u8).read() as u32;
                let mut i = 0u32;
                while i < count {
                    let item = node + ITEMS + i * ITEM_STRIDE;
                    if ((item) as *const u32).read_unaligned() == id {
                        if matched == want {
                            return item;
                        }
                        matched += 1;
                    }
                    i += 1;
                }
                node = ((node + NEXT) as *const u32).read_unaligned();
                if node == 0 {
                    return 0;
                }
            }
        }
    }
});
