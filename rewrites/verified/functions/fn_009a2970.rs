// original: 0x009A2970 audio_select_key_and_cache (proposed)

/// Select a new audio key, releasing the old one, and cache the resolved word.
///
/// When `this`+0x9C already equals `new` nothing is released. Otherwise the
/// id helper (callee 1) and the release call (callee 2, thiscall/2 of the id and the incoming key on the shared audio manager) run,
/// and when the old
/// key was nonzero a second id plus the manager detach (callee 3,
/// thiscall/2) run. The new key is stored, validated by callee 4 (cdecl/1,
/// nonzero means valid, falling back to a global default), resolved by
/// callee 5 (thiscall/1 on the manager), and the low 16 bits of the answer
/// are cached at `this`+0xC8. Returns the resolver's full answer.
lf_checker_rt::export!(thiscall, rw_009A2970(this: u32, new: u32) -> u32 {
    unsafe {
        const KEY: u32 = 0x9C;
        const CACHE_WORD: u32 = 0xC8;
        const MANAGER: u32 = 0x01288780;
        const DEFAULT_KEY: u32 = 0x012844B4;
        const ID_CALLEE: u32 = 1;
        const RELEASE_CALLEE: u32 = 2;
        const DETACH_CALLEE: u32 = 3;
        const VALID_CALLEE: u32 = 4;
        const RESOLVE_CALLEE: u32 = 5;
        let mgr = lf_checker_rt::relocated(MANAGER);
        let old = ((this.wrapping_add(KEY)) as *const u32).read_unaligned();
        if old != new {
            let id = lf_checker_rt::callee_thiscall!(ID_CALLEE, u32, this, 1);
            lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, mgr, id, new);
            let still = ((this.wrapping_add(KEY)) as *const u32).read_unaligned();
            if still != 0 {
                let id2 = lf_checker_rt::callee_thiscall!(ID_CALLEE, u32, this, 1);
                lf_checker_rt::callee_thiscall!(DETACH_CALLEE, u32, mgr, id2, still);
            }
        }
        ((this.wrapping_add(KEY)) as *mut u32).write_unaligned(new);
        let ok = lf_checker_rt::callee_cdecl!(VALID_CALLEE, u32, new);
        if (ok as u8) == 0 {
            let dflt = (lf_checker_rt::global::<u32>(DEFAULT_KEY) as *const u32).read_unaligned();
            ((this.wrapping_add(KEY)) as *mut u32).write_unaligned(dflt);
        }
        let cur = ((this.wrapping_add(KEY)) as *const u32).read_unaligned();
        let ans = lf_checker_rt::callee_thiscall!(RESOLVE_CALLEE, u32, mgr, cur);
        ((this.wrapping_add(CACHE_WORD)) as *mut u16).write_unaligned(ans as u16);
        ans
    }
});
