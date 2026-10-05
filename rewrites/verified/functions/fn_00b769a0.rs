// original: 0x00b769a0 ped_task_request_start (proposed)

/// Start a ped task request and report how it resolved.
///
/// `this` points at the requester, `target` (word 0) at the target or null.
/// Word 2's low byte tags the request and word 3's low byte arms it: an
/// unarmed request returns 1 after tagging. An armed request first notifies
/// the target unless the requester is idle or the target busy. If the
/// requester has no active subtask and subtasks are allowed, a descriptor
/// is built and a creator callee either leaves the subtask slot empty
/// (returns 2) or installs a subtask object, which is then verified (kind
/// byte 0x3b must be 8, else returns 2), released, and — for an idle
/// requester — sampled into the handle slot. Finally the subtask is
/// attached to the target unless subtasks are forbidden and there is no
/// target (returns 2); the attach callee's answer is the return value.
/// A disallowed subtask setup returns 0.
///
/// Original: 0x00b769a0 (thiscall, four stack words).
unsafe fn run_00b769a0(this: u32, target: u32, _w1: u32, w2: u32, w3: u32, skip_tag: bool) -> u32 {
    unsafe {
        const GATE_SUBTASK: u32 = 0x0115dbfc;
        const GATE_ATTACH: u32 = 0x01046ee0;
        const CREATOR_REG: u32 = 0x012845d0;
        const ID_NOTIFY: u32 = 1;
        const ID_BUILD_DESC: u32 = 2;
        const ID_CREATE: u32 = 3;
        const ID_VERIFY: u32 = 4;
        const ID_SAMPLE: u32 = 5;
        const ID_ATTACH: u32 = 6;
        const KIND_WANT: u8 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        if !skip_tag {
            ((this.wrapping_add(0xbc9)) as *mut u8).write((w2 & 0xff) as u8);
        }
        wr32(this.wrapping_add(0x98c), target);
        if ((w3 & 0xff) as u8) == 0 {
            return 1;
        }
        let idle = ((this.wrapping_add(0xbcc)) as *const u8).read() == 0;
        if !idle && rd32(target.wrapping_add(0x98)) == 0 {
            let tok = rd32(this.wrapping_add(0xab4));
            lf_checker_rt::callee_thiscall!(ID_NOTIFY, u32, target, tok);
        }
        let sub = this.wrapping_add(0x964);
        if rd32(sub) == 0 {
            if ((lf_checker_rt::relocated(GATE_SUBTASK)) as *const u8).read() != 0 {
                return 0;
            }
            let mut desc = [0u32; 8];
            let dp = desc.as_mut_ptr() as u32;
            lf_checker_rt::callee_thiscall!(ID_BUILD_DESC, u32, dp);
            let key = rd32(rd32(this.wrapping_add(0x960)).wrapping_add(4));
            lf_checker_rt::callee_thiscall!(ID_CREATE, u32, lf_checker_rt::relocated(CREATOR_REG), key, sub, dp, 0xffffffff, 0, 0);
            let obj = rd32(sub);
            if obj == 0 {
                return 2;
            }
            if ((obj.wrapping_add(0x3b)) as *const u8).read() != KIND_WANT {
                return 2;
            }
            lf_checker_rt::callee_thiscall!(ID_VERIFY, u32, obj);
            if ((this.wrapping_add(0xbcc)) as *const u8).read() == 0 {
                let h = lf_checker_rt::callee_thiscall!(ID_SAMPLE, u32, obj, 0);
                wr32(this.wrapping_add(0x994), h);
            }
        }
        if ((lf_checker_rt::relocated(GATE_ATTACH)) as *const u8).read() == 0 || target != 0 {
            let obj = rd32(sub);
            return lf_checker_rt::callee_thiscall!(ID_ATTACH, u32, obj, target, 1);
        }
        2
    }
}

lf_checker_rt::export!(thiscall, rw_00b769a0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe { run_00b769a0(this, a0, a1, a2, a3, false) }
});
