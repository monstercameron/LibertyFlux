// original: 0x00C7E070 CTaskComplexPoliceSniperScenario::vf19

/// Police-sniper-scenario task update: pick this tick's sub-task for the
/// owning ped.
///
/// `this` is the scenario task, `ped` the ped it drives. The update first
/// resolves a candidate task object through callee 1; a null answer ends
/// the tick with 0. Otherwise it gathers two handles (callee 2 off the
/// ped's field at `+0x224`, callee 3 off the ped block at `+0x2B0`), probes
/// the candidate's scope block (`+0x228`, or a null scope) through callee 4,
/// and requires a flag bit (bit 5 of the word at `+0x20` of the object
/// callee 5 resolves from the second handle's `+0x18` link) to be set.
/// Any failure along this chain falls through to the late path, which asks
/// the task-system singleton (callee 7, reached through the global at
/// `G_TASK_SYS`) for a timed wait task (callee 10: id 1000, 8.0 seconds).
///
/// When the chain succeeds, callee 6 validates the (task, ped) pair: an
/// answer of 1 with a stale marker (the word at candidate `+0xA70` not 1)
/// yields an aim task (callee 8, mode 4, seeded with the global float at
/// `G_AIM_BIAS`); otherwise, if the first handle is live and callee 9
/// confirms it, the tick yields a scan task (callee 8, mode 2, unit bias).
/// A null singleton at any final step ends the tick with 0, otherwise the
/// tick returns the created sub-task.
///
/// Original: 0x00C7E070 (thiscall, one stack word, returns eax).
lf_checker_rt::export!(thiscall, rw_00c7e070(this: u32, ped: u32) -> u32 {
    unsafe {
        const PED_SCOPE: u32 = 0x224;
        const PED_BLOCK: u32 = 0x2B0;
        const CAND_SCOPE: u32 = 0x228;
        const SCOPE_PROBE_OFF: u32 = 0x70;
        const HANDLE_LINK: u32 = 0x18;
        const FLAG_WORD: u32 = 0x20;
        const FLAG_BIT: u32 = 5;
        const CAND_MARKER: u32 = 0xA70;
        const G_TASK_SYS: u32 = 0x0167E2A0;
        const G_AIM_BIAS: u32 = 0x0104B97C;
        const ONE_BITS: u32 = 0x3F800000;
        const C_RESOLVE: u32 = 1;
        const C_HANDLE_A: u32 = 2;
        const C_HANDLE_B: u32 = 3;
        const C_PROBE: u32 = 4;
        const C_FLAGS: u32 = 5;
        const C_VALIDATE: u32 = 6;
        const C_SINGLETON: u32 = 7;
        const C_SCENARIO: u32 = 8;
        const C_CONFIRM: u32 = 9;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn late_wait() -> u32 {
            unsafe {
                const G_TASK_SYS: u32 = 0x0167E2A0;
                const WAIT_ID: u32 = 1000;
                const WAIT_SECS_BITS: u32 = 0x41000000;
                const C_SINGLETON: u32 = 7;
                const C_WAIT: u32 = 10;
                let sys: u32 = lf_checker_rt::callee_thiscall!(
                    C_SINGLETON,
                    u32,
                    lf_checker_rt::global::<u32>(G_TASK_SYS).read()
                );
                if sys == 0 {
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(
                    C_WAIT,
                    u32,
                    sys,
                    WAIT_ID,
                    0,
                    0,
                    WAIT_SECS_BITS
                )
            }
        }

        let cand: u32 = lf_checker_rt::callee_thiscall!(C_RESOLVE, u32, this);
        if cand == 0 {
            return 0;
        }
        let scope_owner = rd32(ped.wrapping_add(PED_SCOPE));
        let handle_a: u32 =
            lf_checker_rt::callee_thiscall!(C_HANDLE_A, u32, scope_owner, 0);
        let handle_b: u32 = lf_checker_rt::callee_thiscall!(
            C_HANDLE_B,
            u32,
            ped.wrapping_add(PED_BLOCK)
        );
        let scope_probe = rd32(cand.wrapping_add(CAND_SCOPE));
        let probe_this = if scope_probe == 0 {
            0
        } else {
            scope_probe.wrapping_add(SCOPE_PROBE_OFF)
        };
        let probe_ok: u32 = lf_checker_rt::callee_thiscall!(C_PROBE, u32, probe_this);
        if probe_ok == 0 || handle_b == 0 {
            return late_wait();
        }
        let link = rd32(handle_b.wrapping_add(HANDLE_LINK));
        let flags_obj: u32 = lf_checker_rt::callee_cdecl!(C_FLAGS, u32, link);
        let flag_word = rd32(flags_obj.wrapping_add(FLAG_WORD));
        if (flag_word >> FLAG_BIT) & 1 == 0 {
            return late_wait();
        }
        let verdict: u32 =
            lf_checker_rt::callee_thiscall!(C_VALIDATE, u32, this, ped, cand);
        if verdict == 1 && rd32(cand.wrapping_add(CAND_MARKER)) != 1 {
            let sys: u32 = lf_checker_rt::callee_thiscall!(
                C_SINGLETON,
                u32,
                lf_checker_rt::global::<u32>(G_TASK_SYS).read()
            );
            if sys == 0 {
                return 0;
            }
            let bias = lf_checker_rt::global::<u32>(G_AIM_BIAS).read();
            return lf_checker_rt::callee_thiscall!(
                C_SCENARIO, u32, sys, 4, cand, 0, bias, 0, 1, 1, ONE_BITS
            );
        }
        if handle_a == 0 {
            return late_wait();
        }
        let confirmed: u32 =
            lf_checker_rt::callee_thiscall!(C_CONFIRM, u32, handle_a, 0, cand);
        if confirmed & 0xFF == 0 {
            return late_wait();
        }
        let sys: u32 = lf_checker_rt::callee_thiscall!(
            C_SINGLETON,
            u32,
            lf_checker_rt::global::<u32>(G_TASK_SYS).read()
        );
        if sys == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(
            C_SCENARIO, u32, sys, 2, cand, 0, ONE_BITS, 0, 1, 1, ONE_BITS
        )
    }
});
