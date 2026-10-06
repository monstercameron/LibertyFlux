// original: 0x0099DCD0 audSpeechAudioEntity_teardown (proposed)

/// Tear down a speech audio entity and tail-jump to the base teardown.
///
/// Installs the speech table address at +0, then unless the 16-bit marker
/// at +4 is 0xFFFF runs the detach sequence: the ready gate (callee 1,
/// thiscall/0), the id helper (callee 2, thiscall/1 of zero), the manager
/// release (callee 3, thiscall/2 of the id and the key at +0x9C on the
/// shared audio manager), the handle release (callee 4, thiscall/1 of
/// zero) and the manager unregister (callee 5, thiscall/1 of the object).
/// A nonzero link at +0x1F8 is unlinked (callee 6, thiscall/1 of the link
/// address) and cleared; a nonzero buffer at +0x204 is freed (callee 7,
/// cdecl/1) and cleared. The two member teardowns (callee 8, thiscall/0,
/// at +0x120 and +0xD0) run, the base table address goes to +0, and
/// control tail-jumps to the base teardown (callee 9, thiscall/0), whose
/// answer is the return; the rewrite issues it as a final call. Thiscall
/// with no stack words.
lf_checker_rt::export!(thiscall, rw_0099DCD0(this: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x00E91260;
        const BASE_TABLE: u32 = 0x00E83134;
        const MARKER: u32 = 4;
        const FULL_MARKER: u16 = 0xFFFF;
        const KEY: u32 = 0x9C;
        const MEMBER_B: u32 = 0x120;
        const MEMBER_A: u32 = 0xD0;
        const LINK: u32 = 0x1F8;
        const BUFFER: u32 = 0x204;
        const MANAGER: u32 = 0x01288780;
        const GATE: u32 = 1;
        const ID: u32 = 2;
        const RELEASE: u32 = 3;
        const HANDLES: u32 = 4;
        const UNREG: u32 = 5;
        const UNLINK: u32 = 6;
        const FREE: u32 = 7;
        const MEMBER: u32 = 8;
        const TAIL: u32 = 9;
        let mgr = lf_checker_rt::relocated(MANAGER);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(TABLE));
        if (this.wrapping_add(MARKER) as *const u16).read_unaligned() != FULL_MARKER {
            lf_checker_rt::callee_thiscall!(GATE, u32, this);
            let id = lf_checker_rt::callee_thiscall!(ID, u32, this, 0);
            let key = ((this.wrapping_add(KEY)) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(RELEASE, u32, mgr, id, key);
            lf_checker_rt::callee_thiscall!(HANDLES, u32, this, 0);
            lf_checker_rt::callee_thiscall!(UNREG, u32, mgr, this);
        }
        let linkp = this.wrapping_add(LINK) as *mut u32;
        if linkp.read_unaligned() != 0 {
            lf_checker_rt::callee_thiscall!(UNLINK, u32, linkp.read_unaligned(), linkp as u32);
            linkp.write_unaligned(0);
        }
        let bufp = this.wrapping_add(BUFFER) as *mut u32;
        if bufp.read_unaligned() != 0 {
            lf_checker_rt::callee_cdecl!(FREE, u32, bufp.read_unaligned());
            bufp.write_unaligned(0);
        }
        lf_checker_rt::callee_thiscall!(MEMBER, u32, this.wrapping_add(MEMBER_B));
        lf_checker_rt::callee_thiscall!(MEMBER, u32, this.wrapping_add(MEMBER_A));
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(BASE_TABLE));
        lf_checker_rt::callee_thiscall!(TAIL, u32, this)
    }
});
