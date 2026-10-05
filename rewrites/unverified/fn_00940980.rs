// original: 0x00940980 streaming_state_dispatch (proposed)

/// Dispatch the streaming request on its 4-bit tag.
///
/// The tag at bits 6..9 of the word at `obj + 0x28` selects the arm
/// (anything outside 2..=6 shares the default arm): tag 2 runs the global
/// object's virtual slot `+0x8` with the object as its argument (thiscall,
/// `this` from the global); tags 3 and 6 queue the object with priority 1
/// (cdecl, two arguments); tag 4 runs the linked object's virtual slot 0
/// with argument 1 when its mask word is clear (thiscall), then the two
/// follow-up workers (thiscall with no arguments, then cdecl with the
/// object); tag 5 and the default arm resolve through the release worker
/// (cdecl, the object and 0) and then run the object's own virtual slot 0
/// with argument 1 (thiscall). Returns the last answer in `eax`.
///
/// Original: 0x00940980 (cdecl, one stack argument).
lf_checker_rt::export!(cdecl, rw_00940980(obj: u32) -> u32 {
    unsafe {
        const TAG_WORD: u32 = 0x28;
        const LINK: u32 = 0x27C;
        const LINK_MASK: u32 = 0x5C;
        const LINK_LIVE: u32 = 0x1FFF;
        const VT_RUN: u32 = 0x0;
        const VT_GLOBAL_RUN: u32 = 0x8;
        const GLOBAL_OBJ: u32 = 0x0166D9FC;
        const G_RUN: u32 = 1;
        const QUEUE: u32 = 2;
        const L_RUN: u32 = 3;
        const FOLLOW_A: u32 = 4;
        const FOLLOW_B: u32 = 5;
        const RELEASE: u32 = 6;
        const O_RUN: u32 = 7;
        let tag = ((((obj + TAG_WORD) as *const u32).read_unaligned() >> 6) & 0xF) as u32;
        let idx = tag.wrapping_sub(2);
        if idx == 0 {
            let global = lf_checker_rt::global::<u32>(GLOBAL_OBJ).read();
            let vt = (global as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                ((vt + VT_GLOBAL_RUN) as *const u32).read_unaligned() as usize,
            );
            return f(global, obj);
        }
        if idx == 1 || idx == 4 {
            return lf_checker_rt::callee_cdecl!(QUEUE, u32, obj, 1u32);
        }
        if idx == 2 {
            let link = ((obj + LINK) as *const u32).read_unaligned();
            if ((link + LINK_MASK) as *const u16).read_unaligned() as u32 & LINK_LIVE == 0 {
                let vt = (link as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                    ((vt + VT_RUN) as *const u32).read_unaligned() as usize,
                );
                f(link, 1);
            }
            let _: u32 = lf_checker_rt::callee_thiscall!(FOLLOW_A, u32, obj);
            return lf_checker_rt::callee_cdecl!(FOLLOW_B, u32, obj);
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(RELEASE, u32, obj, 0u32);
        let vt = (obj as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
            ((vt + VT_RUN) as *const u32).read_unaligned() as usize,
        );
        f(obj, 1)
    }
});
