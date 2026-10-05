// original: 0x009a9af0 conv_state_advance
/// Advance row `row`'s conversation slot, retiring it when asked.
///
/// The tag byte at `this + row + 0x368` plus three times the row,
/// tripled and scaled by 32, selects two adjacent slot groups. The
/// first group's word at `+0x0c` is cleared, then its pointer at
/// `+0x10` is loaded: when the pointed flag word is already set, or
/// when `retire` is clear, the row advances (below); otherwise the
/// group's byte at `+0x08` is cleared and the function returns.
/// Advancing sets that byte to 2, stores the hub pointer (the global
/// at 0x1038eb8, or null when `fresh` is set) plus the global base at
/// 0x11735b4 into the second group, clears the first group's word at
/// `+0x5c`, notifies the row watcher (stubbed, thiscall/2 with
/// `(row, tag)`), and rotates the tag byte to `(tag+1) mod 3`.
/// Thiscall, three stack words, no result.
export!(thiscall, rw_009A9AF0(this: u32, row: u32, retire: u32, fresh: u32) -> u32 {
    unsafe {
        const TAG_TABLE: u32 = 0x368;
        const HUB_GLOBAL: u32 = 0x1038eb8;
        const BASE_GLOBAL: u32 = 0x11735b4;
        let tagp = this + row + TAG_TABLE;
        let tag = (tagp as *const u8).read() as u32;
        let g0 = this + (tag + row * 3) * 3 * 32;
        ((g0 + 0x0c) as *mut u32).write_unaligned(0);
        let slot = ((g0 + 0x10) as *const u32).read_unaligned();
        let flagged = ((slot + 8) as *const u32).read_unaligned() != 0;
        if !flagged && retire as u8 != 0 {
            ((g0 + 0x08) as *mut u8).write(0);
            return 0;
        }
        ((g0 + 0x08) as *mut u8).write(2);
        let hub = if fresh as u8 != 0 {
            0
        } else {
            global::<u32>(HUB_GLOBAL).read_unaligned()
        };
        let base = global::<u32>(BASE_GLOBAL).read_unaligned();
        let g1 = this + (tag + 1 + row * 3) * 3 * 32;
        (g1 as *mut u32).write_unaligned(hub.wrapping_add(base));
        ((g0 + 0x5c) as *mut u32).write_unaligned(0);
        let _: u32 = callee_thiscall!(1, u32, this, row, tag);
        (tagp as *mut u8).write(((tag + 1) % 3) as u8);
        0
    }
});
