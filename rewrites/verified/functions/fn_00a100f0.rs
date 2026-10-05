// original: 0x00a100f0 vehicle_event_route_b (proposed)
/// Route a mirrored vehicle event unless the pair is already linked.
///
/// Mirrors the previous router with the roles exchanged: `obj` (with `tag`)
/// and `peer` form the pair, and when `obj` already matches the table slot
/// selected by the word at `peer + 0x900`, nothing happens. Otherwise the
/// primary handler runs on (`obj`, `head`, `tag`, `peer`); a non-zero low
/// byte in its answer skips the secondary handler on (`obj`, `extra`,
/// `peer`); the closer always runs on (`obj`, `peer`). No return value is
/// set. Cdecl, five stack arguments.
export!(cdecl, rw_00a100f0(head: u32, obj: u32, tag: u32, extra: u32, peer: u32) -> u32 {
    unsafe {
        const PRIMARY: u32 = 1;
        const SECONDARY: u32 = 2;
        const CLOSER: u32 = 3;
        const TABLE: u32 = 0x012bd100;
        const SEL_OFF: u32 = 0x900;
        let sel = ((peer + SEL_OFF) as *const u32).read_unaligned();
        let slot = (relocated(TABLE).wrapping_add(sel.wrapping_mul(4))) as *const u32;
        if obj == slot.read_unaligned() {
            return 0;
        }
        if (callee_thiscall!(PRIMARY, u32, obj, head, tag, peer) & 0xff) == 0 {
            callee_thiscall!(SECONDARY, u32, obj, extra, peer);
        }
        callee_thiscall!(CLOSER, u32, obj, peer);
        0
    }
});
