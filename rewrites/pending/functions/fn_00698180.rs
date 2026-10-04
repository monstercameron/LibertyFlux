// original: 0x00698180 rage::crCreatureComponentMover::vf4
/// Mover update gate: asks a helper whether this mover (tag word at +0x50)
/// needs work this tick, and if so forwards a scratch block plus the mover's
/// inner object (+0x10) to the worker routine on the object at +0x0c.
/// Returns the helper's answer when it says no-work, else the worker's answer.
lf_k2_rt::export!(thiscall, rw_00698180(this: *mut u8, a0: u32, _a1: u32) -> u32 {
    unsafe {
        let tag = *((this.add(0x50)) as *const u16) as u32;
        let mut scratch = [0u32; 19];
        let probe: u32 = lf_k2_rt::callee_thiscall!(1, u32, a0, tag, scratch.as_mut_ptr() as u32);
        if (probe & 0xFF) == 0 {
            return probe;
        }
        let target = *((this.add(0x0c)) as *const u32);
        lf_k2_rt::callee_thiscall!(
            2,
            u32,
            target,
            (scratch.as_mut_ptr() as u32).wrapping_add(4),
            (this as u32).wrapping_add(0x10)
        )
    }
});
