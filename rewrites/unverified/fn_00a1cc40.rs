// original: 0x00a1cc40 cam_pending_publish (proposed)

/// Publishes two pending camera values and resets them to idle.
///
/// Two global slots hold pending floats (idle value `IDLE` = -999.0).
/// For each slot in turn, when its current value differs from idle it is
/// copied to the matching out-pointer (`p0`, then `p1`) and the slot is
/// reset to the idle bits. A NaN slot counts as differing (it is copied
/// and reset); +0.0 and -0.0 both count as idle-equal only against an idle
/// slot of the same value. Returns nothing.
///
/// Original: 0x00a1cc40 (stdcall, two stack words).
lf_checker_rt::export!(stdcall, rw_00a1cc40(p0: u32, p1: u32) -> u32 {
    unsafe {
        const SLOT0: u32 = 0x0103_bfe0;
        const IDLE: f32 = f32::from_bits(0xc479_c000); // -999.0
        let g0 = lf_checker_rt::global::<u32>(SLOT0);
        let a = f32::from_bits(g0.read_unaligned());
        if a != IDLE {
            (p0 as *mut u32).write_unaligned(a.to_bits());
            g0.write_unaligned(IDLE.to_bits());
        }
        let g1 = g0.add(1);
        let b = f32::from_bits(g1.read_unaligned());
        if b != IDLE {
            (p1 as *mut u32).write_unaligned(b.to_bits());
            g1.write_unaligned(IDLE.to_bits());
        }
        0
    }
});
