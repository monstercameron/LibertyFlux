// original: 0x008908b0 audio_chain_cond_release
/// Walk to the chain end and release its info block when non-empty.
///
/// Follows link bytes to the terminal node, resolves its info block through
/// the second row table plus 0xd8, and when the block's counter is nonzero
/// passes the block to the release helper (stubbed by the checker). Returns
/// the helper's answer, or the row-table index expression the original leaves
/// in EAX when the counter is zero.
export!(thiscall, rw_008908b0(this: u32) -> u32 {
    unsafe {
        let mut cur = this;
        if ((cur + 5) as *const u8).read() != 0xff {
            loop {
                let link = ((cur + 5) as *const u8).read();
                let variant = ((cur + 0x40) as *const u8).read() as u32;
                let stride = *global::<u32>(0x0115D964);
                let base = *global::<u32>(0x0115D988);
                let row = ((base
                    .wrapping_add(variant.wrapping_mul(0x6f40))
                    .wrapping_add(0x6f10)) as *const u32)
                    .read();
                cur = stride.wrapping_mul(link as u32).wrapping_add(row);
                if ((cur + 5) as *const u8).read() == 0xff {
                    break;
                }
            }
        }
        let tag = ((cur + 4) as *const u8).read();
        let variant = ((cur + 0x40) as *const u8).read() as u32;
        let (node, index_eax) = if tag == 0xff {
            (0, 0xff)
        } else {
            let stride = *global::<u32>(0x0115D968);
            let base = *global::<u32>(0x0115D988);
            let row = ((base
                .wrapping_add(variant.wrapping_mul(0x6f40))
                .wrapping_add(0x6f14)) as *const u32)
                .read();
            (
                stride.wrapping_mul(tag as u32).wrapping_add(row),
                variant.wrapping_mul(0x6f40),
            )
        };
        let block = node.wrapping_add(0xd8);
        if ((block as *const u32).read()) == 0 {
            index_eax
        } else {
            callee_cdecl!(1, u32, block)
        }
    }
});
