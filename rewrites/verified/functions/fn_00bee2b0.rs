// original: 0x00BEE2B0 copy_task_visual_targets_back

/// Run the task's preparation helpers, then copy the selected target's two
/// coordinate words back to `this + 0x70/+0x74` when its signed kind matches
/// one of the three registered task kinds. The target kind is at `+0x2e`, and
/// its coordinates are at `+0x149c/+0x1498`. Otherwise EAX returns the
/// sign-extended kind. This thiscall method accepts one stack pointer.
lf_checker_rt::export!(thiscall, rw_00BEE2B0(this: u32, target: u32) -> u32 {
    const TARGET_KIND: u32 = 0x2E;
    const TARGET_X: u32 = 0x149C;
    const TARGET_Y: u32 = 0x1498;
    const CURRENT_X: u32 = 0x70;
    const CURRENT_Y: u32 = 0x74;
    const KIND_A: u32 = 0x012FA47C;
    const KIND_B: u32 = 0x012FA008;
    const KIND_C: u32 = 0x012FA248;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, 10);
        let _ = lf_checker_rt::callee_thiscall!(2, u32, this, target);
        let signed_kind = read_u16(target + TARGET_KIND) as i16 as i32;
        let matches_kind = signed_kind == lf_checker_rt::global::<i32>(KIND_A).read_unaligned()
            || signed_kind == lf_checker_rt::global::<i32>(KIND_B).read_unaligned()
            || signed_kind == lf_checker_rt::global::<i32>(KIND_C).read_unaligned();
        if !matches_kind {
            return signed_kind as u32;
        }
        let x = read_u32(target + TARGET_X);
        let y = read_u32(target + TARGET_Y);
        write_u32(this + CURRENT_X, x);
        write_u32(this + CURRENT_Y, y);
        y
    }
});
