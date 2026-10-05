// original: 0x00a10090 vehicle_event_route_a (proposed)
/// Route a vehicle event unless the pair is already linked.
///
/// `tgt` (with `key`) and `src` form a pair: when `tgt` already matches the
/// table slot selected by the word at `src + 0x900`, nothing happens.
/// Otherwise the primary handler runs on (`tgt`, `a`, `b`, `key`, `src`);
/// when the word at `tgt + 0x11c` is zero the readiness probe runs first
/// and a zero answer skips the secondary handler; the secondary handler
/// then runs on (`tgt`, `key`, `src`) and the closer on (`tgt`, `src`).
/// No return value is set. Cdecl, five stack arguments.
export!(cdecl, rw_00a10090(tgt: u32, a: u32, key: u32, src: u32, b: u32) -> u32 {
    unsafe {
        const PRIMARY: u32 = 1;
        const PROBE: u32 = 2;
        const SECONDARY: u32 = 3;
        const CLOSER: u32 = 4;
        const TABLE: u32 = 0x012bd100;
        const SEL_OFF: u32 = 0x900;
        const STATE_OFF: u32 = 0x11c;
        let sel = ((src + SEL_OFF) as *const u32).read_unaligned();
        let slot = (relocated(TABLE).wrapping_add(sel.wrapping_mul(4))) as *const u32;
        if tgt == slot.read_unaligned() {
            return 0;
        }
        callee_thiscall!(PRIMARY, u32, tgt, a, b, key, src);
        if ((tgt + STATE_OFF) as *const u32).read_unaligned() == 0 {
            if (callee_thiscall!(PROBE, u32, tgt) & 0xff) == 0 {
                callee_thiscall!(CLOSER, u32, tgt, src);
                return 0;
            }
        }
        callee_thiscall!(SECONDARY, u32, tgt, key, src);
        callee_thiscall!(CLOSER, u32, tgt, src);
        0
    }
});
