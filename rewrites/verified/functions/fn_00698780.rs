// original: 0x00698780 rage::crCreatureComponentSkeleton::vf5
/// Skeleton shutdown step: unless the flag at +0x10 is set, releases the
/// inner object first, then hands a one-word gate {arg0} with the header
/// word, arg1 and the inner object to the teardown helper.
/// Returns the helper's answer.
lf_k2_rt::export!(thiscall, rw_00698780(this: *mut u8, a0: u32, a1: u32) -> u32 {
    unsafe {
        let obj = *((this.add(0x0c)) as *const u32);
        if *this.add(0x10) == 0 {
            lf_k2_rt::callee_thiscall!(1, u32, obj);
        }
        let mut gate = [a0];
        let head = *((obj as *const u8).add(4) as *const u32);
        lf_k2_rt::callee_thiscall!(
            2,
            u32,
            gate.as_mut_ptr() as u32,
            head,
            a1,
            obj
        )
    }
});
