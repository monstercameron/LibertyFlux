// original: 0x00c57040 task_probe_sixteen_entries (proposed)

/// Probe sixteen task entries, clearing the mark of each failing one.
///
/// The three global words are gathered into a buffer once. For each of the
/// sixteen entries at `this+8+k*0x10`, a null target is skipped; otherwise
/// callee 1 is invoked with the buffer, the target's link (`[t+0x20]+0x30`,
/// or `t+0x10` when the link is null) and the constants (0, 6, 2, 0). When
/// the answer's low byte is 0 the mark byte after the entry is cleared.
/// Returns nothing.
///
/// Original: 0x00c57040 (thiscall: `this` in ecx, no stack words).
lf_checker_rt::export!(thiscall, rw_00c57040(this: u32) -> u32 {
    unsafe {
        const F0_SLOT: u32 = 0x128e340;
        const F1_SLOT: u32 = 0x128e344;
        const F2_SLOT: u32 = 0x128e348;
        const PROBE: u32 = 1;
        let buf = [
            lf_checker_rt::global::<u32>(F0_SLOT).read(),
            lf_checker_rt::global::<u32>(F1_SLOT).read(),
            lf_checker_rt::global::<u32>(F2_SLOT).read(),
        ];
        for k in 0..16u32 {
            let e = this.wrapping_add(8).wrapping_add(k.wrapping_mul(0x10));
            let target = (e as *const u32).read_unaligned();
            if target == 0 {
                continue;
            }
            let link = ((target + 0x20) as *const u32).read_unaligned();
            let arg = if link == 0 {
                target.wrapping_add(0x10)
            } else {
                link.wrapping_add(0x30)
            };
            let r: u32 = lf_checker_rt::callee_cdecl!(
                PROBE, u32, buf.as_ptr() as u32, arg, 0, 6, 2, 0,
            );
            if (r & 0xff) == 0 {
                ((e + 4) as *mut u8).write(0);
            }
        }
        0
    }
});

