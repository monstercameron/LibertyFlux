// original: 0x00698730 rage::crCreatureComponentSkeleton::vf4
/// Skeleton update: builds a two-word gate {arg0, inner} on the stack and
/// hands it with the inner object's header word, arg1 and this to the pose
/// helper; when the dirty flag at +0x10 is clear it also forwards two words
/// of the inner object to the follow-up routine. Returns the last answer.
lf_k2_rt::export!(thiscall, rw_00698730(this: *mut u8, a0: u32, a1: u32) -> u32 {
    unsafe {
        let inner = *((this.add(0x0c)) as *const u32);
        let mut gate = [a0, inner];
        let head = *((inner as *const u8).add(4) as *const u32);
        let ans1: u32 = lf_k2_rt::callee_thiscall!(
            1,
            u32,
            gate.as_mut_ptr() as u32,
            head,
            a1,
            this as u32
        );
        if *this.add(0x10) == 0 {
            let obj = *((this.add(0x0c)) as *const u32);
            let b = *((obj as *const u8).add(8) as *const u32);
            let c = *((obj as *const u8).add(0x14) as *const u32);
            lf_k2_rt::callee_thiscall!(2, u32, obj, b, c)
        } else {
            ans1
        }
    }
});
