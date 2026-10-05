// original: 0x00cd33e0 task_flag_update

/// Update a task flag through probe gates, then steer by measured distance.
///
/// `this` is the task object, `arg0` the subject record and `arg1` a word
/// forwarded to the steer callee. Returns nothing (eax is incidental).
///
/// Behaviour: the flag section requires the enable byte, a non-null record
/// with the ready word and stamp, and the run bit; it clears the run bit
/// when the second probe value exceeds the first, when the gate probe
/// agrees with the record flag, or through a two-probe vote combined with
/// the mode bits (each probe pair is an enable check plus an exact-answer
/// comparison). The measure section fetches a point through the measure
/// callee and compares its squared distance from the subject geometry
/// against the squared bound: the near path builds two steering structs
/// and runs the steer chain, while the far path re-checks the run bit and
/// a helper pointer and either runs a longer steer chain or only releases
/// the helper struct. Every callee struct the original builds on its stack
/// is zero-filled; the rewrite passes zeroed locals the same way.
///
/// Original: 0x00cd33e0 (thiscall, two stack words; no return value).
#[allow(clippy::all)]
#[allow(unsafe_code)]
unsafe fn run_00cd33e0(this: u32, arg0: u32, arg1: u32, mutate_vote: bool) -> u32 {
    unsafe {
        const PROBE_A_CALLEE: u32 = 1;
        const PROBE_B_CALLEE: u32 = 2;
        const GATE_CALLEE: u32 = 3;
        const ENA_A_CALLEE: u32 = 4;
        const VOTE_A_CALLEE: u32 = 5;
        const MEASURE_CALLEE: u32 = 6;
        const BUILD_CALLEE: u32 = 7;
        const STEER_CALLEE: u32 = 8;
        const RELEASE_CALLEE: u32 = 9;
        const HELPER_CALLEE: u32 = 10;
        const CHECK_CALLEE: u32 = 11;
        const HELPER2_CALLEE: u32 = 12;
        const ENA_B_CALLEE: u32 = 13;
        const VOTE_B_CALLEE: u32 = 14;
        const G_BOUND: u32 = 0x1057098;
        const G_BOUND2: u32 = 0x105709c;
        const G_DT: u32 = 0x11735bc;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
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
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(lf_checker_rt::global::<u32>(va).read()) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        // Flag section. Every exit falls into the measure section.
        let rec = rd32(this.wrapping_add(0x24));
        if rd8(this.wrapping_add(0x21)) != 0
            && rec != 0
            && rd16(rec.wrapping_add(0x44)) == 1
            && rd32(rec.wrapping_add(0x40)) != 0
            && rd8(this.wrapping_add(0x5e)) & 1 != 0
        {
            let v1 =
                lf_checker_rt::callee_thiscall!(PROBE_A_CALLEE, f64, rd32(this.wrapping_add(0x14)))
                    as f32;
            let v2 =
                lf_checker_rt::callee_thiscall!(PROBE_B_CALLEE, f64, rd32(this.wrapping_add(0x24)))
                    as f32;
            if v2 > v1 {
                wr8(
                    this.wrapping_add(0x5e),
                    rd8(this.wrapping_add(0x5e)) & 0xfe,
                );
            }
            if rd8(this.wrapping_add(0x5e)) & 1 != 0 {
                let gate: u32 = lf_checker_rt::callee_thiscall!(
                    GATE_CALLEE,
                    u32,
                    rd32(this.wrapping_add(0x14))
                );
                if gate as u8 != 0 && rd8(rec.wrapping_add(0x74)) & 0x10 != 0 {
                    wr8(
                        this.wrapping_add(0x5e),
                        rd8(this.wrapping_add(0x5e)) & 0xfe,
                    );
                }
                if rd8(this.wrapping_add(0x5e)) & 1 != 0 {
                    let ena_a: u32 = lf_checker_rt::callee_thiscall!(
                        ENA_A_CALLEE,
                        u32,
                        rd32(this.wrapping_add(0x24)),
                        0x80u32
                    );
                    let cl: u8 = if ena_a as u8 != 0 {
                        let va: u32 = lf_checker_rt::callee_thiscall!(
                            VOTE_A_CALLEE,
                            u32,
                            rd32(this.wrapping_add(0x24)),
                            0x200u32,
                            0u32,
                            1.0f32.to_bits()
                        );
                        (va == 2) as u8
                    } else {
                        0
                    };
                    let ena_b: u32 = lf_checker_rt::callee_thiscall!(
                        ENA_B_CALLEE,
                        u32,
                        rd32(this.wrapping_add(0x24)),
                        0x80u32
                    );
                    let dl: u8 = if ena_b as u8 != 0 {
                        let vb: u32 = lf_checker_rt::callee_thiscall!(
                            VOTE_B_CALLEE,
                            u32,
                            rd32(this.wrapping_add(0x24)),
                            0x400u32,
                            0u32,
                            1.0f32.to_bits()
                        );
                        (vb == 2) as u8
                    } else {
                        0
                    };
                    // MUTANT (mut_00cd33e0): the vote never clears the bit.
                    let mut clear = false;
                    if !(cl == 0 && dl == 0) {
                        let al = rd8(this.wrapping_add(0x5e));
                        if al & 2 != 0 {
                            clear = true;
                        } else if cl != 0 {
                            let ah = rd8(this.wrapping_add(0x5d));
                            let c1 = (ah >> 4) & 1;
                            if c1 != 0 {
                                // Every sub-case clears: dl==0 and
                                // ah&0x40==0 fall into the retest, which
                                // still sees c1 set.
                                clear = true;
                            } else if dl != 0 && rd8(this.wrapping_add(0x5d)) & 0x40 != 0 {
                                clear = true;
                            }
                        } else if dl != 0 && rd8(this.wrapping_add(0x5d)) & 0x40 != 0 {
                            clear = true;
                        }
                    }
                    if mutate_vote {
                        clear = false;
                    }
                    if clear {
                        let al = rd8(this.wrapping_add(0x5e));
                        wr8(this.wrapping_add(0x5e), al & 0xfe);
                    }
                }
            }
        }

        // Measure section.
        let rec1c = rd32(this.wrapping_add(0x1c));
        if rec1c != 0 {
            let mut pt = [0u32; 2];
            let _: u32 = lf_checker_rt::callee_cdecl!(
                MEASURE_CALLEE,
                u32,
                pt.as_mut_ptr() as u32,
                rec1c
            );
            let d = rd32(arg0.wrapping_add(0x20));
            let dx = sub(rdf(d.wrapping_add(0x30)), f32::from_bits(pt[0]));
            let dy = sub(rdf(d.wrapping_add(0x34)), f32::from_bits(pt[1]));
            let dist2 = add(mul(dx, dx), mul(dy, dy));
            let bound = gf(G_BOUND);
            let bound2 = mul(bound, bound);
            if bound2 > dist2 {
                // Near path.
                let bound_b = gf(G_BOUND2);
                let mut s1 = [0u32; 2];
                let mut s2 = [bound.to_bits(), bound_b.to_bits()];
                let mut s3 = [0u32; 2];
                let mut main = [0u32; 4];
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    BUILD_CALLEE,
                    u32,
                    main.as_mut_ptr() as u32,
                    0u32,
                    s3.as_mut_ptr() as u32,
                    s2.as_mut_ptr() as u32,
                    s1.as_mut_ptr() as u32
                );
                let dt = gf(G_DT);
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    STEER_CALLEE,
                    u32,
                    main.as_mut_ptr() as u32,
                    arg0,
                    rec1c,
                    dt.to_bits(),
                    0u32,
                    1u32,
                    1u32
                );
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    RELEASE_CALLEE,
                    u32,
                    main.as_mut_ptr() as u32
                );
            } else {
                // Far path.
                if rd8(this.wrapping_add(0x5e)) & 1 != 0 {
                    let h = rd32(this.wrapping_add(0x14));
                    if h != 0 {
                        let mut helper = [0u32; 2];
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            HELPER_CALLEE,
                            u32,
                            h,
                            helper.as_mut_ptr() as u32
                        );
                        let chk: u32 = lf_checker_rt::callee_thiscall!(
                            CHECK_CALLEE,
                            u32,
                            helper.as_mut_ptr() as u32,
                            arg0,
                            rec1c,
                            0u32,
                            0u32
                        );
                        if chk as u8 != 0 {
                            let mut main = [0u32; 4];
                            let _: u32 = lf_checker_rt::callee_thiscall!(
                                HELPER2_CALLEE,
                                u32,
                                h,
                                main.as_mut_ptr() as u32
                            );
                            let dt = gf(G_DT);
                            let _: u32 = lf_checker_rt::callee_thiscall!(
                                STEER_CALLEE,
                                u32,
                                main.as_mut_ptr() as u32,
                                arg0,
                                rec1c,
                                dt.to_bits(),
                                rd32(this.wrapping_add(0x24)),
                                0u32,
                                arg1
                            );
                            let _: u32 = lf_checker_rt::callee_thiscall!(
                                RELEASE_CALLEE,
                                u32,
                                main.as_mut_ptr() as u32
                            );
                        }
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            RELEASE_CALLEE,
                            u32,
                            helper.as_mut_ptr() as u32
                        );
                    }
                }
            }
        }
        0
    }
}

lf_checker_rt::export!(thiscall, rw_00cd33e0(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe { run_00cd33e0(this, arg0, arg1, false) }
});

lf_checker_rt::export!(thiscall, mut_00cd33e0(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe { run_00cd33e0(this, arg0, arg1, true) }
});
