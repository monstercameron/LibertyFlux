// original: 0x00b41be0 box_query_global_array
/// Test whether a query box matches any entry of the global object array.
///
/// Scans the global pointer array: each entry whose coarse box (signed word
/// compares) contains the query box is followed through its node chain, and
/// a node whose six bound words contain the query box yields 1. Returns 0
/// when nothing matches. Only the low byte of the result is defined.
export!(cdecl, rw_b41be0(query: u32) -> u32 {
    unsafe {
        let count = (global::<u16>(0x1665420)).read() as i32;
        let qb = |off: usize| (query as *const i16).byte_add(off).read();
        let q_min0 = qb(2);
        let q_min1 = qb(6);
        let q_min2 = qb(10);
        let q_max0 = qb(0);
        let q_max1 = qb(4);
        let q_max2 = qb(8);
        let mut i: i32 = 0;
        while i < count {
            let arr = (global::<u32>(0x166541c)).read();
            let ent = (arr.wrapping_add((i as u32).wrapping_mul(4)) as *const u32).read();
            let mut node = (ent as *const u32).byte_add(0x18).read();
            if node != 0 {
                let ew = |off: usize| (ent as *const i16).byte_add(off).read();
                if ew(0x0c) <= q_min0
                    && ew(0x10) <= q_min1
                    && ew(0x14) <= q_min2
                    && ew(0x0e) >= q_max0
                    && ew(0x12) >= q_max1
                    && ew(0x16) >= q_max2
                {
                    loop {
                        let nw = |off: usize| (node as *const i16).byte_add(off).read();
                        if nw(0x260) <= q_min0
                            && nw(0x264) <= q_min1
                            && nw(0x268) <= q_min2
                            && nw(0x262) >= q_max0
                            && nw(0x266) >= q_max1
                            && nw(0x26a) >= q_max2
                        {
                            return 1;
                        }
                        node = (node as *const u32).byte_add(0x258).read();
                        if node == 0 {
                            break;
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
        0
    }
});
