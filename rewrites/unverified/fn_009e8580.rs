// original: 0x009e8580 ped_model_flag_action
/// Runs the flag byte of the argument through the model check and
/// then the model action for this object's table entry (shared model
/// table by the tag at `+0x2E`), returning the action's answer.
/// (thiscall, 1 arg; only the low byte of the argument is read.)
lf_checker_rt::export!(thiscall, rw_009e8580(this_ptr: u32, flag_arg: u32) -> u32 {
    unsafe {
        const TAG_OFF: u32 = 0x2E;
        const MODEL_TABLE: u32 = 0x1295CD8;
        const ENTRY_ARG_OFF: u32 = 0x3C;
        const ACTION_THIS: u32 = 0x150E0F4;
        let tag = (this_ptr.wrapping_add(TAG_OFF) as *const i16).read_unaligned() as i32;
        let entry = lf_checker_rt::global::<u32>(MODEL_TABLE)
            .wrapping_offset(tag as isize)
            .read_unaligned();
        let flag = flag_arg & 0xFF;
        let tmp: u32 = lf_checker_rt::callee_cdecl!(1, u32, this_ptr, flag);
        let lo = tmp & 0xFF;
        let slot = (entry.wrapping_add(ENTRY_ARG_OFF) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(2, u32, ACTION_THIS, slot, flag, lo)
    }
});
