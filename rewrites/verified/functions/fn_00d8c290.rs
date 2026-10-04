// original: 0x00d8c290 audio_event_dispatch_switch
/// Fire entity audio events for states and the kind switch.
///
/// Ignores entities already registered as self, otherwise resolves the
/// listener hub, scales the listener index and asks the gate (which takes
/// the scaled index as its object) whether the entity may enter; when it
/// accepts, the enter pair of events fires. Always
/// fires the base event, then reads the kind through the outer link and
/// fires the kind event: variant 0x1d1 for kind 7, variant 0x1d0 for any
/// other kind 3..14, and nothing for kinds outside that range. Returns the
/// last event answer, or the out-of-range kind value itself.
lf_rs89_rt::export!(cdecl, rw_00d8c290(ent: u32) -> u32 {
    unsafe {
        let self_ref: u32 = lf_rs89_rt::callee_cdecl!(1, u32, 0);
        if self_ref != ent {
            let hub: u32 = lf_rs89_rt::callee_cdecl!(1, u32, 0);
            let node = *(hub.wrapping_add(0x228) as *const u32);
            let scaled = (*(node.wrapping_add(0x444) as *const u32))
                .wrapping_mul(0x5C)
                .wrapping_add(lf_rs89_rt::relocated(0x1666C90));
            let gate: u32 = lf_rs89_rt::callee_thiscall!(2, u32, scaled, ent);
            if gate & 0xFF != 0 {
                let one = 1.0f32.to_bits();
                lf_rs89_rt::callee_cdecl!(3, u32, 0x1D2, one);
                lf_rs89_rt::callee_cdecl!(4, u32, 0, 0x2AC, one);
            }
        }
        let one = 1.0f32.to_bits();
        lf_rs89_rt::callee_cdecl!(3, u32, 0x1CD, one);
        let outer = *(ent.wrapping_add(0x21C) as *const u32);
        let kind = (*(outer.wrapping_add(0x12C) as *const u32)).wrapping_sub(3);
        if kind > 0xB {
            kind
        } else if kind == 4 {
            lf_rs89_rt::callee_cdecl!(3, u32, 0x1D1, one)
        } else {
            lf_rs89_rt::callee_cdecl!(3, u32, 0x1D0, one)
        }
    }
});
