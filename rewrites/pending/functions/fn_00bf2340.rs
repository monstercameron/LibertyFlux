// original: 0x00bf2340 refresh_indexed_slot
/// Refresh one indexed packed slot of `this`.
///
/// Runs callee 1 (`this` = `src`) and callee 2 over scratch buffers, then
/// stores callee 2's answer at `this + idx*4 + 0x1f0` (`idx` is the low byte
/// of the last argument). Returns the stored answer.
export!(thiscall, rw_bf2340(this_obj: u32, src: u32, idx_arg: u32) -> u32 {
    unsafe {
        let idx = (idx_arg & 0xff) as u32;
        let mut tmp = [0u32; 4];
        callee_thiscall!(1, u32, src, tmp.as_mut_ptr() as u32);
        let mut out = [0u32; 4];
        let packed = callee_cdecl!(2, u32, out.as_mut_ptr() as u32);
        *((this_obj + idx * 4 + 0x1f0) as *mut u32) = packed;
        packed
    }
});
