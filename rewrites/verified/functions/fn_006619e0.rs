// original: 0x006619e0 rage::snModifyPresenceFlagsTask::vf7
/// Presence-flags task teardown: publish dirty flags, forward, drop session.
///
/// thiscall/2, returns void. When tearing down with live session data whose
/// generation pair still matches, publishes the dirty flags through callee 1
/// and goes straight to teardown; otherwise drains a latched submit first.
/// Then forwards both arguments to the shared teardown helper and clears the
/// session link.
export!(thiscall, rw_006619e0(this_ptr: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        // Only the fall-through path (no teardown argument, or a session
        // outside the live window) drains a latched submit first; every path
        // that enters the publish window jumps straight to the teardown call.
        // (The pushed frame pointer accounts for the first push shifting esp.)
        let session = ((this_ptr + 0x60) as *const u32).read();
        let state = ((session + 0x50) as *const u32).read();
        if a0 == 1 && (state == 2 || state == 3) {
            let gen_lo = ((session + 0xbf0) as *const u32).read();
            let gen_hi = ((session + 0xbf4) as *const u32).read();
            let dirty = ((this_ptr + 0x9c) as *const u8).read() & 1;
            if gen_lo == ((session + 0xc30) as *const u32).read()
                && gen_hi == ((session + 0xc34) as *const u32).read()
                && dirty != 0
            {
                let mut frame = [
                    ((session + 0x540) as *const u32).read(),
                    ((session + 0x544) as *const u32).read(),
                    (((this_ptr + 0x98) as *const u8).read() & 1) as u32,
                ];
                callee_thiscall!(1, u32, session,
                    frame.as_mut_ptr() as u32, session);
            }
        } else if ((this_ptr + 0x90) as *const u32).read() == 1 {
            let session = ((this_ptr + 0x60) as *const u32).read();
            callee_thiscall!(2, u32, session.wrapping_add(0x48), this_ptr.wrapping_add(0x90));
        }
        callee_thiscall!(3, u32, this_ptr, a0, a1);
        ((this_ptr + 0x60) as *mut u32).write(0);
        0
    }
});
