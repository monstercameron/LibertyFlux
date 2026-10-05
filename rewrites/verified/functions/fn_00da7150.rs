// original: 0x00DA7150 CTaskComplexFleeAnyMeans::vf20

/// Pick how the ped flees: by vehicle when one is usable, otherwise by a
/// plain flee task issued through the shared dispatcher.
///
/// `this` is the task, `ped` the ped (marked fleeing at `+0x2A0` up
/// front). With no threat holder (`+0x14` null) the subtask's slot-`0x14`
/// check runs and a `0x516` request goes to the dispatcher. Otherwise a
/// vehicle is located through one call; a selector derived from the
/// holder's `+0x28` bits must then pass a membership probe, the subtask
/// must report kind `0x3A0` or `0x38F`, the threat must be farther than
/// the `+0x24` radius, a guided-direction dot product must be positive
/// (the direction is the unit vector toward the vehicle scaled by a global
/// base length, or zero when the length is zero), and a 3-argument
/// clearance call must agree. The vehicle is then installed at `+0x38`
/// (releasing any previous one).
///
/// With `+0x19` set, each usable vehicle slot (the `+0xF50` head plus the
/// counted `+0xF54` array while its `+0x219` byte is clear) is offered
/// through a three-call chain that builds a stack record, queries it and
/// tears it down; the record bytes written by the function (a relocated
/// address constant and `0x2C2`) are what the snapshots compare, while the
/// record addresses themselves are skipped. The request code becomes
/// `0x385`. With `+0x19` clear a slot-`0x1B0` probe on the vehicle picks
/// between `0x2BE` and the default `0xC8`.
///
/// All of that converges on one check: the subtask must now report
/// `0x2BE`, a vehicle slot must be installed, the selector must pass the
/// probe again and a final clearance call must agree, in which case the
/// task's own slot-`0x4C` entry runs and its answer is returned. Failing
/// that, a non-default request code re-runs the slot-`0x14` check and goes
/// to the dispatcher with the code; the default code returns the subtask.
///
/// Original: 0x00DA7150 (thiscall, one stack word, returns eax).
lf_checker_rt::export!(thiscall, rw_00DA7150(this: u32, ped: u32) -> u32 {
    unsafe {
        const PED_FLEE_BIT: u32 = 0x2A0;
        const PED_POS_SRC: u32 = 0x20;
        const PED_DRIVER: u32 = 0x224;
        const DRIVER_OFF: u32 = 0xF4;
        const TASK_SUB: u32 = 0x8;
        const TASK_HOLDER: u32 = 0x14;
        const TASK_USE_VEH: u32 = 0x19;
        const TASK_RADIUS: u32 = 0x24;
        const TASK_VEH: u32 = 0x38;
        const HOLDER_POS: u32 = 0x20;
        const HOLDER_FALLBACK: u32 = 0x10;
        const HOLDER_MODE: u32 = 0x28;
        const POS_OFF: u32 = 0x30;
        const GUIDE_BASE: u32 = 0x00FE88E8;
        const VEH_POS: u32 = 0x20;
        const VEH_HEAD: u32 = 0xF50;
        const VEH_SLOTS: u32 = 0xF54;
        const VEH_COUNT: u32 = 0x1070;
        const SLOT_FLAG: u32 = 0x219;
        const SLOT_DISP: u32 = 0x224;
        const SLOT_POS: u32 = 0x20;
        const DISP_OFF: u32 = 0x84;
        const REC_ADDR_CONST: u32 = 0x00E9CA74;
        const REC_KIND_CONST: u32 = 0x2C2;
        const ONE_BITS: u32 = 0x3F800000;
        const KIND_A: u32 = 0x3A0;
        const KIND_B: u32 = 0x38F;
        const KIND_C: u32 = 0x2BE;
        const CODE_VEH: u32 = 0x385;
        const CODE_PLAIN: u32 = 0x516;
        const CODE_DEFAULT: u32 = 0xC8;
        const VT_SLOT_KIND: u32 = 0x0C;
        const VT_SLOT_CHECK: u32 = 0x14;
        const VT_SLOT_PROBE: u32 = 0x1B0;
        const VT_SLOT_RUN: u32 = 0x4C;

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
        #[inline(always)]
        fn gt(a: f32, b: f32) -> bool {
            core::hint::black_box(a) > core::hint::black_box(b)
        }
        // Offer one vehicle slot through the build/query/teardown chain.
        // The build call takes the vehicle (its position words), while the
        // query call's object comes from the slot; both record addresses
        // are the same stack slot.
        #[inline(always)]
        unsafe fn offer_slot(veh: u32, slot: u32, ped: u32) {
            unsafe {
                const ONE_BITS: u32 = 0x3F800000;
                const REC_ADDR_CONST: u32 = 0x00E9CA74;
                const REC_KIND_CONST: u32 = 0x2C2;
                const SLOT_DISP: u32 = 0x224;
                const VEH_POS: u32 = 0x20;
                const DISP_OFF: u32 = 0x84;
                const POS_OFF: u32 = 0x30;
                let pos_base: u32 = (veh.wrapping_add(VEH_POS) as *const u32).read_unaligned();
                let mut rec_a: [u32; 8] = [0; 8];
                let _ = lf_checker_rt::callee_thiscall!(
                    9, u32, (&mut rec_a as *mut u32) as u32,
                    veh, ped, 7, ONE_BITS, pos_base.wrapping_add(POS_OFF), pos_base
                );
                let buf_b: *mut u32 = rec_a.as_mut_ptr();
                buf_b.write(lf_checker_rt::relocated(REC_ADDR_CONST));
                buf_b.wrapping_add(4).write(REC_KIND_CONST);
                let disp: u32 = (slot.wrapping_add(SLOT_DISP) as *const u32).read_unaligned();
                let _ = lf_checker_rt::callee_thiscall!(
                    10, u32, disp.wrapping_add(DISP_OFF), buf_b as u32, 0, 1
                );
                let _ = lf_checker_rt::callee_thiscall!(11, u32, buf_b as u32);
            }
        }

        wr32(ped.wrapping_add(PED_FLEE_BIT), rd32(ped.wrapping_add(PED_FLEE_BIT)) | 0x20);
        let mut code: u32 = CODE_DEFAULT;
        let holder: u32 = rd32(this.wrapping_add(TASK_HOLDER));
        if holder == 0 {
            let sub: u32 = rd32(this.wrapping_add(TASK_SUB));
            if (sub.wrapping_add(0x0C) as *const u8).read() & 1 == 0 {
                let vt: u32 = rd32(sub);
                let check: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(vt.wrapping_add(VT_SLOT_CHECK)) as usize);
                if check(sub, ped, 0, 0) & 0xFF == 0 {
                    return sub;
                }
                ((sub.wrapping_add(0x0C)) as *mut u8)
                    .write((sub.wrapping_add(0x0C) as *const u8).read() | 2);
            }
            return lf_checker_rt::callee_thiscall!(3, u32, this, CODE_PLAIN, ped);
        }

        // Main path.
        let driver: u32 = rd32(ped.wrapping_add(PED_DRIVER));
        let veh: u32 = lf_checker_rt::callee_thiscall!(4, u32, driver.wrapping_add(DRIVER_OFF));
        let sel: u32 = if rd32(holder.wrapping_add(HOLDER_MODE)) & 0x3C0 == 0xC0 {
            holder
        } else {
            0
        };
        let mut converged_early: bool = false;
        if veh == 0 {
            converged_early = true;
        } else {
            if sel != 0 {
                if lf_checker_rt::callee_stdcall!(5, u32, sel) & 0xFF != 0 {
                    converged_early = true;
                }
            }
            if !converged_early {
                let sub: u32 = rd32(this.wrapping_add(TASK_SUB));
                let vt: u32 = rd32(sub);
                let kind_of: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt.wrapping_add(VT_SLOT_KIND)) as usize);
                let k1: u32 = kind_of(sub);
                if k1 != KIND_A && kind_of(sub) != KIND_B {
                    converged_early = true;
                } else {
                    // Threat-distance gate.
                    let h: u32 = rd32(this.wrapping_add(TASK_HOLDER));
                    let p: u32 = rd32(h.wrapping_add(HOLDER_POS));
                    let apos: u32 = if p != 0 { p.wrapping_add(POS_OFF) } else { h.wrapping_add(HOLDER_FALLBACK) };
                    let dpos: u32 = rd32(ped.wrapping_add(PED_POS_SRC));
                    let dx: f32 = fsub(rdf(dpos.wrapping_add(POS_OFF)), rdf(apos));
                    let dy: f32 = fsub(rdf(dpos.wrapping_add(POS_OFF + 4)), rdf(apos.wrapping_add(4)));
                    let dz: f32 = fsub(rdf(dpos.wrapping_add(POS_OFF + 8)), rdf(apos.wrapping_add(8)));
                    let radius: f32 = rdf(this.wrapping_add(TASK_RADIUS));
                    let dist2: f32 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                    if !gt(dist2, mul(radius, radius)) {
                        converged_early = true;
                    } else {
                        // Guided-direction gate.
                        let vpos: u32 = rd32(veh.wrapping_add(VEH_POS));
                        let gx: f32 = fsub(rdf(vpos.wrapping_add(POS_OFF)), rdf(dpos.wrapping_add(POS_OFF)));
                        let gy: f32 = fsub(rdf(vpos.wrapping_add(POS_OFF + 4)), rdf(dpos.wrapping_add(POS_OFF + 4)));
                        let gz: f32 = fsub(rdf(vpos.wrapping_add(POS_OFF + 8)), rdf(dpos.wrapping_add(POS_OFF + 8)));
                        let len2: f32 = add(add(mul(gy, gy), mul(gx, gx)), mul(gz, gz));
                        let base: f32 = lf_checker_rt::global::<f32>(GUIDE_BASE).read();
                        let scale: f32 = if len2 == 0.0 {
                            0.0
                        } else {
                            let root: f32 = unsafe {
                                core::arch::x86::_mm_cvtss_f32(core::arch::x86::_mm_sqrt_ss(
                                    core::arch::x86::_mm_set_ss(core::hint::black_box(len2)),
                                ))
                            };
                            core::hint::black_box(base) / core::hint::black_box(root)
                        };
                        let ux: f32 = mul(gx, scale);
                        let uy: f32 = mul(gy, scale);
                        let uz: f32 = mul(gz, scale);
                        let dot: f32 = add(
                            add(mul(rdf(dpos.wrapping_add(0x14)), uy), mul(rdf(dpos.wrapping_add(0x10)), ux)),
                            mul(rdf(dpos.wrapping_add(0x18)), uz),
                        );
                        if !gt(dot, 0.0) {
                            converged_early = true;
                        } else if lf_checker_rt::callee_cdecl!(6, u32, veh, ped, 1) & 0xFF == 0 {
                            converged_early = true;
                        } else {
                            // Install the vehicle slot.
                            let slot_ptr: u32 = this.wrapping_add(TASK_VEH);
                            if rd32(slot_ptr) != 0 {
                                let _ = lf_checker_rt::callee_stdcall!(7, u32, slot_ptr);
                                wr32(slot_ptr, 0);
                            }
                            wr32(slot_ptr, veh);
                            let _ = lf_checker_rt::callee_thiscall!(8, u32, veh, slot_ptr);
                            if (this.wrapping_add(TASK_USE_VEH) as *const u8).read() != 0 {
                                code = CODE_VEH;
                                let slot0: u32 = rd32(rd32(slot_ptr).wrapping_add(VEH_HEAD));
                                if slot0 != 0
                                    && (slot0.wrapping_add(SLOT_FLAG) as *const u8).read() == 0
                                {
                                    offer_slot(veh, slot0, ped);
                                }
                                let count: u32 = (rd32(slot_ptr).wrapping_add(VEH_COUNT) as *const u8).read() as u32;
                                if count != 0 {
                                    let mut k: u32 = 0;
                                    while k < count {
                                        let slot: u32 =
                                            rd32(rd32(slot_ptr).wrapping_add(VEH_SLOTS).wrapping_add(k.wrapping_mul(4)));
                                        if slot != 0 && (slot.wrapping_add(SLOT_FLAG) as *const u8).read() == 0 {
                                            offer_slot(veh, slot, ped);
                                        }
                                        k += 1;
                                    }
                                }
                            } else {
                                let vt: u32 = rd32(veh);
                                let probe: extern "thiscall" fn(u32, u32) -> u32 =
                                    core::mem::transmute(rd32(vt.wrapping_add(VT_SLOT_PROBE)) as usize);
                                code = if probe(veh, 0) & 0xFF != 0 { KIND_C } else { CODE_DEFAULT };
                            }
                        }
                    }
                }
            }
        }
        // Converge check: every route above reaches it (early exits jump
        // here, the install/multi/single block falls through).
        let _ = converged_early;
        let sub: u32 = rd32(this.wrapping_add(TASK_SUB));
        let vt: u32 = rd32(sub);
        let kind_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(VT_SLOT_KIND)) as usize);
        if kind_of(sub) == KIND_C
            && rd32(this.wrapping_add(TASK_VEH)) != 0
            && sel != 0
            && lf_checker_rt::callee_stdcall!(13, u32, sel) & 0xFF != 0
            && lf_checker_rt::callee_stdcall!(14, u32, ped, 0, 0) & 0xFF != 0
        {
            let vt_this: u32 = rd32(this);
            let run: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(vt_this.wrapping_add(VT_SLOT_RUN)) as usize);
            return run(this, ped);
        }
        if code == CODE_DEFAULT {
            return sub;
        }
        if (sub.wrapping_add(0x0C) as *const u8).read() & 1 == 0 {
            let check: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(vt.wrapping_add(VT_SLOT_CHECK)) as usize);
            if check(sub, ped, 0, 0) & 0xFF == 0 {
                return sub;
            }
            ((sub.wrapping_add(0x0C)) as *mut u8)
                .write((sub.wrapping_add(0x0C) as *const u8).read() | 2);
        }
        lf_checker_rt::callee_thiscall!(3, u32, this, code, ped)
    }
});
