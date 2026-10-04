// original: 0x009fad20 CPlayStatIntInt::~CPlayStatIntInt_2

/// Reset the playstats collector state when its generation flag is set.
///
/// Takes no arguments and no register inputs (cdecl, callee-saved `esi` only).
/// Reads a tick from callee 0 and stamps it odd (`tick | 1`); the stamp is
/// always stored to `STAMP_SLOT`, and the flag word `GEN_FLAG` is always
/// cleared on exit.
///
/// When `GEN_FLAG` is zero the function returns the tick after storing the
/// stamp. Otherwise it asks gatekeeper callee 1 (thiscall on `GATE_OBJ`) to
/// run; a zero low byte returns that answer after clearing the flag. On a
/// nonzero answer it toggles `ACTIVE_SET` (1 when it was 0, else 0), points
/// `SET_BASE` at one of two 2048-byte banks selected by the toggle, mirrors
/// the flag into `GEN_MIRROR`, and runs callee 2 (thiscall on `BANK_OBJ` with
/// `BANK_ARG`). It then copies the source pair (`SRC_A`/`SRC_B`) into
/// `DST_A`/`DST_B` and clears `SRC_KIND`, unless both sources are zero, in
/// which case the fallback pair (`ALT_A`/`ALT_B`) is copied and `SRC_KIND` is
/// set to 1.
///
/// The timed re-seed runs when `LAST_STAMP` is zero or the stamp has advanced
/// past `RESEED_AFTER` ticks (`stamp - LAST_STAMP >= RESEED_AFTER`, unsigned);
/// it runs callee 3 (cdecl, `RESEED_OBJ` and `RESEED_SIZE`), sets `SEEDED_BYTE`
/// and records the stamp. Finally it runs callee 4 (thiscall on `GATE_OBJ`)
/// and callee 5 (cdecl, the value at `FINAL_ARG`), clears the flag and returns
/// callee 5's answer (the tick on the early path, the gate answer when gated).
lf_checker_rt::export!(cdecl, rw_009fad20() -> u32 {
    unsafe {
        const ACTIVE_SET: u32 = 0x012B_9000;
        const GEN_FLAG: u32 = 0x012B_9004;
        const STAMP_SLOT: u32 = 0x012B_9008;
        const LAST_STAMP: u32 = 0x012B_9018;
        const SEEDED_BYTE: u32 = 0x012B_9024;
        const SRC_A: u32 = 0x012B_9028;
        const SRC_B: u32 = 0x012B_902C;
        const ALT_A: u32 = 0x012B_9030;
        const ALT_B: u32 = 0x012B_9034;
        const DST_A: u32 = 0x012B_9078;
        const DST_B: u32 = 0x012B_907C;
        const SET_BASE: u32 = 0x012B_9084;
        const BANK_LO: u32 = 0x012B_8000;
        const BANK_STRIDE_SHIFT: u32 = 11;
        const GATE_OBJ: u32 = 0x012B_9170;
        const BANK_OBJ: u32 = 0x012B_9100;
        const BANK_ARG: u32 = 0x012B_9088;
        const FINAL_ARG: u32 = 0x012B_9C3C;
        const GEN_MIRROR: u32 = 0x012B_9C40;
        const SRC_KIND: u32 = 0x012B_9C44;
        const RESEED_OBJ: u32 = 0x012B_9C48;
        const RESEED_SIZE: u32 = 0x50;
        const RESEED_AFTER: u32 = 0x0103_B598;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(a) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (lf_checker_rt::global::<u32>(a)).write_unaligned(v) }
        }

        let tick: u32 = lf_checker_rt::callee_cdecl!(0, u32,);
        let stamp = tick | 1;
        let flag = rd32(GEN_FLAG);
        wr32(STAMP_SLOT, stamp);
        let mut result = tick;
        // The worker relocates the image: every absolute address the original
        // forms as a value (call arguments, stored pointers) must be relocated too.
        let gate_obj = lf_checker_rt::relocated(GATE_OBJ);
        let bank_obj = lf_checker_rt::relocated(BANK_OBJ);
        let bank_arg = lf_checker_rt::relocated(BANK_ARG);
        let bank_lo = lf_checker_rt::relocated(BANK_LO);
        if flag != 0 {
            let gate: u32 = lf_checker_rt::callee_thiscall!(1, u32, gate_obj);
            result = gate;
            if gate & 0xFF != 0 {
                let toggled: u32 = if rd32(ACTIVE_SET) == 0 { 1 } else { 0 };
                wr32(ACTIVE_SET, toggled);
                wr32(SET_BASE, toggled.wrapping_shl(BANK_STRIDE_SHIFT).wrapping_add(bank_lo));
                wr32(GEN_MIRROR, rd32(GEN_FLAG));
                let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, bank_obj, bank_arg);
                let src_a = rd32(SRC_A);
                let src_b = rd32(SRC_B);
                if src_a | src_b == 0 {
                    wr32(DST_A, rd32(ALT_A));
                    wr32(DST_B, rd32(ALT_B));
                    wr32(SRC_KIND, 1);
                } else {
                    wr32(DST_A, src_a);
                    wr32(DST_B, src_b);
                    wr32(SRC_KIND, 0);
                }
                let prev = rd32(LAST_STAMP);
                let mut reseed = prev == 0;
                if !reseed {
                    let elapsed = stamp.wrapping_sub(prev);
                    reseed = elapsed >= rd32(RESEED_AFTER);
                }
                if reseed {
                    let _: u32 = lf_checker_rt::callee_cdecl!(
                        3,
                        u32,
                        lf_checker_rt::relocated(RESEED_OBJ),
                        RESEED_SIZE
                    );
                    lf_checker_rt::global::<u8>(SEEDED_BYTE).write(1);
                    wr32(LAST_STAMP, stamp);
                }
                let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, gate_obj);
                result = lf_checker_rt::callee_cdecl!(5, u32, rd32(FINAL_ARG));
            }
        }
        wr32(GEN_FLAG, 0);
        result
    }
});
