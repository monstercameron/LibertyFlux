// original: 0x00bf3230 fetch_forward_copy_triple_s
/// Fetch one packed value, forward a scratch buffer, copy a triple.
///
/// Same shape as `rw_bf10f0` with the packed word at `this+0x4` and the
/// triple at `this+0x8/0xc/0x10`.
export!(thiscall, rw_bf3230(this_obj: u32, dst: u32) -> u32 {
    unsafe {
        let mut decoded = [0u32; 4];
        let packed = *((this_obj + 4) as *const u32);
        callee_cdecl!(1, u32, decoded.as_mut_ptr() as u32, packed);
        let mut out = [0u32; 4];
        callee_thiscall!(2, u32, dst, out.as_mut_ptr() as u32);
        let w0 = *((this_obj + 8) as *const u32);
        let w1 = *((this_obj + 0xc) as *const u32);
        let w2 = *((this_obj + 0x10) as *const u32);
        *((dst + 0x30) as *mut u32) = w0;
        *((dst + 0x34) as *mut u32) = w1;
        *((dst + 0x38) as *mut u32) = w2;
        w2
    }
});
