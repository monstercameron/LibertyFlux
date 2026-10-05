// original: 0x00B72640 CTaskSimpleCarSetPedOut::vf17

/// Set-ped-out-of-car task step: detach the ped from its car, clear the
/// car's occupancy flags, and release the ped's task references. Always
/// returns 1.
///
/// `this` is the task object: bytes at `+0x1d`/`+0x1e`/`+0x1f`/`+0x20`/`+0x21`
/// and a sub-task pointer at `+0x14` (may be null; words at `+0x1304` and
/// `+0x118`, word index at `+0x2e`). `ped` is the ped object: a vtable at
/// `+0x00`, a body pointer at `+0x20`, words at `+0x24`, `+0x26c` (bit 2
/// selects the whole step, bit 19 is set at the end) and `+0x118`, bytes at
/// `+0x219` and `+0x36e` (read, then the latter cleared), pointers at `+0x6c`
/// (may be null; flag byte at `+0xe`), `+0x78` (notify target; low bit
/// cleared), `+0x224` (words at `+0x44` and `+0x54`), `+0x22c` (vtable call)
/// and the car at `+0xb30` (may be null). The car carries: a vtable at
/// `+0x00`, a mode word at `+0x28`, an occupant word at `+0xf50`, a state
/// word at `+0x12d0`, bytes at `+0x10c2`/`+0xf15`/`+0x1310` and a word at
/// `+0x1300`.
///
/// Steps: return 1 at once when the ped's bit 2 is clear. When the `+0x6c`
/// object exists and its flag byte is set, prod it through callee 1
/// (thiscall: 0) and return 1. Otherwise set the ped's bit 0, clear its bit
/// 2 and ping through callee 2 (thiscall on the ped). Then run the ped's
/// vtable slot `0x114` (callee 16: the ped, `0x11` when the task's `+0x1f`
/// byte is set else 1, low byte only), report through callee 3 (cdecl: the
/// ped, the task's `+0x1d` byte) and refresh through callee 4 (thiscall on
/// the ped). When the `+0x1d` byte is set, notify four times through callees
/// 5 and 6 (thiscall on the `+0x78` object: 3 or 1 with `0xC47A0000` or 0)
/// and clear the target's low bit. When the `+0x1f` and `+0x1e` bytes are
/// clear and the sub-task is missing or its `+0x1304` word differs from 2,
/// probe a scratch triple through callee 7 (thiscall on the ped: a scratch
/// address). When a car exists and its occupant is the ped, claim it through
/// callee 8 (thiscall on the car: 1), normalize its mode word (clear
/// `0x7400`, set `0x800` unless already `0xc00`) and reset a state of 5 to 1;
/// when the occupant differs, challenge through callee 9 (thiscall on the
/// car: the ped). Then copy the body's `+0x30`/`+0x34`/`+0x38` floats to
/// scratch and test them through callee 10 (cdecl: the scratch address): on
/// a nonzero low byte clear the car's `0x10c2` low two bits and its `0xf15`
/// low bit. When the `+0x1e`/`+0x1f` bytes are clear, a car exists and its
/// `+0x1300` word is 1, measure twice through the car's vtable slot `0xec`
/// (callee 17: the car, the scratch address): each answer points at a float,
/// and only when both absolute values are strictly below 0.1 does the step
/// poll through callee 11 (thiscall on the car) and, on a zero low byte, set
/// the car's `0x1310` bit 4. When the task's `+0x21` byte is set, fetch an
/// object through the `+0x22c` object's vtable slot `0x14` (callee 18: the
/// object, the ped); when that object exists and the `+0x54` object exists,
/// run its vtable slot `0xc` (callee 19: the object) and the fetched
/// object's slot `0xc` (callee 20: the fetched object): equal answers, or a
/// set marker (the `+0x54` object exists, the sub-task exists and its
/// `+0x1304` word is 1), release through the fetched object's slot 0
/// (callee 21: the object, 1), otherwise park through callee 12 (thiscall on
/// the `+0x224` object plus `0x44`: the fetched object, 4, 0). The tail runs
/// callee 13 (thiscall on the ped plus `0x2b0`: `0x10`, 0, the `+0x36e`
/// byte, 0), sets the ped's bit 19, clears its `+0x36e` byte, mirrors the
/// sub-task's `+0x118` bit 2 onto the ped's `+0x118` word when present, and
/// when the `+0x219` byte is set reports the sub-task's signed `+0x2e` word
/// (or -1 without a sub-task) through callee 14 (cdecl); independently,
/// when the task's `+0x20` byte is clear, the ped goes through callee 15
/// (cdecl).
///
/// All vtable and register targets come from heap objects and are planted by
/// the contract; the scratch addresses are frame pointers whose contents are
/// never observed (the callees taking them are scripted), so only their
/// addresses are skipped. The one-byte mode pushed as a full word is
/// compared low-byte-only. The float limit is the file's read-only 0.1.
/// Original: 0x00B72640 (thiscall, one stack word), returns 1 always.
lf_checker_rt::export!(thiscall, rw_00B72640(this: u32, ped: u32) -> u32 {
    /// Release through the fetched object's slot 0 (callee 21).
    unsafe fn release(got: u32) {
        unsafe {
            #[inline(always)]
            unsafe fn rd32(a: u32) -> u32 {
                unsafe { (a as *const u32).read_unaligned() }
            }
            let got_vt = rd32(got);
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(got_vt) as usize);
            f(got, 1);
        }
    }
    /// Shared tail: notify, set the done bits, mirror state, report.
    unsafe fn tail(ped: u32, this: u32) {
        unsafe {
            const TASK_SUB: u32 = 0x14;
            const TASK_B20: u32 = 0x20;
            const SUB_BITS: u32 = 0x118;
            const SUB_INDEX: u32 = 0x2e;
            const PED_CARBITS: u32 = 0x26c;
            const PED_MIRROR: u32 = 0x118;
            const PED_MARK: u32 = 0x219;
            const PED_SLOT: u32 = 0x36e;
            const TAIL_BIAS: u32 = 0x2b0;
            #[inline(always)]
            unsafe fn rd8(a: u32) -> u8 {
                unsafe { (a as *const u8).read() }
            }
            #[inline(always)]
            unsafe fn rd16(a: u32) -> u16 {
                unsafe { (a as *const u16).read_unaligned() }
            }
            #[inline(always)]
            unsafe fn rd32(a: u32) -> u32 {
                unsafe { (a as *const u32).read_unaligned() }
            }
            #[inline(always)]
            unsafe fn wr8(a: u32, v: u8) {
                unsafe { (a as *mut u8).write(v) }
            }
            #[inline(always)]
            unsafe fn wr32(a: u32, v: u32) {
                unsafe { (a as *mut u32).write_unaligned(v) }
            }
            lf_checker_rt::callee_thiscall!(
                13,
                u32,
                ped.wrapping_add(TAIL_BIAS),
                0x10,
                0,
                rd8(ped.wrapping_add(PED_SLOT)) as u32,
                0
            );
            wr32(
                ped.wrapping_add(PED_CARBITS),
                rd32(ped.wrapping_add(PED_CARBITS)) | 0x80000,
            );
            wr8(ped.wrapping_add(PED_SLOT), 0);
            let sub = rd32(this.wrapping_add(TASK_SUB));
            if sub != 0 && rd8(sub.wrapping_add(SUB_BITS)) & 0x04 != 0 {
                wr32(
                    ped.wrapping_add(PED_MIRROR),
                    rd32(ped.wrapping_add(PED_MIRROR)) | 0x04,
                );
            }
            if rd8(ped.wrapping_add(PED_MARK)) != 0 {
                let sub = rd32(this.wrapping_add(TASK_SUB));
                let index: u32 = if sub != 0 {
                    rd16(sub.wrapping_add(SUB_INDEX)) as i16 as i32 as u32
                } else {
                    0xffff_ffff
                };
                lf_checker_rt::callee_cdecl!(14, u32, index);
            }
            if rd8(this.wrapping_add(TASK_B20)) == 0 {
                lf_checker_rt::callee_cdecl!(15, u32, ped);
            }
        }
    }
    unsafe {
        const TASK_SUB: u32 = 0x14;
        const TASK_B1D: u32 = 0x1d;
        const TASK_B1E: u32 = 0x1e;
        const TASK_B1F: u32 = 0x1f;
        const TASK_B20: u32 = 0x20;
        const TASK_B21: u32 = 0x21;
        const SUB_STATE: u32 = 0x1304;
        const SUB_BITS: u32 = 0x118;
        const SUB_INDEX: u32 = 0x2e;
        const PED_VT: u32 = 0x00;
        const PED_BODY: u32 = 0x20;
        const PED_BITS: u32 = 0x24;
        const PED_CARBITS: u32 = 0x26c;
        const PED_MIRROR: u32 = 0x118;
        const PED_MARK: u32 = 0x219;
        const PED_SLOT: u32 = 0x36e;
        const PED_AUX: u32 = 0x6c;
        const AUX_FLAG: u32 = 0x0e;
        const PED_NOTIFY: u32 = 0x78;
        const PED_PARK: u32 = 0x224;
        const PED_FETCH: u32 = 0x22c;
        const PED_CAR: u32 = 0xb30;
        const PARK_BIAS: u32 = 0x44;
        const PARK_OBJ: u32 = 0x54;
        const CAR_VT: u32 = 0x00;
        const CAR_MODE: u32 = 0x28;
        const CAR_OCC: u32 = 0xf50;
        const CAR_STATE: u32 = 0x12d0;
        const CAR_B0: u32 = 0x10c2;
        const CAR_B1: u32 = 0xf15;
        const CAR_READY: u32 = 0x1300;
        const CAR_DONE: u32 = 0x1310;
        const MODE_MASK: u32 = 0x7c00;
        const MODE_WANT: u32 = 0x0c00;
        const MODE_CLEAR: u32 = 0xffff_8bff;
        const MODE_SET: u32 = 0x0800;
        const SLOT_SETPEDOUT: u32 = 0x114;
        const SLOT_MEASURE: u32 = 0xec;
        const SLOT_FETCH: u32 = 0x14;
        const SLOT_RUN: u32 = 0x0c;
        const SLOT_RELEASE: u32 = 0x00;
        const MODE_FULL: u8 = 0x11;
        const MODE_LITE: u8 = 0x01;
        const NOTE_ARG: u32 = 0xc47a0000;
        const LIMIT: f64 = f64::from_bits(0x3fb9_9999_9999_999a);
        const TAIL_BIAS: u32 = 0x2b0;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn vcall1(slot_at: u32, ecx: u32, arg: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(slot_at) as usize);
                f(ecx, arg)
            }
        }
        #[inline(always)]
        unsafe fn vcall0(slot_at: u32, ecx: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(slot_at) as usize);
                f(ecx)
            }
        }
        #[inline(always)]
        fn below_limit(bits: u32) -> bool {
            f64::from(f32::from_bits(bits & 0x7fff_ffff)) < LIMIT
        }

        if rd8(ped.wrapping_add(PED_CARBITS)) & 0x04 == 0 {
            return 1;
        }
        let aux = rd32(ped.wrapping_add(PED_AUX));
        if aux != 0 && rd8(aux.wrapping_add(AUX_FLAG)) != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, aux, 0);
            return 1;
        }
        wr32(
            ped.wrapping_add(PED_BITS),
            rd32(ped.wrapping_add(PED_BITS)) | 0x01,
        );
        wr32(
            ped.wrapping_add(PED_CARBITS),
            rd32(ped.wrapping_add(PED_CARBITS)) & !0x04,
        );
        lf_checker_rt::callee_thiscall!(2, u32, ped);
        let mode: u8 = if rd8(this.wrapping_add(TASK_B1F)) != 0 {
            MODE_FULL
        } else {
            MODE_LITE
        };
        let ped_vt = rd32(ped.wrapping_add(PED_VT));
        vcall1(ped_vt.wrapping_add(SLOT_SETPEDOUT), ped, mode as u32);
        lf_checker_rt::callee_cdecl!(3, u32, ped, rd8(this.wrapping_add(TASK_B1D)) as u32);
        lf_checker_rt::callee_thiscall!(4, u32, ped);
        if rd8(this.wrapping_add(TASK_B1D)) != 0 {
            let target = rd32(ped.wrapping_add(PED_NOTIFY));
            lf_checker_rt::callee_thiscall!(5, u32, target, 3, NOTE_ARG);
            lf_checker_rt::callee_thiscall!(6, u32, target, 3, 0);
            let target = rd32(ped.wrapping_add(PED_NOTIFY));
            lf_checker_rt::callee_thiscall!(5, u32, target, 1, NOTE_ARG);
            lf_checker_rt::callee_thiscall!(6, u32, target, 1, 0);
            let target = rd32(ped.wrapping_add(PED_NOTIFY));
            wr8(target, rd8(target) & !0x02);
        }
        if rd8(this.wrapping_add(TASK_B1F)) == 0
            && rd8(this.wrapping_add(TASK_B1E)) == 0
        {
            let sub = rd32(this.wrapping_add(TASK_SUB));
            if sub == 0 || rd32(sub.wrapping_add(SUB_STATE)) != 2 {
                let mut triple = [0u32; 3];
                let at = core::ptr::addr_of_mut!(triple) as u32;
                lf_checker_rt::callee_thiscall!(7, u32, ped, at);
            }
        }
        let mut spot = [0u32; 4];
        let spot_at = core::ptr::addr_of_mut!(spot) as u32;
        let car = rd32(ped.wrapping_add(PED_CAR));
        if car != 0 {
            if rd32(car.wrapping_add(CAR_OCC)) == ped {
                lf_checker_rt::callee_thiscall!(8, u32, car, 1);
                if rd32(car.wrapping_add(CAR_MODE)) & MODE_MASK != MODE_WANT
                    && rd32(car.wrapping_add(CAR_MODE)) & MODE_MASK != MODE_WANT
                {
                    let mode = rd32(car.wrapping_add(CAR_MODE));
                    wr32(car.wrapping_add(CAR_MODE), (mode & MODE_CLEAR) | MODE_SET);
                }
                if rd32(car.wrapping_add(CAR_STATE)) == 5 {
                    wr32(car.wrapping_add(CAR_STATE), 1);
                }
            } else {
                lf_checker_rt::callee_thiscall!(9, u32, car, ped);
            }
            let body = rd32(ped.wrapping_add(PED_BODY));
            wr32(spot_at, rd32(body.wrapping_add(0x30)));
            wr32(spot_at.wrapping_add(4), rd32(body.wrapping_add(0x34)));
            wr32(spot_at.wrapping_add(8), rd32(body.wrapping_add(0x38)));
            let tested: u32 = lf_checker_rt::callee_cdecl!(10, u32, spot_at);
            if tested & 0xff != 0 {
                let car = rd32(ped.wrapping_add(PED_CAR));
                wr8(
                    car.wrapping_add(CAR_B0),
                    rd8(car.wrapping_add(CAR_B0)) & 0xfc,
                );
                let car = rd32(ped.wrapping_add(PED_CAR));
                wr8(
                    car.wrapping_add(CAR_B1),
                    rd8(car.wrapping_add(CAR_B1)) & 0xfe,
                );
            }
        }
        if rd8(this.wrapping_add(TASK_B1E)) == 0
            && rd8(this.wrapping_add(TASK_B1F)) == 0
        {
            let car = rd32(ped.wrapping_add(PED_CAR));
            if car != 0 && rd32(car.wrapping_add(CAR_READY)) == 1 {
                let car_vt = rd32(car.wrapping_add(CAR_VT));
                let first: u32 = vcall1(car_vt.wrapping_add(SLOT_MEASURE), car, spot_at);
                if below_limit(rd32(first)) {
                    let car = rd32(ped.wrapping_add(PED_CAR));
                    let car_vt = rd32(car.wrapping_add(CAR_VT));
                    let second: u32 =
                        vcall1(car_vt.wrapping_add(SLOT_MEASURE), car, spot_at);
                    if below_limit(rd32(second.wrapping_add(4))) {
                        let car = rd32(ped.wrapping_add(PED_CAR));
                        let done: u32 = lf_checker_rt::callee_thiscall!(11, u32, car);
                        if done & 0xff == 0 {
                            let car = rd32(ped.wrapping_add(PED_CAR));
                            wr8(
                                car.wrapping_add(CAR_DONE),
                                rd8(car.wrapping_add(CAR_DONE)) | 0x10,
                            );
                        }
                    }
                }
            }
        }
        if rd8(this.wrapping_add(TASK_B21)) != 0 {
            let fetcher = rd32(ped.wrapping_add(PED_FETCH));
            let fetch_vt = rd32(fetcher);
            let got: u32 = vcall1(fetch_vt.wrapping_add(SLOT_FETCH), fetcher, ped);
            let _ = rd8(ped.wrapping_add(0xa60));
            let parked = rd32(ped.wrapping_add(PED_PARK));
            let runner = rd32(parked.wrapping_add(PARK_OBJ));
            let mut marker: u8 = 0;
            if runner != 0 {
                let sub = rd32(this.wrapping_add(TASK_SUB));
                if sub != 0 && rd32(sub.wrapping_add(SUB_STATE)) == 1 {
                    marker = 1;
                }
            }
            if got != 0 {
                if runner != 0 {
                    let run_vt = rd32(runner);
                    let side: u32 = vcall0(run_vt.wrapping_add(SLOT_RUN), runner);
                    let got_vt = rd32(got);
                    let main: u32 = vcall0(got_vt.wrapping_add(SLOT_RUN), got);
                    if main == side {
                        release(got);
                    } else if marker != 0 {
                        release(got);
                    } else {
                        lf_checker_rt::callee_thiscall!(
                            12,
                            u32,
                            parked.wrapping_add(PARK_BIAS),
                            got,
                            4,
                            0
                        );
                    }
                } else if marker != 0 {
                    release(got);
                } else {
                    lf_checker_rt::callee_thiscall!(
                        12,
                        u32,
                        parked.wrapping_add(PARK_BIAS),
                        got,
                        4,
                        0
                    );
                }
            }
        }
        tail(ped, this);
        1

        // (nested helpers follow the main flow)
    }

});
