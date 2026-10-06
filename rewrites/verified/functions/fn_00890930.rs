// original: 0x00890930 audsound_follow_chain_and_emit
/// Follows the +5 link chain, then emits the resolved voice pointer.
///
/// Starting at `this`, repeatedly replaces the cursor with
/// `stride * [cursor+5] + table[idx * 0x6f40 + 0x6f10]` (`stride`/`table`
/// from the globals, `idx` the byte at `[cursor+0x40]`, wrapping unsigned;
/// a 0xff link instead resolves to null) until `[cursor+5]` reads 0xff. Then
/// resolves once more through the sibling column: null when `[cursor+4]` is
/// 0xff, otherwise `stride2 * [cursor+4] + table[idx * 0x6f40 + 0x6f14]`, and
/// calls the emit callee (cdecl, one stack word) with the result plus 0xd8.
/// The chain has no length cap; the contract keeps it to at most one hop.
/// Original: 0x00890930 (thiscall, no stack arguments).
export!(thiscall, rw_00890930(this: *mut u8) -> u32 {
    unsafe {
        const EMIT: u32 = 1;
        const LINK: usize = 5;
        const SLOT: usize = 4;
        const INDEX: usize = 0x40;
        const ROW: u32 = 0x6f40;
        const COL_A: u32 = 0x6f10;
        const COL_B: u32 = 0x6f14;
        const EMIT_OFF: u32 = 0xd8;
        const END: u8 = 0xff;
        const STRIDE_G: u32 = 0x115d964;
        const STRIDE2_G: u32 = 0x115d968;
        const TABLE_G: u32 = 0x115d988;
        let stride = *global::<u32>(STRIDE_G);
        let table = *global::<u32>(TABLE_G);
        let mut cur = this as u32;
        let mut link = *((cur as *const u8).add(LINK));
        if link != END {
            loop {
                let idx = *((cur as *const u8).add(INDEX)) as u32;
                let base = *((table.wrapping_add(idx.wrapping_mul(ROW)).wrapping_add(COL_A))
                    as *const u32);
                cur = if link == END {
                    0
                } else {
                    stride.wrapping_mul(link as u32).wrapping_add(base)
                };
                link = *((cur as *const u8).add(LINK));
                if link == END {
                    break;
                }
            }
        }
        let stride2 = *global::<u32>(STRIDE2_G);
        let slot = *((cur as *const u8).add(SLOT));
        let target = if slot == END {
            0
        } else {
            let idx = *((cur as *const u8).add(INDEX)) as u32;
            let base = *((table.wrapping_add(idx.wrapping_mul(ROW)).wrapping_add(COL_B))
                as *const u32);
            stride2.wrapping_mul(slot as u32).wrapping_add(base)
        };
        callee_cdecl!(EMIT, u32, target.wrapping_add(EMIT_OFF))
    }
});
