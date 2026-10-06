// original: 0x00D40800 ped_task_gate_and_emit_float (proposed)

/// Validate a ped task through a chain of engine queries, then sample one
/// float and emit it to the ped's host object.
///
/// `obj` points to the ped. The function proceeds only if every gate passes:
/// a no-argument engine check returns a non-zero low byte, a query on the
/// ped's sub-object at `+0x224` (queried as `this + 0x44` with argument
/// `0x111`) returns a non-null info block, the block's row index selects a
/// table slot whose first object reports kind `0xD3`, the block's sub index
/// plus one is below 16 (**signed** comparison), the slot's second object is
/// non-null and reports kind `0x11D`, and that object's child at `+0x14`
/// reports kind `0x384` or `0x387`. Any failure returns the value that
/// failed the gate (or the last accepted kind).
///
/// On the main path a callee fills one frame word, a getter result is
/// converted to a float by a cdecl helper (x87 `ST0` result), the frame word
/// is handed to an apply callee, and the float plus two zero words are
/// emitted through the host object at `+0xA80`; flag bit `0x400` in the word
/// at `+0x29C` is set. The exit value is the emit callee's answer.
///
/// The row table lives at file address `0x167F780` (row stride `0x6C`, two
/// object words per sub index at `+0x18`/`+0x1C`); it is game-runtime data,
/// planted by the proof. Original: stdcall/1, returns `eax`.
lf_checker_rt::export!(stdcall, rw_00d40800(obj: u32) -> u32 {
    unsafe {
        const QUERY_ARG: u32 = 0x111;
        const OBJ_QUERY_LINK: u32 = 0x224;
        const QUERY_THIS_OFF: u32 = 0x44;
        const INFO_ROW: u32 = 0x14;
        const INFO_SUB: u32 = 0x18;
        const TABLE_BASE: u32 = 0x0167_f780;
        const TABLE_STRIDE: u32 = 0x6c;
        const SLOT_FIRST: u32 = 0x18;
        const SLOT_SECOND: u32 = 0x1c;
        const KIND_SLOT: u32 = 0x0c;
        const EXPECT_FIRST: u32 = 0xd3;
        const EXPECT_SECOND: u32 = 0x11d;
        const EXPECT_CHILD_A: u32 = 0x384;
        const EXPECT_CHILD_B: u32 = 0x387;
        const MAX_SUB_PLUS_ONE: i32 = 0x10;
        const CHILD_LINK: u32 = 0x14;
        const EMIT_HOST_OFF: u32 = 0xa80;
        const FLAGS_OFF: u32 = 0x29c;
        const EMITTED_FLAG: u32 = 0x400;
        const C_GATE: u32 = 0;
        const C_QUERY: u32 = 1;
        const C_FILL: u32 = 5;
        const C_GET: u32 = 6;
        const C_CONVERT: u32 = 7;
        const C_APPLY: u32 = 8;
        const C_EMIT: u32 = 9;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        /// The original's `(an instruction of the original); (an instruction of the original); (an instruction of the original)`
        /// (virtual slot 3, thiscall, no stack arguments).
        #[inline(always)]
        unsafe fn kind_of(o: u32) -> u32 {
            unsafe {
                let slot = rd32(rd32(o).wrapping_add(KIND_SLOT));
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(o)
            }
        }

        let gate: u32 = lf_checker_rt::callee_cdecl!(C_GATE, u32,);
        if gate & 0xFF == 0 {
            return gate;
        }
        let query_this = rd32(obj.wrapping_add(OBJ_QUERY_LINK)).wrapping_add(QUERY_THIS_OFF);
        let info: u32 = lf_checker_rt::callee_thiscall!(C_QUERY, u32, query_this, QUERY_ARG);
        if info == 0 {
            return 0;
        }
        let row = rd32(info.wrapping_add(INFO_ROW));
        let sub = rd32(info.wrapping_add(INFO_SUB));
        let slot = lf_checker_rt::relocated(TABLE_BASE)
            .wrapping_add(row.wrapping_mul(TABLE_STRIDE))
            .wrapping_add(sub.wrapping_mul(4));
        let first = rd32(slot.wrapping_add(SLOT_FIRST));
        let kind = kind_of(first);
        if kind != EXPECT_FIRST {
            return kind;
        }
        // `(an instruction of the original); (an instruction of the original); jge`: signed.
        if sub.wrapping_add(1) as i32 >= MAX_SUB_PLUS_ONE {
            return EXPECT_FIRST;
        }
        let second = rd32(slot.wrapping_add(SLOT_SECOND));
        if second == 0 {
            return EXPECT_FIRST;
        }
        let kind2 = kind_of(second);
        if kind2 != EXPECT_SECOND {
            return kind2;
        }
        let child = rd32(second.wrapping_add(CHILD_LINK));
        if child == 0 {
            return EXPECT_SECOND;
        }
        let kind3 = kind_of(child);
        if kind3 != EXPECT_CHILD_A && kind3 != EXPECT_CHILD_B {
            return kind3;
        }
        // Frame word below the incoming ESP, zero-filled like the original's
        // untouched stack slot; the fill callee writes the sample through it.
        let mut sample: u32 = 0;
        lf_checker_rt::callee_thiscall!(C_FILL, u32, second, &mut sample as *mut u32 as u32);
        let raw: u32 = lf_checker_rt::callee_thiscall!(C_GET, u32, second);
        let value: f32 = lf_checker_rt::callee_cdecl!(C_CONVERT, f32, raw);
        let _: u32 =
            lf_checker_rt::callee_thiscall!(C_APPLY, u32, obj, &mut sample as *mut u32 as u32);
        let host = rd32(obj.wrapping_add(EMIT_HOST_OFF));
        let ret: u32 =
            lf_checker_rt::callee_thiscall!(C_EMIT, u32, host, value.to_bits(), 0, 0);
        wr32(
            obj.wrapping_add(FLAGS_OFF),
            rd32(obj.wrapping_add(FLAGS_OFF)) | EMITTED_FLAG,
        );
        ret
    }
});
