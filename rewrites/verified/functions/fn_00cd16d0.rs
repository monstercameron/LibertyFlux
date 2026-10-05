// original: 0x00cd16d0 melee_subtask_check
/// Check the melee subtask's kind and run the follow-up through a tail call.
///
/// Returns 0 when the subtask at `this+8` is null or its virtual slot at
/// `+0xc` answers anything but 0x1b1. Otherwise the subtask, two zeros
/// and two relocated constant addresses go to the setup helper (cdecl,
/// five arguments), and the follow-up runs as a tail call on the setup's
/// answer, whose result is returned. Thiscall, no stack arguments.
export!(thiscall, rw_00cd16d0(this: u32) -> u32 {
    unsafe {
        const SUB_OFF: u32 = 0x08;
        const VT_SLOT: u32 = 0x0c;
        const WANT: u32 = 0x1b1;
        const ADR_A: u32 = 0x01112778;
        const ADR_B: u32 = 0x010519c0;
        let sub = (this.wrapping_add(SUB_OFF) as *const u32).read_unaligned();
        if sub == 0 {
            return 0;
        }
        let vtab = (sub as *const u32).read_unaligned();
        let slot = (vtab.wrapping_add(VT_SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        if f(sub) != WANT {
            return 0;
        }
        let prep: u32 = callee_cdecl!(2, u32, sub, 0u32, relocated(ADR_A), relocated(ADR_B), 0u32);
        callee_thiscall!(3, u32, prep)
    }
});
