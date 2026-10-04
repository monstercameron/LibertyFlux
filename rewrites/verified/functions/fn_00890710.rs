// original: 0x00890710 audio_chain_find_fl20
/// Walk the chain to the first node whose tag node has flag bit 5 set.
///
/// Follows link bytes like [`rw_00890680`], but at each node first checks the
/// tag node (second row table) for flag bit 5 at +0xe7 and stops there when
/// set. Returns the node it stopped at, or this object when this object's own
/// link is 0xff.
export!(thiscall, rw_00890710(this: u32) -> u32 {
    unsafe {
        let first = ((this + 5) as *const u8).read();
        let mut node = if first == 0xff {
            0
        } else {
            let variant = ((this + 0x40) as *const u8).read() as u32;
            let stride = *global::<u32>(0x0115D964);
            let base = *global::<u32>(0x0115D988);
            let row = ((base
                .wrapping_add(variant.wrapping_mul(0x6f40))
                .wrapping_add(0x6f10)) as *const u32)
                .read();
            stride.wrapping_mul(first as u32).wrapping_add(row)
        };
        if node == 0 {
            return this;
        }
        loop {
            let tag = ((node + 4) as *const u8).read();
            if tag != 0xff {
                let variant = ((node + 0x40) as *const u8).read() as u32;
                let stride = *global::<u32>(0x0115D968);
                let base = *global::<u32>(0x0115D988);
                let row = ((base
                    .wrapping_add(variant.wrapping_mul(0x6f40))
                    .wrapping_add(0x6f14)) as *const u32)
                    .read();
                let tnode = stride.wrapping_mul(tag as u32).wrapping_add(row);
                if ((((tnode + 0xe7) as *const u8).read()) & 0x20) != 0 {
                    return node;
                }
            }
            let link = ((node + 5) as *const u8).read();
            if link == 0xff {
                return node;
            }
            let variant = ((node + 0x40) as *const u8).read() as u32;
            let stride = *global::<u32>(0x0115D964);
            let base = *global::<u32>(0x0115D988);
            let row = ((base
                .wrapping_add(variant.wrapping_mul(0x6f40))
                .wrapping_add(0x6f10)) as *const u32)
                .read();
            let next = stride.wrapping_mul(link as u32).wrapping_add(row);
            if next == 0 {
                return node;
            }
            node = next;
        }
    }
});
