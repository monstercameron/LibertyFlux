// original: 0x00bf2380 refresh_indexed_pair
/// Refresh one indexed packed pair of `this`.
///
/// Like `rw_bf2340` but the refreshed entry is the 64-bit pair at
/// `this + idx*8 + 0x2f0/0x2f4` (`idx` is the low word of the last
/// argument), filled from callee 2's `edx:eax` answer. Returns the low half.
export!(thiscall, rw_bf2380(this_obj: u32, src: u32, idx_arg: u32) -> u32 {
    unsafe {
        let idx = (idx_arg & 0xffff) as u32;
        let mut tmp = [0u32; 4];
        callee_thiscall!(1, u32, src, tmp.as_mut_ptr() as u32);
        let mut out = [0u32; 4];
        let pair = callee_cdecl!(2, u64, out.as_mut_ptr() as u32);
        *((this_obj + idx * 8 + 0x2f0) as *mut u32) = pair as u32;
        *((this_obj + idx * 8 + 0x2f4) as *mut u32) = (pair >> 32) as u32;
        pair as u32
    }
});
