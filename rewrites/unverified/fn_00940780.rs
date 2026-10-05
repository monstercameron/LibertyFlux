// original: 0x00940780 streaming_entity_release (proposed)

/// Release the streaming entity down the selected path, then detach it.
///
/// Calls the entity's virtual release slot at `+0x18`, or at `+0x20` when
/// bit 27 of the flags word at `obj + 0x24` is set (thiscall, `this` is the
/// object). When bit 25 of the flags is set the reaper worker runs next
/// (thiscall, no stack arguments). A zero flag argument then tail-calls the
/// virtual detach slot at `+0xb0` and returns its answer; a non-zero flag
/// returns with the reaper's answer, or the flags word right-shifted by 25
/// (the shift the bit test leaves behind) when it did not run.
///
/// Original: 0x00940780 (cdecl, two stack arguments: object and flag).
lf_checker_rt::export!(cdecl, rw_00940780(obj: u32, flag: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x24;
        const ALT_PATH: u32 = 0x0800_0000;
        const REAPER_BIT: u32 = 25;
        const VT_RELEASE: u32 = 0x18;
        const VT_RELEASE_ALT: u32 = 0x20;
        const VT_DETACH: u32 = 0xB0;
        const REAPER: u32 = 3;
        let vt = (obj as *const u32).read_unaligned();
        let flags = ((obj + FLAGS) as *const u32).read_unaligned();
        if flags & ALT_PATH != 0 {
            let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                ((vt + VT_RELEASE_ALT) as *const u32).read_unaligned() as usize,
            );
            f(obj);
        } else {
            let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                ((vt + VT_RELEASE) as *const u32).read_unaligned() as usize,
            );
            f(obj);
        }
        let mut answer = flags >> REAPER_BIT;
        if (flags >> REAPER_BIT) & 1 != 0 {
            answer = lf_checker_rt::callee_thiscall!(REAPER, u32, obj);
        }
        if flag as u8 == 0 {
            let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                ((vt + VT_DETACH) as *const u32).read_unaligned() as usize,
            );
            return f(obj);
        }
        answer
    }
});
