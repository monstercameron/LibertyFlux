// original: 0x00abbb60 query_tagged_tree

/// Walk a tagged-pointer tree collecting records whose box overlaps a query.
///
/// Interior nodes (low two tag bits other than 3) descend left or right by
/// comparing two node floats against per-axis bounds; the far child waits on
/// an explicit stack. Leaf nodes (tag 3) hold a run of 32-byte records: each
/// record whose three axis ranges all overlap the query box appends its id
/// word (at record offset 0x0c) to the output list, whose length is clamped
/// to the query's capacity. Every record id is written to the current output
/// slot regardless; only the length increment depends on the overlap test.
/// Always returns 0.
///
/// NaN handling matches the original exactly: a greater-than test with a NaN
/// side counts as overlap (the hardware branch falls through), while a
/// less-than test with a NaN side counts as separation.
export!(cdecl, rs64_abbb60(root: u32, bounds0: *const f32, bounds1: *const f32, query: *mut u8) -> u32 {
    fn rd32(addr: u32) -> u32 {
        unsafe { *(addr as *const u32) }
    }
    fn rdf(addr: u32) -> f32 {
        unsafe { *(addr as *const f32) }
    }
    fn separated(a: f32, b: f32, g: u32) -> u32 {
        // Original `comiss` + branch-if-below-or-equal: NaN falls through
        // to the zero side, exactly like this comparison.
        if a > b { g } else { 0 }
    }
    fn below_or_unordered(a: f32, b: f32) -> bool {
        // Original `comiss` + branch-if-below: taken when ordered-below or
        // when either side is NaN.
        !(a >= b)
    }
    unsafe {
        const LEAF_TAG: u32 = 3;
        const ITEM_STRIDE: u32 = 0x20;
        const PENALTY: u32 = 0x017A_D148;
        if root == 0 {
            return 0;
        }
        let g = *global::<u32>(PENALTY);
        let mut stack = [0u32; 260];
        let mut top = 0usize;
        let mut node = root;
        loop {
            let word = rd32(node);
            if word & 3 == LEAF_TAG {
                let count = rd32(node.wrapping_add(4));
                let end = (word & !3).wrapping_add(count.wrapping_shl(5));
                let mut item = word & !3;
                while item != end {
                    let zx = separated(rdf((query as u32).wrapping_add(8)), rdf(item.wrapping_add(0x18)), g);
                    let zy = separated(rdf((query as u32).wrapping_add(4)), rdf(item.wrapping_add(0x14)), g);
                    let zz = separated(rdf(query as u32), rdf(item.wrapping_add(0x10)), g);
                    let wx = separated(rdf(item.wrapping_add(8)), rdf((query as u32).wrapping_add(0x18)), g);
                    let wy = separated(rdf(item.wrapping_add(4)), rdf((query as u32).wrapping_add(0x14)), g);
                    let wz = separated(rdf(item), rdf((query as u32).wrapping_add(0x10)), g);
                    // The original ORs each axis pair as raw bits and keeps
                    // the record only when all three words compare equal to
                    // positive zero, which holds for +0.0 and -0.0 only.
                    let hit = (zx | wx) & 0x7FFF_FFFF == 0
                        && (zy | wy) & 0x7FFF_FFFF == 0
                        && (zz | wz) & 0x7FFF_FFFF == 0;
                    let id = rd32(item.wrapping_add(0x0c));
                    let out = rd32((query as u32).wrapping_add(0x20));
                    let len = rd32((query as u32).wrapping_add(0x24));
                    *((out.wrapping_add(len.wrapping_shl(2))) as *mut u32) = id;
                    let cap = rd32((query as u32).wrapping_add(0x28));
                    let grown = len.wrapping_add(hit as u32);
                    let next = if (cap as i32) < (grown as i32) { cap } else { grown };
                    *((query as u32).wrapping_add(0x24) as *mut u32) = next;
                    item = item.wrapping_add(ITEM_STRIDE);
                }
            } else {
                let tag = word & 3;
                let along = rdf(node.wrapping_add(8));
                let other = rdf(node.wrapping_add(4));
                let lim0 = *bounds0.add(tag as usize);
                let lim1 = *bounds1.add(tag as usize);
                if below_or_unordered(along, lim0) {
                    if below_or_unordered(lim1, other) {
                        // Pop the stacked far child.
                    } else {
                        node = (word & !3).wrapping_add(0x0c);
                        if node == 0 {
                            return 0;
                        }
                        continue;
                    }
                } else if below_or_unordered(lim1, other) {
                    node = word & !3;
                    if node == 0 {
                        return 0;
                    }
                    continue;
                } else {
                    stack[top] = (word & !3).wrapping_add(0x0c);
                    top += 1;
                    node = word & !3;
                    if node == 0 {
                        return 0;
                    }
                    continue;
                }
            }
            if top == 0 {
                return 0;
            }
            top -= 1;
            node = stack[top];
            if node == 0 {
                return 0;
            }
        }
    }
});
