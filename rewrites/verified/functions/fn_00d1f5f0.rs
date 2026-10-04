// original: 0x00d1f5f0 CTaskComplexSeekCoverShooting::vf21
// Allocates a node from the task pool and constructs it with the firing
// solution: the integer argument converted to float and scaled by a global
// factor, plus this object's position and word field. Returns the
// constructor's answer (the node), or null when allocation fails.
export!(thiscall, rw_00d1f5f0(this_ptr: u32, arg: u32) -> u32 {
    unsafe {
        let mgr = *(global::<u32>(0x0167E2A0));
        let node = callee_thiscall!(1, u32, mgr);
        if node == 0 {
            return 0;
        }
        let factor = f32::from_bits(*(global::<u32>(0x00FE86B4)));
        let scaled = (arg as i32 as f32) * factor;
        let w = *((this_ptr.wrapping_add(0x30)) as *const u32);
        callee_thiscall!(
            2, u32, node, 0xF, w,
            this_ptr.wrapping_add(0x20), scaled.to_bits(), 0xFFFF_FFFF
        )
    }
});
