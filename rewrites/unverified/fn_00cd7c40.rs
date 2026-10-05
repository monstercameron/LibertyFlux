// original: 0x00CD7C40 CTaskComplexFollowLeaderAnyMeans::vf20

/// Original: 0x00CD7C40 (thiscall, one stack word, callee pops 4).
export!(thiscall, rw_00cd7c40(this: u32, ped: u32) -> u32 {
    unsafe {
        const SUBTASK: u32 = 0x18;
        const CHILD: u32 = 0x08;
        const FLAGS: u32 = 0x0c;
        const MODE_REC: u32 = 0x21c;
        const MODE_WORD: u32 = 0x12c;
        const MODE_WANTED: u32 = 2;
        const ACCEPT_OBJ: u32 = 0x22c;
        const ACCEPT_SLOT: u32 = 0x18;
        const ACCEPT_FLAG: u32 = 0x29c;
        const ACCEPT_BIT: u32 = 0x4000_0000;
        const PARAM_OFF: u32 = 0x2b0;
        const REC_WORD: u32 = 0x18;
        const STATE_SLOT: u32 = 0x0c;
        const STATE_SPAWN: u32 = 0x2d4;
        const ABORT_SLOT: u32 = 0x14;
        const PED_B30: u32 = 0xb30;
        const NEG_ONE_BITS: u32 = 0xbf80_0000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }

        unsafe fn fail_path(this: u32, ped: u32) -> u32 {
            unsafe {
                if rd8(this.wrapping_add(FLAGS)) & 1 != 0 {
                    return 0;
                }
                let slot = rd32(rd32(this).wrapping_add(ABORT_SLOT));
                let hook: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                let ok = hook(this, ped, 1, 0) & 0xff;
                if ok == 0 {
                    return rd32(this.wrapping_add(CHILD));
                }
                let f = rd32(this.wrapping_add(FLAGS)) | 2;
                (this.wrapping_add(FLAGS) as *mut u32).write_unaligned(f);
                0
            }
        }

        unsafe fn dispatch(this: u32, ped: u32) -> u32 {
            unsafe {
                let child = rd32(this.wrapping_add(CHILD));
                let slot = rd32(rd32(child).wrapping_add(STATE_SLOT));
                let state: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                if state(child) != STATE_SPAWN {
                    return child;
                }
                let reg: u32 = *global::<u32>(0x167E2A0);
                let prov: u32 = callee_thiscall!(8, u32, reg);
                if prov == 0 {
                    return 0;
                }
                let b30 = rd32(ped.wrapping_add(PED_B30));
                let sub = rd32(this.wrapping_add(SUBTASK));
                callee_thiscall!(9, u32, prov, b30, sub, 7, 2, 0x28, 0, 0, 0, 0, 4, 0x14,
                    NEG_ONE_BITS, 0x1e, 0x14, 1)
            }
        }

        let ident: u32 = callee_thiscall!(1, u32, ped);
        let sub = rd32(this.wrapping_add(SUBTASK));
        if sub == 0 {
            return fail_path(this, ped);
        }
        if ident != 0 {
            let chk: u32 = callee_thiscall!(2, u32, ident.wrapping_add(8));
            if chk != sub {
                return fail_path(this, ped);
            }
        }
        let mode_a = rd32(rd32(ped.wrapping_add(MODE_REC)).wrapping_add(MODE_WORD));
        if mode_a != MODE_WANTED {
            return dispatch(this, ped);
        }
        let mode_b = rd32(rd32(sub.wrapping_add(MODE_REC)).wrapping_add(MODE_WORD));
        if mode_b != MODE_WANTED {
            return dispatch(this, ped);
        }
        let acc_obj = rd32(ped.wrapping_add(ACCEPT_OBJ));
        let slot = rd32(rd32(acc_obj).wrapping_add(ACCEPT_SLOT));
        let accept: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        if accept(acc_obj, ped, 0) & 0xff != 0 {
            let f = rd32(ped.wrapping_add(ACCEPT_FLAG)) | ACCEPT_BIT;
            (ped.wrapping_add(ACCEPT_FLAG) as *mut u32).write_unaligned(f);
        }
        let sub_p = sub.wrapping_add(PARAM_OFF);
        let r1: u32 = callee_thiscall!(4, u32, sub_p);
        if r1 == 0 {
            return dispatch(this, ped);
        }
        let r2: u32 = callee_thiscall!(4, u32, sub_p);
        if rd32(r2.wrapping_add(REC_WORD)) == 0 {
            return dispatch(this, ped);
        }
        let ped_p = ped.wrapping_add(PARAM_OFF);
        let r3: u32 = callee_thiscall!(4, u32, ped_p);
        if r3 == 0 {
            return dispatch(this, ped);
        }
        let r4: u32 = callee_thiscall!(4, u32, ped_p);
        if rd32(r4.wrapping_add(REC_WORD)) != 0 {
            return dispatch(this, ped);
        }
        let count: u32 = callee_thiscall!(5, u32, ped_p, 9);
        if count as i32 <= 0 {
            return dispatch(this, ped);
        }
        let r5: u32 = callee_thiscall!(4, u32, ped_p);
        if count == rd32(r5.wrapping_add(REC_WORD)) {
            return dispatch(this, ped);
        }
        let _: u32 = callee_thiscall!(6, u32, ped_p, 9, 0, 1, 0);
        dispatch(this, ped)
    }
});
