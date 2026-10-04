// original: 0x009e7320 ped_pose16_or_copy16
/// Poses sixteen bytes to the out-pointer: resolves the live
/// source as in `ped_frame_or_slot`, looks up and builds the pose
/// through the two direct callees, and reports bit 0 of `+0x29F`.
/// With no live source the pose is copied from `[this+0x20]+0x30` and
/// the last word's upper bytes are reported (low byte 0). (thiscall,
/// 2 args.)
lf_checker_rt::export!(thiscall, rw_009e7320(this_ptr: u32, out_ptr: u32, tag_arg: u32) -> u32 {
    unsafe {
        const SLOT_A: u32 = 0xA0;
        const SLOT_B: u32 = 0xE0;
        const FALLBACK_OFF: u32 = 0x100;
        const COPY_LINK_OFF: u32 = 0x20;
        const POSE_OFF: u32 = 0x30;
        const FLAG_OFF: u32 = 0x29F;
        let vt = (this_ptr as *const u32).read_unaligned();
        let slot_a = (vt.wrapping_add(SLOT_A) as *const u32).read_unaligned();
        let fa: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(slot_a as usize) };
        let src = if fa(this_ptr) == 0 {
            (this_ptr.wrapping_add(FALLBACK_OFF) as *const u32).read_unaligned()
        } else {
            let a = fa(this_ptr);
            let vt2 = (a as *const u32).read_unaligned();
            let slot_b = (vt2.wrapping_add(SLOT_B) as *const u32).read_unaligned();
            let fb: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(slot_b as usize) };
            fb(a)
        };
        if src == 0 {
            let base = (this_ptr.wrapping_add(COPY_LINK_OFF) as *const u32).read_unaligned();
            let psrc = base.wrapping_add(POSE_OFF);
            for i in 0..4u32 {
                let w = (psrc.wrapping_add(i * 4) as *const u32).read_unaligned();
                (out_ptr.wrapping_add(i * 4) as *mut u32).write_unaligned(w);
            }
            let last = (psrc.wrapping_add(12) as *const u32).read_unaligned();
            last & 0xFFFFFF00
        } else {
            let key = (src.wrapping_add(4) as *const u32).read_unaligned();
            let built: u32 = lf_checker_rt::callee_cdecl!(3, u32, key, tag_arg);
            let pose: u32 = lf_checker_rt::callee_thiscall!(4, u32, this_ptr, built);
            let psrc = pose.wrapping_add(POSE_OFF);
            for i in 0..4u32 {
                let w = (psrc.wrapping_add(i * 4) as *const u32).read_unaligned();
                (out_ptr.wrapping_add(i * 4) as *mut u32).write_unaligned(w);
            }
            (this_ptr.wrapping_add(FLAG_OFF) as *const u8).read() as u32 & 1
        }
    }
});
