// original: 0x005B2AD0 STATS_NET_I_PC2
/// Service the network stats indicator and stage its display values.
///
/// Two gate calls plus a sticky flag select one of six outcomes: return
/// early when the second gate accepts while the flag is set; with the
/// first gate accepting, clear a set flag and return, else run the source
/// probe and, when it accepts, publish the mode word and two converted
/// display values (falling back to the first indicator string); with the
/// first gate rejecting, reset the mode outputs when the second gate
/// accepts, show the second indicator string when only the flag is set,
/// else return. Returns nothing meaningful.
lf_checker_rt::export!(cdecl, rb126_fn4() -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const C_GATE1: u32 = 1; // gate, cdecl/7 (8, 1, 2, zeros), al result
    const C_GATE2: u32 = 2; // gate, cdecl/7 (0xb, 1, zeros), al result
    const C_CLEAR: u32 = 3; // clear step, cdecl/0
    const C_SRC: u32 = 4; // source probe, cdecl/0
    const C_PROBE: u32 = 5; // entry probe, thiscall/0, al result
    const C_MODE: u32 = 6; // mode publish, cdecl/1
    const C_PAIR_A: u32 = 7; // float pair, cdecl/2 (out, 0x1d)
    const C_PAIR_B: u32 = 8; // float pair, cdecl/2 (out, 0x1d)
    const C_SET3: u32 = 9; // value set, thiscall/3 (value, 0xff, 0)
    const C_PRE: u32 = 10; // precondition, cdecl/0, al result
    const C_SHOW: u32 = 11; // show string, cdecl/8 (0, str, 4, 0, -1, 0, 0, 0)
    const C_RESET: u32 = 12; // reset step, cdecl/1

    // Globals (file VAs; resolved through the worker's image base).
    const G_FLAG_VA: u32 = 0x018B6CA5; // sticky flag byte
    const G_GATE_VA: u32 = 0x017F5FC6; // gate byte, must be clear to proceed
    const G_MODE_VA: u32 = 0x01160C40; // mode word (published, or reset to 4)
    const G_SLOT_VA: u32 = 0x018B6C94; // published mode slot
    const G_NEG_VA: u32 = 0x01030BA8; // reset to -1 on the reset path
    const G_ZERO_VA: u32 = 0x018E51CD; // reset to 0 on the reset path
    const G_ONE_VA: u32 = 0x01160B86; // set to 1 on the reset path
    const G_CTX1_VA: u32 = 0x01161548; // first value-set context
    const G_CTX2_VA: u32 = 0x01161578; // second value-set context
    const S_ONE_VA: u32 = 0x00F8A5E4; // first indicator string
    const S_TWO_VA: u32 = 0x00F8A5D4; // second indicator string

    /// `cvttss2si` with exact x86 semantics: truncation toward zero, and the
    /// integer-indefinite value for NaN and out-of-range inputs (Rust's `as`
    /// saturates instead, which differs exactly there).
    #[inline(always)]
    fn cvt_trunc(f: f32) -> u32 {
        if f.is_nan() || f >= 2147483648.0 || f < -2147483648.0 {
            0x8000_0000
        } else {
            f as i32 as u32
        }
    }

    #[inline(always)]
    unsafe fn pair_word(pair: u32, word: u32) -> u32 {
        *((pair.wrapping_add(word.wrapping_mul(4))) as *const u32)
    }

    unsafe {
        let mut out = [0u32; 2];
        let out_ptr = out.as_mut_ptr() as u32;
        let gate1 = lf_checker_rt::callee_cdecl!(C_GATE1, u32, 8, 1, 2, 0, 0, 0, 0) & 0xFF;
        let gate2 = lf_checker_rt::callee_cdecl!(C_GATE2, u32, 0xB, 1, 0, 0, 0, 0, 0) & 0xFF;
        let flag = *lf_checker_rt::global::<u8>(G_FLAG_VA);
        if gate2 != 0 && flag != 0 {
            return 0;
        }
        if gate1 != 0 {
            if flag != 0 {
                lf_checker_rt::callee_cdecl!(C_CLEAR, u32,);
                *lf_checker_rt::global::<u8>(G_FLAG_VA) = 0;
                return 0;
            }
            let src = lf_checker_rt::callee_cdecl!(C_SRC, u32,);
            if *lf_checker_rt::global::<u8>(G_GATE_VA) == 0 && src != 0 {
                let ok = lf_checker_rt::callee_thiscall!(C_PROBE, u32, src) & 0xFF;
                if ok != 0 {
                    let mode = *lf_checker_rt::global::<u32>(G_MODE_VA);
                    *lf_checker_rt::global::<u32>(G_SLOT_VA) = mode;
                    lf_checker_rt::callee_cdecl!(C_MODE, u32, 0x45);
                    let pa = lf_checker_rt::callee_cdecl!(C_PAIR_A, u32, out_ptr, 0x1D);
                    let va = cvt_trunc(f32::from_bits(pair_word(pa, 0)));
                    lf_checker_rt::callee_thiscall!(
                        C_SET3,
                        u32,
                        lf_checker_rt::relocated(G_CTX1_VA),
                        va,
                        0xFF,
                        0
                    );
                    let pb = lf_checker_rt::callee_cdecl!(C_PAIR_B, u32, out_ptr, 0x1D);
                    let vb = cvt_trunc(f32::from_bits(pair_word(pb, 0)));
                    lf_checker_rt::callee_thiscall!(
                        C_SET3,
                        u32,
                        lf_checker_rt::relocated(G_CTX2_VA),
                        vb,
                        0xFF,
                        0
                    );
                    return 0;
                }
            }
            if lf_checker_rt::callee_cdecl!(C_PRE, u32,) & 0xFF != 0 {
                return 0;
            }
            lf_checker_rt::callee_cdecl!(
                C_SHOW,
                u32,
                0,
                lf_checker_rt::relocated(S_ONE_VA),
                4,
                0,
                0xFFFF_FFFF,
                0,
                0,
                0
            );
            *lf_checker_rt::global::<u8>(G_FLAG_VA) = 1;
            return 0;
        }
        if gate2 != 0 {
            if flag == 0 {
                lf_checker_rt::callee_cdecl!(C_RESET, u32, 0);
                *lf_checker_rt::global::<u32>(G_MODE_VA) = 4;
                *lf_checker_rt::global::<u32>(G_NEG_VA) = 0xFFFF_FFFF;
                *lf_checker_rt::global::<u8>(G_ZERO_VA) = 0;
                *lf_checker_rt::global::<u8>(G_ONE_VA) = 1;
                return 0;
            }
        } else if flag == 0 {
            return 0;
        }
        lf_checker_rt::callee_cdecl!(
            C_SHOW,
            u32,
            0,
            lf_checker_rt::relocated(S_TWO_VA),
            4,
            0,
            0xFFFF_FFFF,
            0,
            0,
            0
        );
        0
    }
});
