// original: 0x00cf8b00 ladder_task_dispatch (proposed)

/// Dispatch a climb-ladder task event by id (thiscall: task, ped).
///
/// `this` is the climb-ladder task, `task` the event id, `ped` the ped.
/// Each arm ends by returning its last callee's answer (or 0, or by
/// faulting on a null handle where the original does):
///
/// - 0x120: look up the ped's 0x84 entry (thiscall pair); run the align
///   callee (cdecl: frame slot, state word, pitch float, `this+0x20`,
///   `this+0x30`); fetch the shared handle (null returns 0); run the
///   start callee (thiscall with the handle: subtask word, `this+0x20`,
///   `this+0x30`, frame slot, pitch float, flag byte twice, 6 when the
///   state word is 4 else 4).
/// - 0xcb: fetch the shared handle (null returns 0); run the speed callee
///   (thiscall: 0x3e8 when `this+8` is set else 1, 0, 0, 8.0).
/// - 0x191: return the ladder anim-request callee over (state, flag byte).
/// - 0x386: smooth the pitch (raw `[this+0x70]`, or the x87 filter callee
///   over pitch + pi when the state is not 1 and the flag byte is 0);
///   fetch two handles (first null returns 0); without the second, run
///   the finish callee with a 0 result, else run the blend callee
///   (thiscall: smoothed pitch, 2.0, 0.02) and finish with its answer.
/// - default (anything else, e.g. 0x516): run the fallback callee with
///   (`this`, `ped`), return 0.
/// - 0x387/0x3ae: score the approach (see below), setting the flag byte;
///   0x387 then fetches two handles and finishes like 0x386 but through
///   the wide-blend callee (thiscall: 2.0, `this+0x40`, 0.2, 2.0, 0, 0);
///   0x3ae fetches one handle (null faults on `[0+0xdc]` like the
///   original), runs the full-blend callee (thiscall: 2.0, frame slot,
///   0.2, 3.0, -1, 1, 0, 0, 0, 1), sets bit 0x40 at result+0xdc, fetches
///   again (null returns 0) and finishes with the blend result.
/// - 0x3a6: smooth the pitch as in 0x386; when the state is 4, blend the
///   0x50-row by 0.1 into the 0x40-row (frame slots and `this+0x60..68`)
///   and store the unread frame slot (0 with the contract's zero stack
///   fill) at `this+0x6c`; fetch the handle (null faults on `[0+0xdc]`
///   like the original), run the commit callee (thiscall: frame slot,
///   smoothed pitch, 1000.0), stamp 2pi/0.1/1000 at result+0xdc/+0xe4/
///   +0xe8 and return the result.
///
/// Approach score: dx/dy from `this+0x20/0x24` minus target+0x30/0x34;
/// q = dy*dy + dx*dx; s = 1/sqrt(q) when q is positive or NaN (the
/// original's lahf/jp test), else 0; x1 = t14*(dy*s) + (dx*s)*t10 +
/// t18*(s*0), all in the original's operand order; the flag byte is 1
/// when -0.4 exceeds x1 (ordered) and the state is 4, else 0. The set
/// path also notifies (thiscall with (`[ped+0xa80]`, 1)) and retunes the
/// pitch (thiscall with (`ped`, pitch + pi)).
lf_checker_rt::export!(thiscall, rw_00cf8b00(this: u32, task: u32, ped: u32) -> u32 {
    unsafe {
        const ANIM_SET: u32 = 0x167e2a0;
        const SUBTASK: u32 = 8;
        const TASK_STATE: u32 = 0x14;
        const PITCH: u32 = 0x70;
        const FLAG75: u32 = 0x75;
        const FLAG74: u32 = 0x74;
        const STATE_WORD: u32 = 0x90;
        const PED_ANIM: u32 = 0xa80;
        const PED_SLOT: u32 = 0x78;
        const PED_TARGET: u32 = 0x20;
        const PI_BITS: u32 = 0x4049_0fdb;
        const DOT_LIMIT_BITS: u32 = 0xbecc_cccd;
        const BLEND_K_BITS: u32 = 0x3ca3_d70a;
        const WIDE_K_BITS: u32 = 0x3e4c_cccd;
        const FULL_K_BITS: u32 = 0x4040_0000;
        const STEP_K_BITS: u32 = 0x3dcc_cccd;
        const TWO: f32 = 2.0;
        const ONE: f32 = 1.0;
        const LOOKUP: u32 = 1;
        const ACQUIRE: u32 = 2;
        const ALIGN: u32 = 3;
        const FETCH: u32 = 4;
        const START: u32 = 5;
        const ANIMREQ: u32 = 6;
        const FILTER: u32 = 7;
        const BLEND: u32 = 8;
        const FINISH: u32 = 9;
        const FALLBACK: u32 = 10;
        const NOTIFY: u32 = 11;
        const RETUNE: u32 = 12;
        const WBLEND: u32 = 13;
        const FBLEND: u32 = 14;
        const COMMIT: u32 = 15;
        const SPEED: u32 = 16;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn fetch_shared() -> u32 {
            unsafe {
                let set = (lf_checker_rt::relocated(ANIM_SET) as *const u32).read_unaligned();
                lf_checker_rt::callee_thiscall!(FETCH, u32, set)
            }
        }
        #[inline(always)]
        unsafe fn smooth_pitch(this: u32) -> f32 {
            unsafe {
                let state = rd32(this + TASK_STATE);
                let flag = ((this + FLAG75) as *const u8).read();
                if state == 1 || flag != 0 {
                    rdf(this + PITCH)
                } else {
                    let arg = add(rdf(this + PITCH), f32::from_bits(PI_BITS));
                    lf_checker_rt::callee_cdecl!(FILTER, f32, arg.to_bits())
                }
            }
        }

        match task {
            0x120 => {
                let slot = rd32(ped + PED_SLOT);
                let entry = lf_checker_rt::callee_thiscall!(LOOKUP, u32, slot, 0x84);
                if entry != 0 {
                    lf_checker_rt::callee_thiscall!(ACQUIRE, u32, slot, entry);
                }
                let mut f1 = [0u32; 1];
                lf_checker_rt::callee_cdecl!(
                    ALIGN, u32, f1.as_mut_ptr() as u32, rd32(this + TASK_STATE),
                    rd32(this + PITCH), this + 0x20, this + 0x30
                );
                let h = fetch_shared();
                if h == 0 {
                    return 0;
                }
                let k = if rd32(this + TASK_STATE) == 4 { 6 } else { 4 };
                let flag = ((this + FLAG74) as *const u8).read() as u32;
                let mut f2 = [0u32; 1];
                lf_checker_rt::callee_thiscall!(
                    START, u32, h, rd32(this + STATE_WORD), this + 0x20,
                    this + 0x30, f2.as_mut_ptr() as u32, rd32(this + PITCH), flag, k
                )
            }
            0xcb => {
                let h = fetch_shared();
                if h == 0 {
                    return 0;
                }
                let speed = if rd32(this + SUBTASK) == 0 { 1 } else { 0x3e8 };
                lf_checker_rt::callee_thiscall!(SPEED, u32, h, speed, 0, 0, 0x4100_0000)
            }
            0x191 => {
                let flag = ((this + FLAG75) as *const u8).read() as u32;
                lf_checker_rt::callee_cdecl!(ANIMREQ, u32, rd32(this + TASK_STATE), flag)
            }
            0x386 => {
                let x = smooth_pitch(this);
                let h1 = fetch_shared();
                if h1 == 0 {
                    return 0;
                }
                let h2 = fetch_shared();
                let r = if h2 == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(
                        BLEND, u32, h2, x.to_bits(), TWO.to_bits(), BLEND_K_BITS
                    )
                };
                lf_checker_rt::callee_thiscall!(FINISH, u32, h1, r, 0, 0, 0)
            }
            0x387 | 0x3ae => {
                let tgt = rd32(ped + PED_TARGET);
                let dx = sub(rdf(this + 0x20), rdf(tgt + 0x30));
                let dy = sub(rdf(this + 0x24), rdf(tgt + 0x34));
                let q = add(mul(dy, dy), mul(dx, dx));
                let s = if q > 0.0 || q.is_nan() {
                    div(ONE, q.sqrt())
                } else {
                    0.0
                };
                let dy_s = mul(dy, s);
                let dx_s = mul(dx, s);
                let t3 = mul(dx_s, rdf(tgt + 0x10));
                let t1 = mul(rdf(tgt + 0x14), dy_s);
                let z = mul(s, 0.0);
                let t0 = mul(rdf(tgt + 0x18), z);
                let x1 = add(add(t1, t3), t0);
                let state = rd32(this + TASK_STATE);
                if f32::from_bits(DOT_LIMIT_BITS) > x1 && state == 4 {
                    ((this + FLAG75) as *mut u8).write(1);
                    let anim = rd32(ped + PED_ANIM);
                    lf_checker_rt::callee_thiscall!(NOTIFY, u32, anim, 1);
                    let tuned = add(rdf(this + PITCH), f32::from_bits(PI_BITS));
                    lf_checker_rt::callee_thiscall!(RETUNE, u32, ped, tuned.to_bits());
                } else {
                    ((this + FLAG75) as *mut u8).write(0);
                }
                if task == 0x387 {
                    let h1 = fetch_shared();
                    if h1 == 0 {
                        return 0;
                    }
                    let h2 = fetch_shared();
                    let r = if h2 == 0 {
                        0
                    } else {
                        lf_checker_rt::callee_thiscall!(
                            WBLEND, u32, h2, TWO.to_bits(), this + 0x40,
                            WIDE_K_BITS, TWO.to_bits(), 0, 0
                        )
                    };
                    lf_checker_rt::callee_thiscall!(FINISH, u32, h1, r, 0, 0, 0)
                } else {
                    let h = fetch_shared();
                    let mut fs = [0u32; 1];
                    let r = if h == 0 {
                        0
                    } else {
                        lf_checker_rt::callee_thiscall!(
                            FBLEND, u32, h, TWO.to_bits(), fs.as_mut_ptr() as u32,
                            WIDE_K_BITS, FULL_K_BITS, 0xffff_ffff, 1, 0, 0, 0, 1
                        )
                    };
                    let at = ((r + 0xdc) as *mut u32).read();
                    ((r + 0xdc) as *mut u32).write(at | 0x40);
                    let h2 = fetch_shared();
                    if h2 == 0 {
                        return 0;
                    }
                    lf_checker_rt::callee_thiscall!(FINISH, u32, h2, r, 0, 0, 0)
                }
            }
            0x3a6 => {
                let x = smooth_pitch(this);
                let v4 = rdf(this + 0x40);
                let v5 = rdf(this + 0x44);
                let v6 = rdf(this + 0x48);
                if rd32(this + TASK_STATE) == 4 {
                    let k = f32::from_bits(STEP_K_BITS);
                    let w3 = add(mul(rdf(this + 0x50), k), v4);
                    let w2 = add(mul(rdf(this + 0x54), k), v5);
                    let w1 = add(mul(rdf(this + 0x58), k), v6);
                    ((this + 0x6c) as *mut u32).write_unaligned(0);
                    ((this + 0x60) as *mut u32).write_unaligned(w3.to_bits());
                    ((this + 0x64) as *mut u32).write_unaligned(w2.to_bits());
                    ((this + 0x68) as *mut u32).write_unaligned(w1.to_bits());
                }
                let h = fetch_shared();
                let mut fs = [0u32; 1];
                let r = if h == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(
                        COMMIT, u32, h, fs.as_mut_ptr() as u32, x.to_bits(), 0x447a_0000
                    )
                };
                ((r + 0xdc) as *mut u32).write(0x40c9_0fdb);
                ((r + 0xe4) as *mut u32).write(0x3dcc_cccd);
                ((r + 0xe8) as *mut u32).write(0x3e8);
                r
            }
            _ => {
                lf_checker_rt::callee_thiscall!(FALLBACK, u32, this, ped);
                0
            }
        }
    }
});
