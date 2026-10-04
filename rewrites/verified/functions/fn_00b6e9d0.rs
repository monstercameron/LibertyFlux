// original: 0x00B6E9D0 task_pick_subtask_id (proposed)

// Pick this task's subtask id from a switch value and a sub-decision, report
// it, and optionally run two follow-up calls.
//
// Arguments (thiscall): `this` is the task object, `arg1` a context pointer.
// Returns: the incoming eax when the task already has an id; otherwise the
// report callee's answer, or 0 when the first follow-up declines, or the
// second follow-up's answer.
//
// Layout read: `id` = u32 at `this+0x1c` (-1 means unset); `key` = u32 at
// `this+0x28`; `cobj` = u32 at `this+0x24`. The sub-decision object (when
// non-null) is read at +0x2e (signed word table index), +0x211 (flag byte)
// and +0xa70 (must equal 1 for the flag path). The context's +0x21c points to
// an object whose +0x12c must equal 2 for the flag path. The shared row table
// at file VA 0x1295CD8 is indexed by signed word * 4; rows are read at +0x120
// (bit 1 feeds the report's boolean) and +0xc4 (compared against the report's
// answer).
//
// Behavior: if `id` is not -1, return the incoming eax unchanged (the
// contract fixes it to ENTRY_EAX, which the rewrite returns). Else call the
// sub-decision helper (thiscall, no stack args) and dispatch on
// `key - 5`: values 2-3 take the B body, 23 takes the C body, anything else
// (including out-of-range, via the unsigned comparison) takes the D body:
// B writes 0x16b/0x169, D writes 0x16a/0x168 (first/second variant according
// to whether the sub-decision is non-null with its flag byte set), C writes
// 0x16c. Each body also sets an auxiliary id (-1 initially, 0x173/0x16f in B,
// 0x171/0x16d in D) whose address is passed to the report callee. Then
// compute flag = (sub non-null, its +0xa70 is 1, context's row id is 2); the
// original also plants the flag into the low byte of its incoming argument
// slot, which has no Rust equivalent (checks.stack is off; the flag value
// itself is verified as a call argument). Call the report callee (cdecl/8)
// with (row_c4, aux-pointer, arg1, cobj, !bl, 0, flag_dword, 1) where
// flag_dword is arg1 with its low byte replaced by the flag. Store the answer
// at `this+0x20`; if it differs from the row's +0xc4, return it. Else call the
// first follow-up (thiscall/0 on arg1+0x2b0); if it returns 0, return 0. Else
// call the second follow-up (thiscall/3 on arg1+0x2b0 with (arg1, 0, 0)) and
// return its answer.
//
// The jump table behind the dispatch was verified byte for byte: index bytes
// [0,0,1,1,3...3,2] over targets [D,B,C,D].
lf_checker_rt::export!(thiscall, rw_00b6e9d0(this: u32, arg1: u32) -> u32 {
    unsafe {
        const ID_OFF: u32 = 0x1c;
        const KEY_OFF: u32 = 0x28;
        const COBJ_OFF: u32 = 0x24;
        const RESULT_OFF: u32 = 0x20;
        const UNSET: u32 = 0xFFFF_FFFF;
        const ENTRY_EAX: u32 = 0x1234_5678;
        const SUB_FLAG_OFF: u32 = 0x211;
        const SUB_WORD_OFF: u32 = 0x2e;
        const SUB_READY_OFF: u32 = 0xA70;
        const CTX_ROW_OFF: u32 = 0x21C;
        const ROW_ID_OFF: u32 = 0x12C;
        const ROW_ID_WANT: u32 = 2;
        const TABLE_VA: u32 = 0x1295CD8;
        const ROW_BITS_OFF: u32 = 0x120;
        const ROW_C4_OFF: u32 = 0xC4;
        const FOLLOW_OFF: u32 = 0x2B0;
        const SUB_CALLEE: u32 = 1;
        const REPORT_CALLEE: u32 = 2;
        const FOLLOW1_CALLEE: u32 = 3;
        const FOLLOW2_CALLEE: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16i(a: u32) -> i32 {
            unsafe { (a as *const i16).read_unaligned() as i32 }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        if rd32(this + ID_OFF) != UNSET {
            return ENTRY_EAX;
        }
        let sub: u32 = lf_checker_rt::callee_thiscall!(SUB_CALLEE, u32, this);
        let c = rd32(this + KEY_OFF).wrapping_sub(5);
        let sub_set = sub != 0 && rd8(sub + SUB_FLAG_OFF) != 0;
        let mut aux_id = UNSET;
        if c == 2 || c == 3 {
            wr32(this + ID_OFF, if sub_set { 0x16b } else { 0x169 });
            aux_id = if sub_set { 0x173 } else { 0x16f };
        } else if c == 23 {
            wr32(this + ID_OFF, 0x16c);
        } else {
            wr32(this + ID_OFF, if sub_set { 0x16a } else { 0x168 });
            aux_id = if sub_set { 0x171 } else { 0x16d };
        }

        let table = lf_checker_rt::relocated(TABLE_VA);
        let flag: u32 = if sub != 0
            && rd32(sub + SUB_READY_OFF) == 1
            && rd32(rd32(arg1 + CTX_ROW_OFF) + ROW_ID_OFF) == ROW_ID_WANT
        {
            1
        } else {
            0
        };
        let bl: bool = if sub == 0 {
            false
        } else {
            let w = rd16i(sub + SUB_WORD_OFF);
            let e = rd32((table as i32).wrapping_add(w * 4) as u32);
            rd32(e + ROW_BITS_OFF) & 2 != 0
        };
        let cobj = rd32(this + COBJ_OFF);
        let w2 = rd16i(cobj + SUB_WORD_OFF);
        let e2 = rd32((table as i32).wrapping_add(w2 * 4) as u32);
        let e2c4 = rd32(e2 + ROW_C4_OFF);
        let mut aux_slot = aux_id;
        let flag_dword = (arg1 & 0xFFFF_FF00) | flag;
        let ans: u32 = lf_checker_rt::callee_cdecl!(
            REPORT_CALLEE, u32, e2c4, &mut aux_slot as *mut u32 as u32, arg1, cobj,
            (!bl) as u32, 0, flag_dword, 1
        );
        wr32(this + RESULT_OFF, ans);
        let e3 = rd32((table as i32).wrapping_add(w2 * 4) as u32);
        if ans != rd32(e3 + ROW_C4_OFF) {
            return ans;
        }
        let t1: u32 =
            lf_checker_rt::callee_thiscall!(FOLLOW1_CALLEE, u32, arg1.wrapping_add(FOLLOW_OFF));
        if t1 == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(
            FOLLOW2_CALLEE, u32, arg1.wrapping_add(FOLLOW_OFF), arg1, 0, 0
        )
    }
});
