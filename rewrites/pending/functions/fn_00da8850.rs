// original: 0x00da8850 CTaskComplexFleeAndDive::vf5
/// Decide whether the dive task accepts the proffered sub-task.
///
/// A null candidate skips straight to the subtask check. Otherwise the
/// candidate's kind is queried twice and kinds 0x87/0x88 veto it. Then the
/// current subtask, when present and unflagged, gets the final say through
/// its own slot, and a pass marks it decided. Returns 1 on accept, 0 on veto.
/// The two kind queries and the subtask call are indirect (vtable slots +4
/// and +0x14); both sides resolve them through the same planted objects.
export!(thiscall, rw_00da8850(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        /// Current subtask's slot and its flag word.
        const SUB: u32 = 8;
        const SUB_FLAGS: u32 = 0x0c;
        /// Vetoing candidate kinds and the decided marker bit.
        const VETO_A: u32 = 0x87;
        const VETO_B: u32 = 0x88;
        const DECIDED: u32 = 2;
        if a2 != 0 {
            let slot = ((((a2) as *const u32).read_unaligned().wrapping_add(4)) as *const u32).read_unaligned();
            let kind: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(slot as usize);
            if kind(a2) == VETO_A {
                return 0;
            }
            if kind(a2) == VETO_B {
                return 0;
            }
        }
        let sub = ((this.wrapping_add(SUB)) as *const u32).read_unaligned();
        if sub == 0 {
            return 1;
        }
        if (*((sub.wrapping_add(SUB_FLAGS)) as *const u8)) & 1 != 0 {
            return 1;
        }
        let slot = ((((sub) as *const u32).read_unaligned().wrapping_add(0x14)) as *const u32).read_unaligned();
        let decide: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        if decide(sub, a0, a1, a2) as u8 == 0 {
            return 0;
        }
        ((sub.wrapping_add(SUB_FLAGS)) as *mut u32).write_unaligned(((sub.wrapping_add(SUB_FLAGS)) as *const u32).read_unaligned() | DECIDED);
        1
    }
});
