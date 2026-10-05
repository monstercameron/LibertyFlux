// original: 0x00dc5560 TaskShoot_UpdateAim (proposed)

/// Refresh the shoot task's aim vector: validate the ped's weapon, then
/// either reuse the stored aim direction or recompute it through the aim
/// solver, chosen by the ped's aim flags and whether the stored direction
/// is set.
///
/// `this` points to the task (stored direction at `+0x20`, link at `+0x14`,
/// helper at `+0x68`, result at `+0x30`) and `ped` to the ped. Callee 0
/// validates the ped's weapon block (`ped+0x2B0`); a null answer, or a null
/// `ped+0x2C4`, ends the call. When `ped+0x218` is zero and `ped+0x219` is
/// not, the task takes the direct branch, otherwise the guided branch; both
/// branches then test the stored direction component-wise against zero
/// (exact equality: a NaN counts as set).
///
/// On the direct branch a set direction is pushed through callee 1 and
/// copied to `+0x30`. An unset direction loads the global default vector
/// `AIM_DEFAULT` into two frame buffers and runs callee 2 (this `wmgr`,
/// `ped`, `[edx+0x20]`, input buffer, output buffer); when its low byte is
/// non-zero callee 1 runs on the output buffer, which is then copied to
/// `+0x30`. On the guided branch a null helper ends the call; a set
/// direction runs callee 1 on it; an unset direction with a null link runs
/// callee 3, otherwise callee 1 runs on the linked offset (`link+0x10`, or
/// `[link+0x20]+0x30`). Returns the last value in eax on each path.
///
/// The frame buffers are the rewrite's own locals: the contract skips their
/// addresses, snapshots their contents and scripts the callees' outputs.
///
/// Original: 0x00dc5560 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00dc5560(this: u32, ped: u32) -> u32 {
    unsafe {
        const AIM_DEFAULT: u32 = 0x01B4B2A0;
        const VALIDATE_WEAPON: u32 = 0;
        const PUSH_AIM: u32 = 1;
        const SOLVE_AIM: u32 = 2;
        const PUSH_DEFAULT: u32 = 3;

        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }

        let wmgr: u32 = lf_checker_rt::callee_thiscall!(VALIDATE_WEAPON, u32, ped + 0x2b0);
        if wmgr == 0 {
            return 0;
        }
        let extra = rd32(ped + 0x2c4);
        if extra == 0 {
            return wmgr;
        }
        let aim_at = ped + 0xbb0;
        if rd8(ped + 0x218) == 0 && rd8(ped + 0x219) != 0 {
            let dir_x = f32::from_bits(rd32(this + 0x20));
            let dir_y = f32::from_bits(rd32(this + 0x24));
            if dir_x == 0.0 || dir_y == 0.0 {
                let g0 = rd32(lf_checker_rt::relocated(AIM_DEFAULT));
                let g1 = rd32(lf_checker_rt::relocated(AIM_DEFAULT + 4));
                let g2 = rd32(lf_checker_rt::relocated(AIM_DEFAULT + 8));
                let mut out = [g0, g1, g2, 0u32];
                let inp = [g0, g1, g2];
                let solved: u32 = lf_checker_rt::callee_thiscall!(SOLVE_AIM, u32, wmgr, ped,
                    rd32(extra + 0x20), inp.as_ptr() as u32, out.as_mut_ptr() as u32);
                let mut result = solved;
                if (solved & 0xff) != 0 {
                    let helper = rd32(this + 0x68);
                    result = lf_checker_rt::callee_thiscall!(PUSH_AIM, u32, aim_at,
                        out.as_mut_ptr() as u32, rd32(helper + 0x58));
                }
                wr32(this + 0x30, out[0]);
                wr32(this + 0x34, out[1]);
                wr32(this + 0x38, out[2]);
                wr32(this + 0x3c, out[3]);
                return result;
            }
            let helper = rd32(this + 0x68);
            lf_checker_rt::callee_thiscall!(PUSH_AIM, u32, aim_at, this + 0x20, rd32(helper + 0x58));
            wr32(this + 0x30, rd32(this + 0x20));
            wr32(this + 0x34, rd32(this + 0x24));
            wr32(this + 0x38, rd32(this + 0x28));
            let w = rd32(this + 0x2c);
            wr32(this + 0x3c, w);
            return w;
        }
        let helper = rd32(this + 0x68);
        if helper == 0 {
            return wmgr;
        }
        let dir_x = f32::from_bits(rd32(this + 0x20));
        let dir_y = f32::from_bits(rd32(this + 0x24));
        if dir_x == 0.0 || dir_y == 0.0 {
            let rate = rd32(helper + 0x58);
            let link = rd32(this + 0x14);
            if link == 0 {
                return lf_checker_rt::callee_thiscall!(PUSH_DEFAULT, u32, aim_at,
                    rd32(ped + 0xaa0), 0, 0, rate);
            }
            let inner = rd32(link + 0x20);
            let target = if inner == 0 { link + 0x10 } else { inner + 0x30 };
            return lf_checker_rt::callee_thiscall!(PUSH_AIM, u32, aim_at, target, rate);
        }
        lf_checker_rt::callee_thiscall!(PUSH_AIM, u32, aim_at, this + 0x20, rd32(helper + 0x58))
    }
});
