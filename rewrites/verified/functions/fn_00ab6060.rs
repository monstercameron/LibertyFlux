// original: 0x00ab6060 stream_begin_load (proposed)

/// Open a streaming load for object `obj` and vet its eleven lanes.
///
/// Resolves the lane-family table by the index at `obj + 0x2e`, builds a
/// load manager through the setup callee on the loader block, publishes
/// `a1` into the global epoch while clearing the slot and error globals,
/// then vets each of the eleven lane bytes at `[obj + 0x21c] + 0x5c + i`
/// through the vet callee. Returns 1 when every lane passes, 0 at the first
/// failure (judged on the callee's low byte).
///
/// Callees: 1 = manager setup (thiscall, one word),
/// 2 = lane vet (thiscall, two words).
///
/// Original: 0x00ab6060 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00ab6060(obj: u32, a1: u32) -> u32 {
    unsafe {
        const SETUP: u32 = 1;
        const VET: u32 = 2;
        const FAMILY_TABLE: u32 = 0x0129_5CD8;
        const LOADER: u32 = 0x0150_E0F4;
        const SLOTS: u32 = 0x0150_E128;
        const ERRORS: u32 = 0x0150_E0EC;
        const EPOCH: u32 = 0x0150_E0F0;
        const INDEX_OFF: u32 = 0x2E;
        const LANES_OFF: u32 = 0x21C;
        const LANE_BASE: u32 = 0x5C;
        const LANES: u32 = 11;
        const PARAM_OFF: u32 = 0x3C;
        let index = ((obj + INDEX_OFF) as *const i16).read_unaligned() as i32 as u32;
        let family = (lf_checker_rt::global::<u32>(FAMILY_TABLE) as u32)
            .wrapping_add(index.wrapping_mul(4));
        let entry = (family as *const u32).read_unaligned();
        let param = ((entry + PARAM_OFF) as *const u32).read_unaligned();
        let mgr = lf_checker_rt::callee_thiscall!(SETUP, u32, lf_checker_rt::relocated(LOADER), param);
        (lf_checker_rt::global::<u32>(SLOTS) as *mut u32).write_unaligned(0);
        (lf_checker_rt::global::<u32>(ERRORS) as *mut u32).write_unaligned(0);
        (lf_checker_rt::global::<u32>(EPOCH) as *mut u32).write_unaligned(a1);
        let lanes = ((obj + LANES_OFF) as *const u32)
            .read_unaligned()
            .wrapping_add(LANE_BASE);
        let mut i = 0u32;
        loop {
            let lane = ((lanes + i) as *const u8).read() as u32;
            let ok = lf_checker_rt::callee_thiscall!(VET, u32, mgr, i, lane);
            if ok & 0xFF == 0 {
                return 0;
            }
            i += 1;
            if i >= LANES {
                return 1;
            }
        }
    }
});
