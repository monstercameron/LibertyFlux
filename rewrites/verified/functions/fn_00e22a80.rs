// original: 0x00E22A80 task_thunk_d2277e3a (proposed)

/// Cached-dispatch thunk for handler id `d2277e3a`.
///
/// Calls the setup routine once; a nonzero setup result other than
/// `RESET_CACHE` returns at once, while `RESET_CACHE` clears this thunk's
/// handler cache (`CACHE`) first. An empty cache is filled by asking the
/// lookup slot for `HASH` when the setup context is nonzero; a still-empty
/// cache returns `NOT_FOUND`. Otherwise the optional pre-hook runs, the
/// cached handler receives this thunk's 3 argument(s) unchanged, the
/// optional post-hook observes (`HASH`, pre-hook outputs, handler result),
/// and slot 0 of the use table is decremented. The lookup, pre-hook,
/// handler and post-hook addresses all come from game data slots that the
/// checker fabricates; the rewrite loads and calls them exactly as the
/// original does.
///
/// Original: 0x00A22A80 (cdecl, 3 stack word(s), eax result).
lf_checker_rt::export!(cdecl, rw_00a22a80(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const HASH: u32 = 0xd2277e3a;
        const CACHE: u32 = 0x017AC72C;
        const LOOKUP_SLOT: u32 = 0x17AC464;
        const PRE_SLOT: u32 = 0x17AC46C;
        const POST_SLOT: u32 = 0x17AC470;
        const USE_TABLE: u32 = 0x17AC498;
        const RESET_CACHE: u32 = 0xFFFF_FFF2;
        const NOT_FOUND: u32 = 0xFFFF_FFFD;
        const SETUP_CALLEE: u32 = 1;

        let mut setup_ctx: u32 = 0;
        let table_slot: u32 = 0; // never written after init, like the original's local
        let setup: u32 = lf_checker_rt::callee_cdecl!(
            SETUP_CALLEE, u32, &mut setup_ctx as *mut u32 as u32);
        let mut result = setup;
        if result != 0 && result != RESET_CACHE {
            return result;
        }
        if result == RESET_CACHE {
            lf_checker_rt::global::<u32>(CACHE).write(0);
        }
        let mut handler = lf_checker_rt::global::<u32>(CACHE).read();
        if handler == 0 {
            if setup_ctx != 0 {
                let lookup = lf_checker_rt::global::<u32>(LOOKUP_SLOT).read();
                if lookup != 0 {
                    let find: extern "cdecl" fn(u32) -> u32 =
                        core::mem::transmute(lookup as usize);
                    handler = find(HASH);
                    lf_checker_rt::global::<u32>(CACHE).write(handler);
                }
            }
        }
        if handler == 0 {
            return NOT_FOUND;
        }
        // One array: the callee writes two words here, so the layout must
        // be two adjacent zeroed words, like the original's locals.
        let mut pre_out = [0u32; 2];
        let pre = lf_checker_rt::global::<u32>(PRE_SLOT).read();
        if pre != 0 {
            let run_pre: extern "cdecl" fn(u32, u32) -> u32 =
                core::mem::transmute(pre as usize);
            run_pre(HASH, pre_out.as_mut_ptr() as u32);
        }
        let run: extern "cdecl" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(handler as usize);
        result = run(a0, a1, a2);
        let post = lf_checker_rt::global::<u32>(POST_SLOT).read();
        if post != 0 {
            let run_post: extern "cdecl" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(post as usize);
            run_post(HASH, pre_out[0], pre_out[1], result);
        }
        let cell = lf_checker_rt::global::<u32>(USE_TABLE).add(table_slot as usize);
        cell.write(cell.read().wrapping_sub(1));
        result
    }
});
