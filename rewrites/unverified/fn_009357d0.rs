// original: 0x009357D0 net_activation_gate (proposed)

/// Gate a network session object, then activate it down one of two paths.
///
/// `obj` points to a session control block: dword at `+0x00` is the address
/// of its gate routine (called with the object itself, original passes it in
/// ECX), dword at `+0x08` points to an inner block whose dword at `+0x80`
/// selects the path. Returns the gate's answer unchanged when it is not 1.
///
/// Mode 0 (fast path): three liveness probes must all answer non-zero, two
/// key comparisons must match (each compares an 8-byte key reached through
/// the inner block at `+0x10` against one reached through the global session
/// at `+0x158`), then the global active flag is set and control tail-jumps
/// to the shared finish routine with the global session pointer. Any failure
/// returns the failing answer (0, or the comparison's full EAX with AL 0).
///
/// Mode 1 (slow path): one key comparison must match, then two notifier
/// routines run against the global session. If the session's ready byte at
/// `+0x169` is clear, two published counters are zeroed and the second
/// notifier's answer is returned. Otherwise the published counters are set
/// from the pending counter (clamped: non-positive becomes 1), four quiet
/// guards (a flag byte, a busy dword, two lock bytes) must all be clear, an
/// availability probe decides whether two start-up routines run, and a final
/// 3-argument log call's answer is returned.
///
/// Any other mode returns the mode word itself. Every multi-word cleanup is
/// the caller's (cdecl, one stack word); the tail jump carries no stack
/// arguments, only the session pointer in ECX.
lf_checker_rt::export!(cdecl, rw_009357D0(obj: u32) -> u32 {
    unsafe {
        const INNER_OFF: u32 = 8;
        const MODE_OFF: u32 = 0x80;
        const KEY_BASE_OFF: u32 = 0x10;
        const SESS_KEY_OFF: u32 = 0x158;
        const READY_OFF: u32 = 0x169;
        const GATE_OK: u32 = 1;
        const G_SESSION: u32 = 0x1BB5624;
        const G_ACTIVE: u32 = 0x18B6EDE;
        const G_PENDING: u32 = 0x11D6FD4;
        const G_COUNT: u32 = 0x11D6FD8;
        const G_COUNT_CLAMP: u32 = 0x11D6FDC;
        const G_QUIET: u32 = 0x11766D2;
        const G_BUSY: u32 = 0x18B6EF8;
        const G_LOCK0: u32 = 0x1160C35;
        const G_LOCK1: u32 = 0x1160C36;
        const LOG_TAG: u32 = 0xE877A0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn g32(file_va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(file_va)) }
        }
        #[inline(always)]
        unsafe fn g8(file_va: u32) -> u8 {
            unsafe { rd8(lf_checker_rt::relocated(file_va)) }
        }
        #[inline(always)]
        unsafe fn wg32(file_va: u32, v: u32) {
            unsafe { wr32(lf_checker_rt::relocated(file_va), v) }
        }
        #[inline(always)]
        unsafe fn wg8(file_va: u32, v: u8) {
            unsafe { wr8(lf_checker_rt::relocated(file_va), v) }
        }

        // Indirect gate call through the object's table word, as the original.
        let gate: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(rd32(obj)) };
        let gated = gate(obj);
        if gated != GATE_OK {
            return gated;
        }
        let inner = rd32(obj.wrapping_add(INNER_OFF));
        let mode = rd32(inner.wrapping_add(MODE_OFF));
        if mode == 0 {
            if lf_checker_rt::callee_cdecl!(1, u32,) == 0 {
                return 0;
            }
            let key_a = lf_checker_rt::callee_thiscall!(2, u32, inner.wrapping_add(KEY_BASE_OFF));
            let probe = lf_checker_rt::callee_cdecl!(1, u32,);
            let key_b = lf_checker_rt::callee_thiscall!(2, u32, probe);
            let cmp1 = lf_checker_rt::callee_thiscall!(3, u32, key_b, key_a);
            if (cmp1 as u8) == 0 {
                return cmp1;
            }
            let sess_key = g32(G_SESSION).wrapping_add(SESS_KEY_OFF);
            let probe2 = lf_checker_rt::callee_cdecl!(1, u32,);
            let key_c = lf_checker_rt::callee_thiscall!(2, u32, probe2);
            let cmp2 = lf_checker_rt::callee_thiscall!(4, u32, key_c, sess_key);
            if (cmp2 as u8) == 0 {
                return cmp2;
            }
            wg8(G_ACTIVE, 1);
            return lf_checker_rt::callee_thiscall!(11, u32, g32(G_SESSION));
        }
        if mode != 1 {
            return mode;
        }
        let sess0 = g32(G_SESSION);
        let key_a = lf_checker_rt::callee_thiscall!(2, u32, inner.wrapping_add(KEY_BASE_OFF));
        let cmp1 = lf_checker_rt::callee_thiscall!(3, u32, sess0.wrapping_add(SESS_KEY_OFF), key_a);
        if (cmp1 as u8) == 0 {
            return cmp1;
        }
        lf_checker_rt::callee_thiscall!(5, u32, g32(G_SESSION));
        let sess = g32(G_SESSION);
        let notified = lf_checker_rt::callee_thiscall!(6, u32, sess);
        if rd8(sess.wrapping_add(READY_OFF)) == 0 {
            wg32(G_COUNT, 0);
            wg32(G_COUNT_CLAMP, 0);
            return notified;
        }
        let pending = g32(G_PENDING);
        let clamped = if (pending as i32) > 0 { pending } else { 1 };
        let quiet = g8(G_QUIET);
        wg32(G_COUNT, pending);
        wg32(G_COUNT_CLAMP, clamped);
        if quiet != 0 {
            return pending;
        }
        if g32(G_BUSY) != 0 {
            return pending;
        }
        if g8(G_LOCK0) != 0 {
            return pending;
        }
        if g8(G_LOCK1) != 0 {
            return pending;
        }
        if (lf_checker_rt::callee_cdecl!(7, u32,) as u8) != 0 {
            lf_checker_rt::callee_cdecl!(8, u32,);
            lf_checker_rt::callee_cdecl!(9, u32,);
        }
        lf_checker_rt::callee_cdecl!(10, u32, 0xFFFF_FFFF, 0, LOG_TAG)
    }
});
