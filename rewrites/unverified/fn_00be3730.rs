// original: 0x00be3730 CTaskComplexUseMobilePhone::vf19 (merged name)

/// Run a use-mobile-phone task tick: pick the call variant, set it up, run it.
///
/// `task` points to the task (`+0x2b` active flag, `+0x24` model slot,
/// `+0x2a` started flag); `ped` is the ped the task runs on (manager link at
/// `+0x78`, model link at `+0x2c4`, state byte at `+0x219`, vehicle link at
/// `+0xb30`, phone state at `+0x2b0`, vtable at `+0x0`).
///
/// The ped's phone manager (callee 1) is asked for the current call: a
/// non-null answer marks the task active and saves the call's progress float
/// (`+0x4c`). A variant flag (0xa or 0xb) is then chosen: 0xb when the task
/// is active, otherwise by comparing the linked model's id word (`+0x2e`,
/// sign-extended) against the ped's own id (virtual slot `+0x12c`) — equal
/// means 0xb, missing or different means 0xa. The ped is notified (callee 3)
/// when its state byte is set, and the phone state is reset (callee 4) when
/// the model link is missing or disagrees a second time. The ped's id is
/// stored in the model slot; an id of -1 ends the tick with 0. A vehicle
/// occupant check (callee 5) fires when the ped sits in the linked vehicle.
///
/// The model slot and a global parameter choose between two endings. When
/// the lookup (callee 6, cdecl) fails, a diagnostic (callee 7) runs and a
/// shared dispatcher (callee 8) is asked: an inactive task returns the
/// dispatcher's plain answer passed through callee 9, while an active task
/// builds a phone call through callee 10 (cdecl, six words) and returns
/// callee 11's answer. When the lookup succeeds, a global table entry for
/// the model is reference-counted (callee 12, which preserves its object
/// register and increments the word at `+0x44`), a call frame is prepared
/// (callees 13 and 14 take a pointer to it) and, unless the dispatcher
/// answer is null (which faults on the null object read, identically on
/// both sides), a second call is built (callee 18, eight words, carrying
/// the variant flag and an image pointer) and run (callee 15); the tick
/// returns the dispatcher's answer after driving it through virtual slot
/// `+0x44` and releasing the frame (callee 16).
///
/// Callees 10 and 11 (and 18 and 15) share one argument buildup: the caller
/// pops a single word between them, so the second call's stack arguments 1
/// and up are the first call's shifted words. A Rust rewrite cannot pop a
/// single word mid-sequence, so it pushes the faithful words (keeping the
/// stack balanced for the second callee's wide cleanup) while the contract
/// compares only argument 0 of callees 11 and 15 and all of 10 and 18: every
/// value is still compared once, on the first call.
///
/// Original: 0x00be3730 (thiscall, ecx = task, one stack word = ped).
lf_checker_rt::export!(thiscall, rw_00be3730(task: u32, ped: u32) -> u32 {
    unsafe {
        const ACTIVE: u32 = 0x2b;
        const MODEL_SLOT: u32 = 0x24;
        const STARTED: u32 = 0x2a;
        const PED_MGR: u32 = 0x78;
        const PED_MODEL: u32 = 0x2c4;
        const MODEL_ID: u32 = 0x2e;
        const PED_STATE: u32 = 0x219;
        const PED_VEHICLE: u32 = 0xb30;
        const VEHICLE_OCCUPANT: u32 = 0xf50;
        const PED_PHONE: u32 = 0x2b0;
        const CALL_PROGRESS: u32 = 0x4c;
        const TABLE: u32 = 0x1295cd8;
        const PARAM: u32 = 0x12b4138;
        const DISPATCHER: u32 = 0x167e2a0;
        const PHONE_DATA: u32 = 0xeb8a84;
        const SLOT_ID: u32 = 0x12c;
        const SLOT_RUN: u32 = 0x44;
        const FOUR: u32 = 0x40800000;
        const ONE: u32 = 0x3f800000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16s(a: u32) -> u32 {
            unsafe { ((a as *const u16).read_unaligned() as i16) as i32 as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj).wrapping_add(slot)) as usize);
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn vcall1(obj: u32, slot: u32, a: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj).wrapping_add(slot)) as usize);
                f(obj, a)
            }
        }

        let mgr = rd32(ped.wrapping_add(PED_MGR));
        let call: u32 = lf_checker_rt::callee_thiscall!(1, u32, mgr, 0x0eu32, 0u32);
        let mut saved = 0.0f32;
        if call != 0 {
            wr8(task.wrapping_add(ACTIVE), 1);
            saved = rdf(call.wrapping_add(CALL_PROGRESS));
        }
        let flag: u32;
        if rd8(task.wrapping_add(ACTIVE)) == 0 {
            let m = rd32(ped.wrapping_add(PED_MODEL));
            if m == 0 {
                flag = 0xa;
            } else if rd16s(m.wrapping_add(MODEL_ID)) == vcall0(ped, SLOT_ID) {
                flag = 0xb;
            } else {
                flag = 0xa;
            }
        } else {
            flag = 0xb;
        }
        if rd8(ped.wrapping_add(PED_STATE)) != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, ped);
        }
        let m = rd32(ped.wrapping_add(PED_MODEL));
        let mut reset = false;
        if m == 0 {
            reset = true;
        } else if rd16s(m.wrapping_add(MODEL_ID)) != vcall0(ped, SLOT_ID) {
            reset = true;
        }
        if reset {
            let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, ped.wrapping_add(PED_PHONE), ped, 0u32, 0u32);
        }
        let id = vcall0(ped, SLOT_ID);
        wr32(task.wrapping_add(MODEL_SLOT), id);
        if id == 0xffffffff {
            return 0;
        }
        if rd8(ped.wrapping_add(PED_STATE)) != 0 {
            let q = rd32(ped.wrapping_add(PED_VEHICLE));
            if q != 0 && rd32(q.wrapping_add(VEHICLE_OCCUPANT)) == ped {
                let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, q, 1u32);
            }
        }
        let param = rd32(lf_checker_rt::relocated(PARAM));
        let slot = rd32(task.wrapping_add(MODEL_SLOT));
        let ok: u32 = lf_checker_rt::callee_cdecl!(6, u32, slot, param);
        if ok & 0xff == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(7, u32, slot, param, 0x18u32);
            let g = rd32(lf_checker_rt::relocated(DISPATCHER));
            if rd8(task.wrapping_add(ACTIVE)) == 0 {
                let r: u32 = lf_checker_rt::callee_thiscall!(8, u32, g);
                if r == 0 {
                    return 0;
                }
                return lf_checker_rt::callee_thiscall!(9, u32, r, 0x12cu32);
            }
            let r: u32 = lf_checker_rt::callee_thiscall!(8, u32, g);
            if r == 0 {
                return 0;
            }
            let ans: u32 =
                lf_checker_rt::callee_cdecl!(10, u32, ped, 0x0eu32, FOUR, 0u32, saved.to_bits(), ONE);
            return lf_checker_rt::callee_thiscall!(11, u32, r, ans, ped, 0x0eu32, FOUR, 0u32, saved.to_bits());
        }
        wr8(task.wrapping_add(STARTED), 1);
        let entry = rd32(lf_checker_rt::relocated(TABLE).wrapping_add(slot.wrapping_mul(4)));
        let _: u32 = lf_checker_rt::callee_thiscall!(12, u32, entry);
        let mut frame = [0u32; 8];
        let fp = frame.as_mut_ptr() as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(13, u32, fp, 0u32, 0u32, 0u32, FOUR);
        let _: u32 = lf_checker_rt::callee_thiscall!(14, u32, fp, ped);
        let g = rd32(lf_checker_rt::relocated(DISPATCHER));
        let r: u32 = lf_checker_rt::callee_thiscall!(8, u32, g);
        let rr: u32;
        if r == 0 {
            rr = 0;
        } else {
            let ans: u32 = lf_checker_rt::callee_cdecl!(
                18, u32, ped, flag, FOUR, 0x642u32,
                lf_checker_rt::relocated(PHONE_DATA), 0u32, 0u32, ONE
            );
            rr = lf_checker_rt::callee_thiscall!(
                15, u32, r, ans, flag, FOUR, 0x642u32,
                lf_checker_rt::relocated(PHONE_DATA), 0u32, 0u32, ONE
            );
        }
        vcall1(rr, SLOT_RUN, ped);
        let _: u32 = lf_checker_rt::callee_thiscall!(16, u32, fp);
        rr
    }
});
