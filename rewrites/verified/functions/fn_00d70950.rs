// original: 0x00d70950 CReplayProgressBar::vf1

/// Replay progress bar update (entry path with two optional phases).
///
/// `this` is the progress bar object. The update runs three straight-line
/// call sequences with two gates between them. Seven frame words (`s16`,
/// `s28`, `s24`, `s8`, `s4`, `s12`, `s20`, named for their position below
/// the incoming stack pointer) carry values between the calls; helpers 2
/// and 3 fill them through out-pointers, and helper 4 consumes helper 3's
/// two argument words off the stack (a cross-call stack cake: helper 3
/// pops nothing, helper 4 pops all three words). Helper 2's first word
/// (`s16`) is produced but never read back.
///
/// First the inner object at `this + INNER (+4)` is set up (helper 1),
/// helper 2 produces the first three frame words, helper 3 the next two,
/// and helper 4 takes helper 3's answer plus its two words. `this + TOTAL
/// (+0x100)` becomes helper 5's answer plus `s20` (wrapping add), and
/// `this + MARK (+0xa4)` becomes -1. When `this + PHASE (+0x104)` is
/// neither 1 nor 2, a second phase runs: helper 2 refreshes three words
/// (`s28`/`s24` zeroed first), and their values fan out through helpers 6
/// to 10 (helper 9 reuses helper 8's stack word as its second word).
/// Helper 11 then takes (`s12`, `s20`) with `this + 0x1c` in ECX,
/// helper 12 runs, and helper 13's answer is compared against `s8` as
/// UNSIGNED (`jae`): only when it is strictly below does the signal step
/// run (helper 14 with 7 or 6 chosen by the globals `MODE_G` and `FLAG_G`,
/// then two globals updated, one OR-ed with 2, one set to `s8`).
///
/// Helper 15 takes `s4`. When the gate byte `GATE_G` is nonzero, helpers
/// 16 and 17 run unless the inner object's byte at `+0x1b` is set, and
/// helper 18 always runs. The inner pointer is then reloaded: when that
/// inner byte is set the function returns the inner pointer itself;
/// otherwise helper 19 runs and the base update (tail (an instruction of the original)
/// whose answer is the result.
///
/// Helpers that the original calls without setting ECX (3, 5, 8, 13) take
/// whatever the previous stub left there; they pop nothing, so they are
/// declared cdecl and their ECX is not compared. The contract fills
/// unwritten stack with 0 (`stack_fill`): slot `s24` is never written on
/// the skip path yet is passed to helper 11, and the out-slots hold 0 when
/// their contents are snap-compared.
///
/// Original: thiscall, no stack arguments, 19 intercepted calls plus a
/// tail jump, word result in EAX.
lf_checker_rt::export!(thiscall, rw_00d70950(this: u32) -> u32 {
    const INNER: u32 = 0x4;
    const TOTAL: u32 = 0x100;
    const PHASE: u32 = 0x104;
    const MARK: u32 = 0xa4;
    const SETUP: u32 = 1;
    const PRODUCE: u32 = 2;
    const FILL2: u32 = 3;
    const CONSUME: u32 = 4;
    const MEASURE: u32 = 5;
    const USE_A: u32 = 6;
    const USE_B: u32 = 7;
    const USE_C: u32 = 8;
    const USE_D: u32 = 9;
    const USE_E: u32 = 10;
    const COMBINE: u32 = 11;
    const STEP: u32 = 12;
    const COMPARE: u32 = 13;
    const SIGNAL: u32 = 14;
    const PUSH_S4: u32 = 15;
    const GATE_A: u32 = 16;
    const GATE_B: u32 = 17;
    const GATE_C: u32 = 18;
    const FINISH: u32 = 19;
    const BASE: u32 = 20;
    const MODE_G: u32 = 0x0103_7720;
    const FLAG_G: u32 = 0x011f_701e;
    const OR_G: u32 = 0x011f_70e0;
    const SAVE_G: u32 = 0x011f_70e4;
    const GATE_G: u32 = 0x0103_f690;
    const INNER_FLAG: u32 = 0x1b;
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let edi = this;
        let inner = rd32(edi + INNER);
        lf_checker_rt::callee_thiscall!(SETUP, u32, inner, 0);
        let mut s16: u32 = 0;
        let mut s28: u32 = 0;
        let mut s24: u32 = 0;
        let mut s8: u32 = 0;
        let mut s4: u32 = 0;
        let mut s12: u32 = 0;
        let mut s20: u32 = 0;
        lf_checker_rt::callee_thiscall!(
            PRODUCE, u32, edi,
            core::ptr::addr_of_mut!(s16) as u32,
            core::ptr::addr_of_mut!(s8) as u32,
            core::ptr::addr_of_mut!(s4) as u32);
        ((edi + MARK) as *mut u32).write_unaligned(0xffff_ffff);
        let c1 = lf_checker_rt::callee_cdecl!(
            FILL2, u32,
            core::ptr::addr_of_mut!(s12) as u32,
            core::ptr::addr_of_mut!(s20) as u32);
        lf_checker_rt::callee_thiscall!(
            CONSUME, u32, edi, c1,
            core::ptr::addr_of_mut!(s12) as u32,
            core::ptr::addr_of_mut!(s20) as u32);
        let e = lf_checker_rt::callee_cdecl!(MEASURE, u32,);
        ((edi + TOTAL) as *mut u32)
            .write_unaligned(e.wrapping_add(s20));
        let phase = rd32(edi + PHASE);
        if phase != 1 && phase != 2 {
            s28 = 0;
            s24 = 0;
            lf_checker_rt::callee_thiscall!(
                PRODUCE, u32, edi,
                core::ptr::addr_of_mut!(s16) as u32,
                core::ptr::addr_of_mut!(s28) as u32,
                core::ptr::addr_of_mut!(s24) as u32);
            let a = s28;
            lf_checker_rt::callee_cdecl!(USE_A, u32, a);
            let b = lf_checker_rt::callee_thiscall!(USE_B, u32, edi, a, 0);
            lf_checker_rt::callee_cdecl!(USE_C, u32, b);
            let c = s24;
            lf_checker_rt::callee_cdecl!(USE_D, u32, c, b);
            let d = lf_checker_rt::callee_thiscall!(USE_B, u32, edi, c, 0);
            lf_checker_rt::callee_cdecl!(USE_E, u32, d);
        }
        lf_checker_rt::callee_thiscall!(
            COMBINE, u32, edi.wrapping_add(0x1c), s12, s20);
        lf_checker_rt::callee_thiscall!(STEP, u32, edi);
        let cmp_ans = lf_checker_rt::callee_cdecl!(COMPARE, u32,);
        let lo = s8;
        if cmp_ans < lo {
            let m = lf_checker_rt::global::<u32>(MODE_G).read_unaligned();
            let f = lf_checker_rt::global::<u8>(FLAG_G).read_unaligned();
            let sig = if (m == 2 || m == 7) && f != 0 { 7 } else { 6 };
            lf_checker_rt::callee_cdecl!(SIGNAL, u32, sig);
            let g = lf_checker_rt::global::<u32>(OR_G).read_unaligned();
            lf_checker_rt::global::<u32>(OR_G).write_unaligned(g | 2);
            lf_checker_rt::global::<u32>(SAVE_G).write_unaligned(lo);
        }
        lf_checker_rt::callee_thiscall!(PUSH_S4, u32, edi, s4);
        if lf_checker_rt::global::<u8>(GATE_G).read_unaligned() != 0 {
            let inner = rd32(edi + INNER);
            if ((inner + INNER_FLAG) as *const u8).read_unaligned() == 0 {
                lf_checker_rt::callee_thiscall!(GATE_A, u32, edi);
                lf_checker_rt::callee_thiscall!(GATE_B, u32, edi);
            }
            lf_checker_rt::callee_thiscall!(GATE_C, u32, edi);
        }
        let inner = rd32(edi + INNER);
        if ((inner + INNER_FLAG) as *const u8).read_unaligned() != 0 {
            inner
        } else {
            lf_checker_rt::callee_thiscall!(FINISH, u32, edi);
            lf_checker_rt::callee_thiscall!(BASE, u32, edi)
        }
    }
});
