// original: 0x00C7EB00 MOBILE_UH_HUH mobile-call scenario tick (proposed)

/// Mobile-call scenario tick: advance the phone-call task one step.
///
/// `this` is the call task, `ped` the ped on the call. Every tick clears
/// the output slot (`+0x2C`) and the started flag (`+0x31`), notifies the
/// ped's call block (`ped+0x570`, callee 1), then advances a counter:
/// the old count at `+0x28` is copied to `+0x24` and incremented. The old
/// count minus one selects one of eleven cases; anything above 10 takes
/// the default path, which returns 0 without further stores.
///
/// The cases: 0 blends a random value between two global floats into the
/// output slot (callee 2 primes the ped's audio block, callee 3 draws the
/// random integer, scaled by the global at `G_RAND_SCALE`); 1 resolves a
/// target through the ped's blocks (callees 4-6, including the virtual
/// call at slot `+0x12C`), flags the resolved block, seeds the output
/// slot and returns a scenario task from the singleton (callees 7-8), or
/// 0 when the singleton is null; 2 primes the audio block, seeds the
/// output slot and, for a type-`0x5D` task, resets the counter to 5;
/// 3 seeds the output slot and tail-calls the scenario builder (callee
/// 19), or returns 0 on a null singleton; 4 and 5 request a call line
/// (callee 10, two request shapes), mark started, optionally randomise
/// the output slot through callee 11 for type-`0x5D` tasks, and set the
/// counter to 7; 6 randomises the output slot (callee 11, two seed pairs
/// by task type), then sets the counter to 6 when a fresh random draw
/// scaled by `G_RAND_SCALE` exceeds the global at `G_CASE6_LIM`, else to
/// 8 or 9 by a signed comparison of `+0x34` against the global at
/// `G_CASE6_CMP`; 7 requests a chat or outro line (callee 13) unless a
/// type-`0x5D` task with a loud draw skips it, counts the confirmations
/// in `+0x34`, then rejoins case 4's tail; 8 requests a line (callee 14)
/// and marks started; 9 walks a two-object chain through the virtual
/// calls at slot `+0xC` (callees 16-17), clearing a flag bit and marking
/// the chain done, or marks the task stalled (`+0x30`) when the chain
/// disagrees; 10 only marks the task stalled.
///
/// The floating-point order is the original's; the `comiss`/`jbe` pairs
/// become ordered `>` tests (an unordered comparison takes the `jbe`
/// side). The tail call is a normal call returning its result.
///
/// Original: 0x00C7EB00 (thiscall, one stack word, returns eax).
lf_checker_rt::export!(thiscall, rw_00c7eb00(this: u32, ped: u32) -> u32 {
    unsafe {
        const PED_CALL: u32 = 0x570;
        const PED_AUDIO: u32 = 0x3C0;
        const PED_BLOCK: u32 = 0x2B0;
        const OUT_SLOT: u32 = 0x2C;
        const STARTED: u32 = 0x31;
        const STALLED: u32 = 0x30;
        const COUNT: u32 = 0x28;
        const PREV: u32 = 0x24;
        const TYPE: u32 = 0x14;
        const PHONE_TYPE: u32 = 0x5D;
        const CONFIRMS: u32 = 0x34;
        const VT_CALL_LINE: u32 = 0x12C;
        const VT_CHAIN: u32 = 0xC;
        const G_LO: u32 = 0x0104B920;
        const G_HI: u32 = 0x0104B924;
        const G_RAND_SCALE: u32 = 0x00FE8684;
        const G_SEED1: u32 = 0x0104B928;
        const G_SEED2: u32 = 0x0104B92C;
        const G_SEED3: u32 = 0x0104B930;
        const G_RNG_B: u32 = 0x0104B940;
        const G_RNG_A: u32 = 0x0104B93C;
        const G_RNG_D: u32 = 0x0104B938;
        const G_RNG_C: u32 = 0x0104B934;
        const G_CASE6_LIM: u32 = 0x0104B944;
        const G_CASE6_CMP: u32 = 0x0104B948;
        const G_CASE7_LIM: u32 = 0x00FE8830;
        const G_TASK_SYS: u32 = 0x0167E2A0;
        const ONE_BITS: u32 = 0x3F800000;
        const STR_INTRO: u32 = 0x00ED5230;
        const STR_UH_HUH: u32 = 0x00ED5240;
        const STR_CHAT: u32 = 0x00ED5250;
        const STR_MCHAT: u32 = 0x00ED5264;
        const STR_OUTRO: u32 = 0x00ED5270;
        const C_NOTIFY: u32 = 1;
        const C_PRIME: u32 = 2;
        const C_RAND: u32 = 3;
        const C_T4: u32 = 4;
        // Callee 5 (virtual slot VT_CALL_LINE) is reached through the
        // fabricated object, not the stub table.
        const C_T6: u32 = 6;
        const C_SINGLETON: u32 = 7;
        const C_SCEN: u32 = 8;
        const C_PRIME0: u32 = 9;
        const C_LINE45: u32 = 10;
        const C_RNG: u32 = 11;
        const C_PRIME6: u32 = 12;
        const C_LINE7: u32 = 13;
        const C_LINE8: u32 = 14;
        const C_CHAIN_END: u32 = 15;
        // Callees 16/17 (virtual slot VT_CHAIN) are reached through the
        // fabricated objects, not the stub table.
        const C_TAIL: u32 = 19;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrb(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
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
        #[inline(always)]
        unsafe fn g(a: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(a)) }
        }
        /// Case 4/5/7 tail: mark started, randomise the output slot for
        /// type-0x5D tasks, set the counter to 7.
        #[inline(always)]
        unsafe fn line_tail(task: u32) {
            unsafe {
                const OUT_SLOT: u32 = 0x2C;
                const STARTED: u32 = 0x31;
                const COUNT: u32 = 0x28;
                const TYPE: u32 = 0x14;
                const PHONE_TYPE: u32 = 0x5D;
                const COUNT7: u32 = 7;
                const G_RNG_B: u32 = 0x0104B940;
                const G_RNG_A: u32 = 0x0104B93C;
                const C_RNG: u32 = 11;
                wrb(task.wrapping_add(STARTED), 1);
                if rd32(task.wrapping_add(TYPE)) == PHONE_TYPE {
                    let r: f32 = lf_checker_rt::callee_cdecl!(
                        C_RNG,
                        f32,
                        g(G_RNG_A),
                        g(G_RNG_B)
                    );
                    wr32(task.wrapping_add(OUT_SLOT), r.to_bits());
                }
                wr32(task.wrapping_add(COUNT), COUNT7);
            }
        }

        wr32(this.wrapping_add(OUT_SLOT), 0);
        wrb(this.wrapping_add(STARTED), 0);
        lf_checker_rt::callee_thiscall!(C_NOTIFY, u32, ped.wrapping_add(PED_CALL));
        let old = rd32(this.wrapping_add(COUNT));
        wr32(this.wrapping_add(PREV), old);
        wr32(this.wrapping_add(COUNT), old.wrapping_add(1));
        let idx = old.wrapping_sub(1);
        if idx > 10 {
            return 0;
        }
        match idx {
            0 => {
                lf_checker_rt::callee_thiscall!(
                    C_PRIME,
                    u32,
                    ped.wrapping_add(PED_AUDIO),
                    0x2710
                );
                let lo = rdf(lf_checker_rt::relocated(G_LO));
                let hi = rdf(lf_checker_rt::relocated(G_HI));
                let draw: u32 = lf_checker_rt::callee_cdecl!(C_RAND, u32,);
                let r = (draw as i32) as f32;
                let span = sub(hi, lo);
                let v = add(mul(mul(r, rdf(lf_checker_rt::relocated(G_RAND_SCALE))), span), lo);
                wr32(this.wrapping_add(OUT_SLOT), v.to_bits());
                0
            }
            1 => {
                lf_checker_rt::callee_thiscall!(
                    C_T4,
                    u32,
                    ped.wrapping_add(PED_BLOCK),
                    0x2E,
                    1
                );
                let vt_call: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(
                        rd32(rd32(ped).wrapping_add(VT_CALL_LINE)) as usize
                    );
                let t = vt_call(ped, 0);
                let u: u32 = lf_checker_rt::callee_thiscall!(
                    C_T6,
                    u32,
                    ped.wrapping_add(PED_BLOCK),
                    ped,
                    t
                );
                if u != 0 {
                    let blk = rd32(ped.wrapping_add(0x2C4));
                    wr32(blk.wrapping_add(0x210), rd32(blk.wrapping_add(0x210)) | 0x04000000);
                }
                wr32(this.wrapping_add(OUT_SLOT), g(G_SEED1));
                let sys: u32 = lf_checker_rt::callee_thiscall!(
                    C_SINGLETON,
                    u32,
                    lf_checker_rt::global::<u32>(G_TASK_SYS).read()
                );
                if sys == 0 {
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(
                    C_SCEN,
                    u32,
                    sys,
                    7,
                    14,
                    0x40800000,
                    0,
                    ONE_BITS,
                    0
                )
            }
            2 => {
                lf_checker_rt::callee_thiscall!(
                    C_PRIME0,
                    u32,
                    ped.wrapping_add(PED_AUDIO)
                );
                wr32(this.wrapping_add(OUT_SLOT), g(G_SEED2));
                if rd32(this.wrapping_add(TYPE)) != PHONE_TYPE {
                    return 0;
                }
                const COUNT5: u32 = 5;
                wr32(this.wrapping_add(COUNT), COUNT5);
                0
            }
            3 => {
                wr32(this.wrapping_add(OUT_SLOT), g(G_SEED3));
                let sys: u32 = lf_checker_rt::callee_thiscall!(
                    C_SINGLETON,
                    u32,
                    lf_checker_rt::global::<u32>(G_TASK_SYS).read()
                );
                if sys == 0 {
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(C_TAIL, u32, sys, 0xFFFFFFFF)
            }
            4 => {
                lf_checker_rt::callee_thiscall!(
                    C_LINE45,
                    u32,
                    ped.wrapping_add(PED_CALL),
                    lf_checker_rt::relocated(STR_INTRO),
                    1,
                    1,
                    0,
                    0xFFFFFFFF,
                    0,
                    0,
                    ONE_BITS,
                    0,
                    0
                );
                line_tail(this);
                0
            }
            5 => {
                lf_checker_rt::callee_thiscall!(
                    C_LINE45,
                    u32,
                    ped.wrapping_add(PED_CALL),
                    lf_checker_rt::relocated(STR_UH_HUH),
                    0,
                    1,
                    0,
                    0xFFFFFFFF,
                    0,
                    0,
                    ONE_BITS,
                    0,
                    0
                );
                line_tail(this);
                0
            }
            6 => {
                if rd32(this.wrapping_add(TYPE)) == PHONE_TYPE {
                    wrb(this.wrapping_add(STARTED), 1);
                    let r: f32 = lf_checker_rt::callee_cdecl!(
                        C_RNG,
                        f32,
                        g(G_RNG_A),
                        g(G_RNG_B)
                    );
                    wr32(this.wrapping_add(OUT_SLOT), r.to_bits());
                    lf_checker_rt::callee_thiscall!(
                        C_PRIME6,
                        u32,
                        ped.wrapping_add(PED_AUDIO)
                    );
                } else {
                    let r: f32 = lf_checker_rt::callee_cdecl!(
                        C_RNG,
                        f32,
                        g(G_RNG_C),
                        g(G_RNG_D)
                    );
                    wr32(this.wrapping_add(OUT_SLOT), r.to_bits());
                }
                let draw: u32 = lf_checker_rt::callee_cdecl!(C_RAND, u32,);
                let r = (draw as i32) as f32;
                let v = mul(r, rdf(lf_checker_rt::relocated(G_RAND_SCALE)));
                if v > rdf(lf_checker_rt::relocated(G_CASE6_LIM)) {
                    wr32(this.wrapping_add(COUNT), 6);
                    return 0;
                }
                let n = rd32(this.wrapping_add(CONFIRMS)) as i32;
                let lim = g(G_CASE6_CMP) as i32;
                wr32(this.wrapping_add(COUNT), if n >= lim { 9 } else { 8 });
                0
            }
            7 => {
                let loud = if rd32(this.wrapping_add(TYPE)) == PHONE_TYPE {
                    let draw: u32 = lf_checker_rt::callee_cdecl!(C_RAND, u32,);
                    let r = (draw as i32) as f32;
                    mul(r, rdf(lf_checker_rt::relocated(G_RAND_SCALE)))
                        > rdf(lf_checker_rt::relocated(G_CASE7_LIM))
                } else {
                    false
                };
                let s = lf_checker_rt::relocated(if loud { STR_CHAT } else { STR_MCHAT });
                let ok: u32 = lf_checker_rt::callee_thiscall!(
                    C_LINE7,
                    u32,
                    ped.wrapping_add(PED_CALL),
                    s,
                    0,
                    0,
                    0,
                    0xFFFFFFFF,
                    0,
                    0,
                    ONE_BITS,
                    0,
                    0
                );
                if ok & 0xFF != 0 {
                    let n = rd32(this.wrapping_add(CONFIRMS));
                    wr32(this.wrapping_add(CONFIRMS), n.wrapping_add(1));
                }
                line_tail(this);
                0
            }
            8 => {
                lf_checker_rt::callee_thiscall!(
                    C_LINE8,
                    u32,
                    ped.wrapping_add(PED_CALL),
                    lf_checker_rt::relocated(STR_OUTRO),
                    0,
                    1,
                    0,
                    0xFFFFFFFF,
                    0,
                    0,
                    ONE_BITS,
                    0,
                    0
                );
                wrb(this.wrapping_add(STARTED), 1);
                0
            }
            9 => {
                let chain = rd32(this.wrapping_add(8));
                let v1call: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(chain).wrapping_add(VT_CHAIN)) as usize);
                if v1call(chain) != 0x11D {
                    wrb(this.wrapping_add(STALLED), 1);
                    return 0;
                }
                let link = rd32(chain.wrapping_add(8));
                if link == 0 {
                    wrb(this.wrapping_add(STALLED), 1);
                    return 0;
                }
                let v2call: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(link).wrapping_add(VT_CHAIN)) as usize);
                if v2call(link) == 0x640 {
                    lf_checker_rt::callee_stdcall!(C_CHAIN_END, u32, ped);
                    wr32(chain.wrapping_add(0x1C), 1);
                    return 0;
                }
                if v2call(link) != 0x190 {
                    wrb(this.wrapping_add(STALLED), 1);
                    wr32(chain.wrapping_add(0x1C), 1);
                    return 0;
                }
                let tail = rd32(link.wrapping_add(0x14));
                if tail == 0 {
                    wrb(this.wrapping_add(STALLED), 1);
                    wr32(chain.wrapping_add(0x1C), 1);
                    return 0;
                }
                let flags = rd32(tail.wrapping_add(4));
                if (flags >> 6) & 1 == 0 {
                    wr32(chain.wrapping_add(0x1C), 1);
                    return 0;
                }
                wr32(tail.wrapping_add(4), flags & !0x40);
                wr32(chain.wrapping_add(0x1C), 1);
                0
            }
            _ => {
                wrb(this.wrapping_add(STALLED), 1);
                0
            }
        }
    }
});
