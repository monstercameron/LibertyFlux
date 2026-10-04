// original: 0x00622770 net_session_dispatch_keyed
/// Dispatch a keyed request when session and key headers match.
///
/// Returns without calling when the state is not 2 or 3, the id pairs do
/// not match, or the key header words differ from `this+0x540`/`+0x544`.
/// Otherwise resolves the key tail and dispatches it with `a1`.
export!(thiscall, rw_00622770(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        let base = this as *const u8;
        let r = |off: usize| (base.add(off) as *const u32).read_unaligned();
        let state = r(0x50);
        if state < 2 || state > 3 {
            return 0;
        }
        if r(0xbf0) != r(0xc30) || r(0xbf4) != r(0xc34) {
            return 0;
        }
        if r(0x540) != (a0 as *const u32).read_unaligned()
            || r(0x544) != ((a0 as *const u32).add(1)).read_unaligned()
        {
            return 0;
        }
        let found: u32 =
            callee_thiscall!(1, u32, this, ((a0 as *const u8).add(8) as u32));
        if found == 0 {
            return 0;
        }
        let _: u32 = callee_thiscall!(2, u32, this, found.wrapping_add(0x48), a1, 0, 0);
        0
    }
});
