// original: 0x00890680 audio_chain_tail
/// Follow link bytes through the first row table to the end of the chain.
///
/// Each record's link byte at +5 selects the next node; a 0xff link ends the
/// walk and the current node is returned. A 0xff link on this object itself
/// returns this object.
export!(thiscall, rw_00890680(this: u32) -> u32 {
    unsafe {
        let first = ((this + 5) as *const u8).read();
        if first == 0xff {
            return this;
        }
        let variant = ((this + 0x40) as *const u8).read() as u32;
        let stride = *global::<u32>(0x0115D964);
        let base = *global::<u32>(0x0115D988);
        let row = ((base
            .wrapping_add(variant.wrapping_mul(0x6f40))
            .wrapping_add(0x6f10)) as *const u32)
            .read();
        let mut node = stride.wrapping_mul(first as u32).wrapping_add(row);
        if node == 0 {
            return this;
        }
        loop {
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
