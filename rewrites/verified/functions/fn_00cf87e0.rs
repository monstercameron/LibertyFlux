// original: 0x00cf87e0 CTaskComplexClimbLadder::vf18

/// Route a climb-ladder task by its subtask's current id (thiscall).
///
/// `this` is the climb-ladder task, `ped` the ped (one stack word).
/// When bit 1 of `+STATE_FLAGS` is set, the fallback callee runs with
/// (`this`, `ped`) and 0 is returned. Otherwise the subtask at `+SUBTASK`
/// is asked for its id through virtual slot `+0xc` (thiscall, no stack
/// words): a first answer of 0x11d with a null nested task at `sub+0x14`
/// forces id 0xc8, with a non-null nested task the slot is asked again
/// on the nested task (the reload of the object pointer is skipped), and
/// any other first answer is discarded for a second asking on the subtask.
///
/// The id then selects the dispatch through the task-dispatch callee
/// (thiscall with (`this`, task, `ped`)): above 0x386, 0x387 and 0x3ae map
/// to task 0x3a6, 0x3a6 maps to 0x386, anything else falls back (call the
/// fallback callee, return 0); 0x386 runs the prepare callee with (`this`,
/// `ped`) then dispatches 0x120 when `+TASK_STATE` is 1, else 0x191 (a
/// `(an instruction of the original)` on that path computes a value nothing reads); 0xcb
/// dispatches 0x516; 0x120 reads `sub+0x1c` minus one through a 4-entry
/// table where only entry 2 runs the pitch callee (thiscall with (`ped`,
/// `[this+PITCH] + PI` as bits)) before dispatching 0x516 like every
/// other entry; 0x191 notifies (thiscall with (`[ped+PED_ANIM]`, 0))
/// unless `+LADDER_BYTE` is 0, then dispatches 0x120 for task states 1
/// and 4 and 0x516 otherwise (a 5-entry table, verified); anything else
/// falls back. Returns the dispatch answer, or 0 on fallback paths.
lf_checker_rt::export!(thiscall, rw_00cf87e0(this: u32, ped: u32) -> u32 {
    unsafe {
        const SUBTASK: u32 = 8;
        const STATE_FLAGS: u32 = 0x0c;
        const TASK_STATE: u32 = 0x14;
        const PITCH: u32 = 0x70;
        const LADDER_BYTE: u32 = 0x78;
        const PED_ANIM: u32 = 0xa80;
        const VTABLE_SLOT: u32 = 0x0c;
        const SUB_NESTED: u32 = 0x14;
        const SUB_SELECT: u32 = 0x1c;
        const PI_BITS: u32 = 0x4049_0fdb;
        const GET_ID: u32 = 1;
        const FALLBACK: u32 = 2;
        const NOTIFY: u32 = 3;
        const DISPATCH: u32 = 4;
        const PREPARE: u32 = 5;
        const SET_PITCH: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        if (rd32(this + STATE_FLAGS) >> 1) & 1 == 1 {
            lf_checker_rt::callee_thiscall!(FALLBACK, u32, this, ped);
            return 0;
        }
        let sub = rd32(this + SUBTASK);
        let vtab = rd32(sub);
        let get_id: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtab + VTABLE_SLOT) as usize);
        let first = get_id(sub);
        let id = if first == 0x11d {
            let inner = rd32(sub + SUB_NESTED);
            if inner == 0 {
                0xc8
            } else {
                let get_inner: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(inner) + VTABLE_SLOT) as usize);
                get_inner(inner)
            }
        } else {
            get_id(sub)
        };
        if id > 0x386 {
            let t = id - 0x387;
            if t == 0 {
                return lf_checker_rt::callee_thiscall!(DISPATCH, u32, this, 0x3a6, ped);
            }
            let t = t.wrapping_sub(0x1f);
            if t == 0 {
                return lf_checker_rt::callee_thiscall!(DISPATCH, u32, this, 0x386, ped);
            }
            if t.wrapping_sub(8) == 0 {
                return lf_checker_rt::callee_thiscall!(DISPATCH, u32, this, 0x3a6, ped);
            }
            lf_checker_rt::callee_thiscall!(FALLBACK, u32, this, ped);
            return 0;
        }
        if id == 0x386 {
            lf_checker_rt::callee_thiscall!(PREPARE, u32, this, ped);
            let task = if rd32(this + TASK_STATE) == 1 { 0x120 } else { 0x191 };
            return lf_checker_rt::callee_thiscall!(DISPATCH, u32, this, task, ped);
        }
        let t = id.wrapping_sub(0xcb);
        if t == 0 {
            return lf_checker_rt::callee_thiscall!(DISPATCH, u32, this, 0x516, ped);
        }
        let t = t.wrapping_sub(0x55);
        if t == 0 {
            if rd32(sub + SUB_SELECT).wrapping_sub(1) == 1 {
                let pitched = add(rdf(this + PITCH), f32::from_bits(PI_BITS));
                lf_checker_rt::callee_thiscall!(SET_PITCH, u32, ped, pitched.to_bits());
            }
            return lf_checker_rt::callee_thiscall!(DISPATCH, u32, this, 0x516, ped);
        }
        if t.wrapping_sub(0x71) != 0 {
            lf_checker_rt::callee_thiscall!(FALLBACK, u32, this, ped);
            return 0;
        }
        if ((this + LADDER_BYTE) as *const u8).read() != 0 {
            let anim = rd32(ped + PED_ANIM);
            lf_checker_rt::callee_thiscall!(NOTIFY, u32, anim, 0);
        }
        let task = match rd32(this + TASK_STATE) {
            1 | 4 => 0x120,
            _ => 0x516,
        };
        lf_checker_rt::callee_thiscall!(DISPATCH, u32, this, task, ped)
    }
});
