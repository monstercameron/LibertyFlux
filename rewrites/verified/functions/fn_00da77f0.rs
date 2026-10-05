// original: 0x00DA77F0 CTaskComplexSmartFleePoint::vf20

/// Refresh the point-flee task: re-check the subtask, the time window and
/// two distances, then either keep, rebuild or replace the flee plan.
///
/// `this` is the task, `ped` the ped. The ped is marked fleeing (`+0x2A0`
/// bit `0x20`) and its flee timer at `+0xBA0` is set to `0x1B9`. When flag
/// bit 0 of `+0x6C` is set and the ped's kind-table entry byte at `+0xEE`
/// is below 3, a 10-argument setup call runs first.
///
/// With bit 1 set and bit 2 clear the task takes the short path: it clears
/// bit 1, and when the subtask's slot-`0xC` answer is `0x11D` it refreshes
/// the subtask through a 3-argument call taking the ped and `this+0x30`.
/// Otherwise the long path runs: unless `+0x64` is set the task drops to
/// the distance checks; a signed time gate (`+0x60 + +0x5C` at or below the
/// clock global, `+0x65` set re-arms it from the clock) and a slot-`0xC`
/// answer other than `0xCB` are required to reach the rebuild, which runs
/// the subtask's slot-`0x14` check (skipped when `+0xC` bit 0 is set),
/// flags the subtask, and issues two task calls, returning the second
/// answer directly.
///
/// The distance checks compare two squared distances around the flee
/// point against the squared radius at `+0x54`: first the `+0x30` point
/// (dx squared plus dy squared, plus dz squared), whose failure re-saves
/// the subtask and falls through, then the `+0x20` point (dy squared plus
/// dx squared, plus dz squared), whose success issues a `0xCB` task call
/// whose answer becomes the result. Both use "not above", so NaN fails.
///
/// Every other path ends at a tail check (a `0xC9D9C0` probe when `+0x58`
/// is 3, a fallback call when its low byte is clear) and returns the saved
/// subtask or, on the issue path, the issue answer. One narrowing: the
/// original reuses its incoming argument slot as scratch for the subtask
/// pointer (dead after return, unaddressable from Rust), so the stack
/// comparison is off and only that dead write goes unverified.
///
/// Original: 0x00DA77F0 (thiscall, one stack word, returns eax).
lf_checker_rt::export!(thiscall, rw_00DA77F0(this: u32, ped: u32) -> u32 {
    unsafe {
        const PED_FLEE_BIT: u32 = 0x2A0;
        const PED_FLEE_TIMER: u32 = 0xBA0;
        const PED_KIND: u32 = 0x2E;
        const PED_POS_SRC: u32 = 0x20;
        const PED_BLOCK_OFF: u32 = 0x570;
        const KIND_TABLE: u32 = 0x01295CD8;
        const KIND_LIMIT_OFF: u32 = 0xEE;
        const SETUP_BLOCK: u32 = 0x00EEF950;
        const SETUP_FLAG: u32 = 0x01284530;
        const CLOCK: u32 = 0x011735B4;
        const TASK_SUB: u32 = 0x8;
        const TASK_POINT_BX: u32 = 0x20;
        const TASK_POINT_BY: u32 = 0x24;
        const TASK_POINT_BZ: u32 = 0x28;
        const TASK_POINT_AX: u32 = 0x30;
        const TASK_POINT_AY: u32 = 0x34;
        const TASK_POINT_AZ: u32 = 0x38;
        const TASK_RADIUS: u32 = 0x54;
        const TASK_PROBE: u32 = 0x58;
        const TASK_T0: u32 = 0x5C;
        const TASK_SPAN: u32 = 0x60;
        const TASK_ARMED: u32 = 0x64;
        const TASK_REARM: u32 = 0x65;
        const TASK_FLAGS: u32 = 0x6C;
        const VEC_X: u32 = 0x30;
        const VEC_Y: u32 = 0x34;
        const VEC_Z: u32 = 0x38;
        const ONE_BITS: u32 = 0x3F800000;
        const FLEE_TIMER_VAL: u32 = 0x1B9;
        const WANT_REFRESH: u32 = 0x11D;
        const SKIP_REBUILD: u32 = 0xCB;
        const VT_SLOT_KIND: u32 = 0x0C;
        const VT_SLOT_CHECK: u32 = 0x14;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        wr32(ped.wrapping_add(PED_FLEE_BIT), rd32(ped.wrapping_add(PED_FLEE_BIT)) | 0x20);
        wr32(ped.wrapping_add(PED_FLEE_TIMER), FLEE_TIMER_VAL);
        let mut saved: u32 = rd32(this.wrapping_add(TASK_SUB));

        if rd32(this.wrapping_add(TASK_FLAGS)) & 1 != 0 {
            let kind: u32 = ((ped.wrapping_add(PED_KIND) as *const i16).read_unaligned() as i32) as u32;
            let table: u32 = lf_checker_rt::relocated(KIND_TABLE);
            let entry: u32 = rd32(table.wrapping_add(kind.wrapping_mul(4)));
            if (entry.wrapping_add(KIND_LIMIT_OFF) as *const u8).read() < 3 {
                let block: u32 = lf_checker_rt::relocated(SETUP_BLOCK);
                let flag: u32 = lf_checker_rt::global::<u32>(SETUP_FLAG).read();
                let _ = lf_checker_rt::callee_thiscall!(
                    3, u32, ped.wrapping_add(PED_BLOCK_OFF),
                    block, 0, 1, flag, 0xFFFF_FFFF, 0, 0, ONE_BITS, 0, 0
                );
            }
        }

        let flags: u32 = rd32(this.wrapping_add(TASK_FLAGS));
        let short: bool = flags & 2 != 0 && flags & 4 == 0;
        if short {
            let sub: u32 = rd32(this.wrapping_add(TASK_SUB));
            wr32(this.wrapping_add(TASK_FLAGS), flags & !2);
            let vt: u32 = rd32(sub);
            let kind_of: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(vt.wrapping_add(VT_SLOT_KIND)) as usize);
            if kind_of(sub) == WANT_REFRESH {
                let _ = lf_checker_rt::callee_stdcall!(
                    4, u32, ped, this.wrapping_add(TASK_POINT_AX), 0
                );
                saved = rd32(this.wrapping_add(TASK_SUB));
            }
        } else {
            // Long path.
            let mut to_distances: bool = false;
            if (this.wrapping_add(TASK_ARMED) as *const u8).read() == 0 {
                to_distances = true;
            } else {
                if (this.wrapping_add(TASK_REARM) as *const u8).read() != 0 {
                    wr32(this.wrapping_add(TASK_T0), lf_checker_rt::global::<u32>(CLOCK).read());
                    (this.wrapping_add(TASK_REARM) as *mut u8).write(0);
                }
                let now: u32 = lf_checker_rt::global::<u32>(CLOCK).read();
                let end: u32 =
                    rd32(this.wrapping_add(TASK_SPAN)).wrapping_add(rd32(this.wrapping_add(TASK_T0)));
                if (end as i32) > (now as i32) {
                    to_distances = true;
                } else {
                    let sub: u32 = rd32(this.wrapping_add(TASK_SUB));
                    let vt: u32 = rd32(sub);
                    let kind_of: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(rd32(vt.wrapping_add(VT_SLOT_KIND)) as usize);
                    if kind_of(sub) == SKIP_REBUILD {
                        to_distances = true;
                    } else {
                        if (sub.wrapping_add(0x0C) as *const u8).read() & 1 == 0 {
                            let check: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                                core::mem::transmute(rd32(vt.wrapping_add(VT_SLOT_CHECK)) as usize);
                            if check(sub, ped, 1, 0) & 0xFF == 0 {
                                // Falls through to the tail check below.
                                return tail_check(this, ped, saved);
                            }
                            ((sub.wrapping_add(0x0C)) as *mut u8)
                                .write((sub.wrapping_add(0x0C) as *const u8).read() | 2);
                        }
                        let _ = lf_checker_rt::callee_thiscall!(7, u32, this, ped);
                        return lf_checker_rt::callee_thiscall!(8, u32, this, 0x516, ped);
                    }
                }
            }
            if to_distances {
                // Distance checks.
                if rd32(this.wrapping_add(TASK_FLAGS)) & 4 == 0 {
                    let src: u32 = rd32(ped.wrapping_add(PED_POS_SRC));
                    let ax: f32 = fsub(rdf(this.wrapping_add(TASK_POINT_AX)), rdf(src.wrapping_add(VEC_X)));
                    let ay: f32 = fsub(rdf(this.wrapping_add(TASK_POINT_AY)), rdf(src.wrapping_add(VEC_Y)));
                    let az: f32 = fsub(rdf(this.wrapping_add(TASK_POINT_AZ)), rdf(src.wrapping_add(VEC_Z)));
                    let radius: f32 = rdf(this.wrapping_add(TASK_RADIUS));
                    let da2: f32 = add(add(mul(ax, ax), mul(ay, ay)), mul(az, az));
                    let rad2: f32 = mul(radius, radius);
                    if dist2_gt(da2, rad2) {
                        let bx: f32 = fsub(rdf(this.wrapping_add(TASK_POINT_BX)), rdf(src.wrapping_add(VEC_X)));
                        let by: f32 = fsub(rdf(this.wrapping_add(TASK_POINT_BY)), rdf(src.wrapping_add(VEC_Y)));
                        let bz: f32 = fsub(rdf(this.wrapping_add(TASK_POINT_BZ)), rdf(src.wrapping_add(VEC_Z)));
                        let db2: f32 = add(add(mul(by, by), mul(bx, bx)), mul(bz, bz));
                        if dist2_gt(db2, rad2) {
                            saved = lf_checker_rt::callee_thiscall!(8, u32, this, SKIP_REBUILD, ped);
                        }
                    } else {
                        saved = rd32(this.wrapping_add(TASK_SUB));
                    }
                }
            }
        }
        return tail_check(this, ped, saved);

        // Tail check shared by every path except the direct rebuild return.
        #[inline(always)]
        unsafe fn tail_check(this: u32, ped: u32, saved: u32) -> u32 {
            unsafe {
                const TASK_PROBE: u32 = 0x58;
                const PED_PROBE_OFF: u32 = 0xBB0;
                if (this.wrapping_add(TASK_PROBE) as *const u32).read_unaligned() == 3 {
                    let probe: u32 =
                        lf_checker_rt::callee_thiscall!(5, u32, ped.wrapping_add(PED_PROBE_OFF));
                    if probe & 0xFF == 0 {
                        let _ = lf_checker_rt::callee_stdcall!(6, u32,);
                    }
                }
                saved
            }
        }
        #[inline(always)]
        fn dist2_gt(d2: f32, r2: f32) -> bool {
            core::hint::black_box(d2) > core::hint::black_box(r2)
        }
    }
});
