// original: 0x00addcd0 CRenderPhaseScript2d::vf8
/// Render-phase callback: notify with the slot at 0x940, then mark ready.
///
/// Calls the worker with (slot, 0), sets the ready byte at 0x20, and returns
/// the worker's answer.
export!(thiscall, rw_00addcd0(this: *mut u8) -> u32 {
    unsafe {
        let slot = *(this.add(0x940) as *const u32);
        let answer = callee_cdecl!(1, u32, slot, 0);
        *(this.add(0x20)) = 1;
        answer
    }
});
