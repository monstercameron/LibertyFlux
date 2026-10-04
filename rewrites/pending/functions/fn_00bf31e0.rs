// original: 0x00bf31e0 refresh_indexed_slot_t2
/// Refresh one indexed packed slot of `this` (second table).
///
/// Same shape as `rw_bf2340` but the refreshed slot is at
/// `this + idx*4 + 0x370`.
export!(thiscall, rw_bf31e0(this_obj: u32, src: u32, idx_arg: u32) -> u32 {
    unsafe {
        let idx = (idx_arg & 0xff) as u32;
        let mut tmp = [0u32; 4];
        callee_thiscall!(1, u32, src, tmp.as_mut_ptr() as u32);
        let mut out = [0u32; 4];
        let packed = callee_cdecl!(2, u32, out.as_mut_ptr() as u32);
        *((this_obj + idx * 4 + 0x370) as *mut u32) = packed;
        packed
    }
});
