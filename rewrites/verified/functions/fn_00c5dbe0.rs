// original: 0x00c5dbe0 gang_driveby_can_attack (proposed)

/// Decide whether a gang-driveby task may engage: range, ped state, weapon
/// and ammunition checks, returning 1 when the attack proceeds.
///
/// `this` is the task object (an id word at `+0x20` and the address of the
/// word at `+0x30` passed to the locator callee, an engagement radius at
/// `+0x40`, a latched "cannot engage"
/// flag at `+0x48`, an enable flag at `+0x4a`). `ped` is the ped (matrix at
/// `+0x20` whose `+0x30` entry is its position, an auxiliary object at
/// `+0x6c` whose byte at `+0xe` forces engagement, state flags at
/// `+0x218`/`+0x219`, a status word at `+0x2a0`, weapon slots at `+0x2b0`, a
/// state word at `+0xa70`, a helper object at `+0xb30`). `bounds` points at a
/// pair of floats (`+0x1c`/`+0x20`) bracketing the probe value the facing
/// callee writes. `flags` bit 19 enables the facing probe, bit 18 the final
/// manager check. `want` equal to 2 refuses. The remaining words are passed
/// through to the facing callee (three of them as float bits).
///
/// Order: locate a target point through callee 0 and measure its squared
/// distance from the ped; refuse when out of range (unless the ped flags or
/// callee 1 say otherwise). Refuse when callee 2 fails (marking the ped
/// status word), or when the facing probe disagrees. Then require a usable
/// weapon: callee 6 resolves the slot's weapon id to an info struct whose
/// slot field (`+0x4`) must not be 8, callee 8 must report ammunition (else
/// callee 9 rearms and the answer is 0), and callee 10's manager must not be
/// in state 3 with bit 18 set. Any refusal after the range gate latches
/// `this.+0x48`. Float operation order is the original's; comparisons treat
/// NaN as not-above, like `comiss`/`ja`.
///
/// Original: 0x00c5dbe0 (thiscall, nine stack words).
lf_checker_rt::export!(thiscall, rw_00c5dbe0(
    this: u32,
    ped: u32,
    want: u32,
    a10: u32,
    a14: u32,
    f18: u32,
    f1c: u32,
    f20: u32,
    flags: u32,
    bounds: u32,
) -> u32 {
    unsafe {
        const CAL_LOCATE: u32 = 0;
        const CAL_GATE1: u32 = 1;
        const CAL_GATE2: u32 = 2;
        const CAL_FACE: u32 = 3;
        const CAL_HELPER: u32 = 4;
        const CAL_PROBE: u32 = 5;
        const CAL_WINFO: u32 = 6;
        const CAL_WCHECK: u32 = 7;
        const CAL_AMMO: u32 = 8;
        const CAL_REARM: u32 = 9;
        const CAL_MGR: u32 = 10;
        const FLAG_FACE: u32 = 0x80000;
        const FLAG_MGR: u32 = 0x40000;
        const PED_MARK: u32 = 0x2000;
        const BAD_SLOT: u32 = 8;
        const MGR_STATE: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        // Locate the target point; the callee fills three floats.
        let mut out = [0u32; 3];
        lf_checker_rt::callee_cdecl!(
            CAL_LOCATE,
            u32,
            out.as_mut_ptr() as u32,
            ped,
            rd32(this.wrapping_add(0x20)),
            this.wrapping_add(0x30)
        );
        let mat = rd32(ped.wrapping_add(0x20));
        let dx = sub(f32::from_bits(out[0]), rdf(mat.wrapping_add(0x30)));
        let dy = sub(f32::from_bits(out[1]), rdf(mat.wrapping_add(0x34)));
        let dz = sub(f32::from_bits(out[2]), rdf(mat.wrapping_add(0x38)));
        let dy2 = mul(dy, dy);
        let dx2 = mul(dx, dx);
        let dz2 = mul(dz, dz);
        let dist2 = add(add(dy2, dx2), dz2);

        if rd8(this.wrapping_add(0x4a)) == 0 {
            return 0;
        }
        if rd8(ped.wrapping_add(0x218)) == 0 && rd8(ped.wrapping_add(0x219)) != 0 {
            let gate: u32 = lf_checker_rt::callee_cdecl!(CAL_GATE1, u32, ped);
            if gate & 0xff == 0 {
                wr8(this.wrapping_add(0x48), 1);
                return 0;
            }
        } else {
            let r = rdf(this.wrapping_add(0x40));
            if dist2 > mul(r, r) {
                return 0;
            }
        }

        let aux = rd32(ped.wrapping_add(0x6c));
        if aux != 0 && rd8(aux.wrapping_add(0x0e)) != 0 {
            return 1;
        }
        let gate2: u32 = lf_checker_rt::callee_cdecl!(CAL_GATE2, u32, ped);
        if gate2 & 0xff == 0 {
            wr32(
                ped.wrapping_add(0x2a0),
                rd32(ped.wrapping_add(0x2a0)) | PED_MARK,
            );
            return 0;
        }

        let mut probe = 0u32;
        let face: u32 = lf_checker_rt::callee_cdecl!(
            CAL_FACE,
            u32,
            ped,
            f18,
            a10,
            a14,
            &mut probe as *mut u32 as u32,
            f1c,
            f20
        );
        if rd8(ped.wrapping_add(0x218)) != 0 || rd8(ped.wrapping_add(0x219)) == 0 {
            if face & 0xff != 0 {
                return 0;
            }
        }

        let helper = rd32(ped.wrapping_add(0xb30));
        if helper != 0 {
            let h: u32 = lf_checker_rt::callee_thiscall!(CAL_HELPER, u32, helper, ped);
            if h & 0xff != 0 && flags & FLAG_FACE != 0 {
                let mut c1 = 0xffffffffu32;
                let mut c2 = 0xffffffffu32;
                let pr: u32 = lf_checker_rt::callee_cdecl!(
                    CAL_PROBE,
                    u32,
                    ped,
                    0,
                    &mut c2 as *mut u32 as u32,
                    &mut c1 as *mut u32 as u32
                );
                if pr & 0xff != 0 {
                    let lo = rdf(bounds.wrapping_add(0x1c));
                    let hi = rdf(bounds.wrapping_add(0x20));
                    let pv = f32::from_bits(probe);
                    if lo > pv || pv > hi {
                        wr8(this.wrapping_add(0x48), 1);
                        return 0;
                    }
                }
            }
        }

        if want == 2 {
            return 0;
        }
        if rd8(this.wrapping_add(0x48)) != 0 {
            return 0;
        }
        if rd32(ped.wrapping_add(0xa70)) == 1 {
            wr8(this.wrapping_add(0x48), 1);
            return 0;
        }
        let slot = rd32(ped.wrapping_add(0x2b0));
        let idx = slot.wrapping_add(3).wrapping_mul(3);
        let wid = rd32(ped.wrapping_add(idx.wrapping_mul(4)).wrapping_add(0x2b0));
        let info: u32 = lf_checker_rt::callee_cdecl!(CAL_WINFO, u32, wid);
        let wc: u32 = lf_checker_rt::callee_thiscall!(CAL_WCHECK, u32, info, ped);
        if wc & 0xff == 0 {
            wr8(this.wrapping_add(0x48), 1);
            return 0;
        }
        if rd32(info.wrapping_add(4)) == BAD_SLOT {
            wr8(this.wrapping_add(0x48), 1);
            return 0;
        }
        let slots_base = ped.wrapping_add(0x2b0);
        let ammo: u32 = lf_checker_rt::callee_thiscall!(CAL_AMMO, u32, slots_base, slot);
        if ammo == 0 {
            lf_checker_rt::callee_thiscall!(CAL_REARM, u32, ped);
            return 0;
        }
        let mgr: u32 = lf_checker_rt::callee_thiscall!(CAL_MGR, u32, slots_base);
        if mgr == 0 {
            return 1;
        }
        if rd32(mgr.wrapping_add(0x1c)) != MGR_STATE {
            return 1;
        }
        if flags & FLAG_MGR == 0 {
            return 0;
        }
        1
    }
});
