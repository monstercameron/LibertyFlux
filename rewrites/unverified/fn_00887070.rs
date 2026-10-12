// original: 0x00887070 audio_config_unload (proposed)

/// Release an audio configuration and the shared config document.
///
/// `this` points to the config: frees the document at `+0` through its
/// releaser and the allocator (patched callees) and nulls the slot, clears
/// `+4`, then, when the global document at 0x0115A520 is set and its
/// can-unload check (patched callee) agrees, tears it down (patched callee),
/// frees it and nulls the global. Returns the last callee's result, or the
/// pinned entry eax when no call ran (the original's eax in each case).
///
/// Original: 0x00887070 (thiscall: `this` in ecx, no stack words).
lf_checker_rt::export!(thiscall, rw_00887070(this: u32) -> u32 {
    unsafe {
        const ENTRY_EAX: u32 = 0x5a5a_5a5a;
        const DOC_RELEASE: u32 = 1;
        const ALLOC_FREE: u32 = 2;
        const CAN_UNLOAD: u32 = 3;
        const DOC_TEARDOWN: u32 = 4;
        const DOC_SLOT: u32 = 0x0115_a520;

        let mut ret = ENTRY_EAX;
        let doc = (this as *const u32).read_unaligned();
        if doc != 0 {
            lf_checker_rt::callee_thiscall!(DOC_RELEASE, u32, doc);
            ret = lf_checker_rt::callee_cdecl!(ALLOC_FREE, u32, doc);
            (this as *mut u32).write_unaligned(0);
        }
        if ((this + 4) as *const u32).read_unaligned() != 0 {
            ((this + 4) as *mut u32).write_unaligned(0);
        }
        let shared = lf_checker_rt::global::<u32>(DOC_SLOT).read();
        if shared != 0 {
            let ok = lf_checker_rt::callee_thiscall!(CAN_UNLOAD, u32, shared);
            ret = ok;
            if (ok & 0xff) != 0 {
                let shared2 = lf_checker_rt::global::<u32>(DOC_SLOT).read();
                if shared2 != 0 {
                    lf_checker_rt::callee_thiscall!(DOC_TEARDOWN, u32, shared2);
                    ret = lf_checker_rt::callee_cdecl!(ALLOC_FREE, u32, shared2);
                }
                lf_checker_rt::global::<u32>(DOC_SLOT).write(0);
            }
        }
        ret
    }
});
