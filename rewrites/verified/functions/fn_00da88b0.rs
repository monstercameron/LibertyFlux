// original: 0x00da88b0 CTaskComplexSmartFleePoint::vf5
/// Decide whether the flee-point task accepts the proffered sub-task.
///
/// When the middle argument is null and the task is unstamped, the task is
/// stamped with the clock first. Then the subtask, when unflagged, gets the
/// final say through its own slot, and a pass marks it decided. Returns 1 on
/// accept, 0 on veto. The subtask call is indirect (vtable slot +0x14) and
/// resolves through the same planted object on both sides.
export!(thiscall, rw_00da88b0(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        /// Clock slot and the task's stamp fields.
        const CLOCK: u32 = 0x011735B4;
        const STAMP: u32 = 0x5c;
        const STAMP_ARG: u32 = 0x60;
        const STAMP_FLAG: u32 = 0x64;
        /// Subtask's slot, flag word and decided marker bit.
        const SUB: u32 = 8;
        const SUB_FLAGS: u32 = 0x0c;
        const DECIDED: u32 = 2;
        if a1 == 0 && (*((this.wrapping_add(STAMP_FLAG)) as *const u8)) != 0 {
            ((this.wrapping_add(STAMP)) as *mut u32).write_unaligned(global::<u32>(CLOCK).read());
            ((this.wrapping_add(STAMP_ARG)) as *mut u32).write_unaligned(a1);
            (*((this.wrapping_add(STAMP_FLAG)) as *mut u8) = (1));
        }
        let sub = ((this.wrapping_add(SUB)) as *const u32).read_unaligned();
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
