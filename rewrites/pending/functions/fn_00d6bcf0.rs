// original: 0x00d6bcf0 teardown_release_slots

/// Tear down one menu-state object (original 0x00D6BCF0).
///
/// Stamps the teardown vtable, opens a token through two helpers, then
/// releases five owned slots: the first three are released and freed when
/// non-null, the last two get one release call first and are released and
/// freed only if still non-null afterwards. Every touched slot is cleared.
/// Closes the token through a third helper, runs two finalizers, and stamps
/// the dead vtable. Returns nothing meaningful.
export!(thiscall, rw_00d6bcf0(this: u32) -> u32 {
    const VTABLE_LIVE: u32 = 0x00EE_AF58;
    const TOKEN_ARG: u32 = 0x00EE_AEF0;
    const VTABLE_DEAD: u32 = 0x00EE_A88C;
    unsafe {
        (this as *mut u32).write(relocated(VTABLE_LIVE));
        callee_thiscall!(1, u32, this);
        let token = callee_cdecl!(2, u32, relocated(TOKEN_ARG));
        callee_cdecl!(3, u32, token);
        for off in [0x88usize, 0x8C, 0x90] {
            let slot = (this as *mut u32).add(off / 4);
            let owned = slot.read();
            if owned != 0 {
                callee_thiscall!(4, u32, owned);
                callee_cdecl!(5, u32, owned);
                slot.write(0);
            }
        }
        for off in [0x94usize, 0x98] {
            let slot = (this as *mut u32).add(off / 4);
            if slot.read() != 0 {
                callee_thiscall!(4, u32, slot.read());
                let still = slot.read();
                if still != 0 {
                    callee_thiscall!(4, u32, still);
                    callee_cdecl!(5, u32, still);
                }
                slot.write(0);
            }
        }
        callee_cdecl!(6, u32, token);
        callee_cdecl!(7, u32,);
        callee_cdecl!(8, u32,);
        (this as *mut u32).write(relocated(VTABLE_DEAD));
    }
    0
});
