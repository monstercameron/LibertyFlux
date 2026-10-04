// original: 0x00bf35b0 fetch_forward_copy_triple_b
/// Fetch one packed value, forward a scratch buffer, copy a triple.
///
/// Same shape as `rw_bf10f0` with the packed word at `this+0x1c` and the
/// triple at `this+0x20/0x24/0x28`.
export!(thiscall, rw_bf35b0(this_obj: u32, dst: u32) -> u32 {
    unsafe {
        let mut decoded = [0u32; 4];
        let packed = *((this_obj + 0x1c) as *const u32);
        callee_cdecl!(1, u32, decoded.as_mut_ptr() as u32, packed);
        let mut out = [0u32; 4];
        callee_thiscall!(2, u32, dst, out.as_mut_ptr() as u32);
        let w0 = *((this_obj + 0x20) as *const u32);
        let w1 = *((this_obj + 0x24) as *const u32);
        let w2 = *((this_obj + 0x28) as *const u32);
        *((dst + 0x30) as *mut u32) = w0;
        *((dst + 0x34) as *mut u32) = w1;
        *((dst + 0x38) as *mut u32) = w2;
        w2
    }
});
