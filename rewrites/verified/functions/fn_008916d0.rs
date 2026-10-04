// original: 0x008916d0 aud_chain_store_tag2
/// Walks the object chain while the tag at 0x70 (bits 2-3) reads 2, storing
/// each visited node to the current-node global. Returns the final tag, or
/// the admitting tag when a link byte of 0xff ends the walk.
export!(thiscall, rw_008916d0(this: *mut u8) -> u32 {
    unsafe {
        *global::<u32>(0x115d898) = this as u32;
        let tag0 = ((*((this.add(0x70)) as *const u32) >> 2) & 3) as u8;
        if tag0 != 2 {
            return tag0 as u32;
        }
        let stride = *global::<u32>(0x115d964);
        let table = *global::<u32>(0x115d988);
        let mut node = this as u32;
        let mut tag = tag0;
        loop {
            // The link load targets DL, so EAX still holds the admitting tag
            // (always 2 here) when a 0xff link ends the walk.
            let link = *((node + 5) as *const u8);
            if link == 0xff {
                return tag as u32;
            }
            let idx = *((node + 0x40) as *const u8) as u32;
            let entry = *((table
                .wrapping_add(idx.wrapping_mul(0x6f40))
                .wrapping_add(0x6f10)) as *const u32);
            node = stride.wrapping_mul(link as u32).wrapping_add(entry);
            *global::<u32>(0x115d898) = node;
            tag = ((*((node + 0x70) as *const u32) >> 2) & 3) as u8;
            if tag != 2 {
                return tag as u32;
            }
        }
    }
});
