// original: 0x009A6940 audio_dispatch_entry (proposed)

/// Dispatches an entry through an indexed row table, then marks it done.
///
/// thiscall, two stack bytes (`quick`, `consume`). When the flag byte at
/// `FLAG` (+8) of `this` is clear, returns at once (the return register
/// keeps caller leftovers on this path, so the contract compares no
/// return channel). When `quick` is set, the middle lookup is skipped.
/// Otherwise the signed 16-bit index at `CUR` (+0x2BD0) is read: a
/// negative index skips the lookup, while a non-negative one (zero
/// included) selects row dword `ROW` (+0x15EC) + index * `ROW_STRIDE`
/// (0x70), maps it through `MAP` (+0x2DF0) + row * 4, and calls the
/// row handler (callee 1, thiscall) on the mapped address (+ `EXTRA`
/// 0x570), or on `this + FALLBACK` (+0x2BE0) when the mapped address is
/// null. When `consume` is set the index is decremented (wrapping), the
/// done byte at `DONE` (+0x2BD4) is set to 1, and the closer (callee 2,
/// thiscall on `this`, one zero word) runs.
lf_checker_rt::export!(thiscall, rw_009a6940(this: u32, quick: u32, consume: u32) -> u32 {
    unsafe {
        const HANDLER: u32 = 1;
        const CLOSER: u32 = 2;
        const FLAG: u32 = 8;
        const CUR: u32 = 0x2BD0;
        const DONE: u32 = 0x2BD4;
        const ROW: u32 = 0x15EC;
        const ROW_STRIDE: u32 = 0x70;
        const MAP: u32 = 0x2DF0;
        const FALLBACK: u32 = 0x2BE0;
        const EXTRA: u32 = 0x570;
        if (this.wrapping_add(FLAG) as *const u8).read() == 0 {
            return 0;
        }
        if (quick as u8) == 0 {
            let idx = (this.wrapping_add(CUR) as *const u16).read_unaligned() as i16 as i32;
            if idx >= 0 {
                let row = (this
                    .wrapping_add(ROW)
                    .wrapping_add((idx as u32).wrapping_mul(ROW_STRIDE))
                    as *const u32)
                    .read_unaligned();
                let target =
                    (this.wrapping_add(MAP).wrapping_add(row.wrapping_mul(4)) as *const u32)
                        .read_unaligned();
                let target = if target == 0 {
                    this.wrapping_add(FALLBACK)
                } else {
                    target.wrapping_add(EXTRA)
                };
                let _: u32 = lf_checker_rt::callee_thiscall!(HANDLER, u32, target);
            }
        }
        if (consume as u8) != 0 {
            let slot = this.wrapping_add(CUR) as *mut u16;
            slot.write_unaligned(slot.read_unaligned().wrapping_sub(1));
        }
        (this.wrapping_add(DONE) as *mut u8).write(1);
        let _: u32 = lf_checker_rt::callee_thiscall!(CLOSER, u32, this, 0);
    }
    0
});
