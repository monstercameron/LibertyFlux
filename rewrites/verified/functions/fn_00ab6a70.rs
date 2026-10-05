// original: 0x00ab6a70 stream_dispatch_iface (proposed)

/// Fetch a streaming interface result, directly or through its vtable.
///
/// When the low byte of `a2` is nonzero returns the word behind the
/// pointer at `a0 + 0xe98`. Otherwise resolves the family object by the
/// index at `a0 + 0x2e`, invokes its slot at vtable `+0x34`, scales the
/// answer through the scale callee, and bills `(answer, 0, lane, a1 + 4)`
/// — lane from `[a1 + 4 + 0x58]` — through the billing callee, returning
/// its answer.
///
/// Callees: 1 = family slot (thiscall through the planted vtable, no
/// words), 2 = scale (cdecl, one word), 3 = billing (cdecl, four words).
///
/// Original: 0x00ab6a70 (cdecl, three stack words).
lf_checker_rt::export!(cdecl, rw_00ab6a70(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const SCALE: u32 = 2;
        const BILLING: u32 = 3;
        const FAMILY_TABLE: u32 = 0x0129_5CD8;
        const INDEX_OFF: u32 = 0x2E;
        const DIRECT_OFF: u32 = 0xE98;
        const VTABLE_SLOT: u32 = 0x34;
        const LANE_OFF: u32 = 0x58;
        if (a2 as u8) != 0 {
            let p = ((a0 + DIRECT_OFF) as *const u32).read_unaligned();
            return (p as *const u32).read_unaligned();
        }
        let index = ((a0 + INDEX_OFF) as *const i16).read_unaligned() as i32 as u32;
        let family = (lf_checker_rt::global::<u32>(FAMILY_TABLE) as u32)
            .wrapping_add(index.wrapping_mul(4));
        let obj = (family as *const u32).read_unaligned();
        let vt = (obj as *const u32).read_unaligned();
        let slot: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute((((vt + VTABLE_SLOT)) as *const u32).read_unaligned() as usize);
        let r1 = slot(obj);
        let r2 = lf_checker_rt::callee_cdecl!(SCALE, u32, r1);
        let q = a1.wrapping_add(4);
        let lane = ((q + LANE_OFF) as *const u8).read() as u32;
        lf_checker_rt::callee_cdecl!(BILLING, u32, r2, 0, lane, q)
    }
});
