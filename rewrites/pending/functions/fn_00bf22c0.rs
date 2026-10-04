// original: 0x00bf22c0 refresh_copy_triple_into_this
/// Refresh one packed slot and copy a triple into `this`.
///
/// Runs callee 1 (`this` = `src`) and callee 2 over scratch buffers, stores
/// callee 2's answer at `this+0x20`, then copies the triple at
/// `src+0x30/0x34/0x38` to `this+0x24/0x28/0x2c`. Returns the last copied
/// word.
export!(thiscall, rw_bf22c0(this_obj: u32, src: u32) -> u32 {
    unsafe {
        let mut tmp = [0u32; 4];
        callee_thiscall!(1, u32, src, tmp.as_mut_ptr() as u32);
        let mut out = [0u32; 4];
        let packed = callee_cdecl!(2, u32, out.as_mut_ptr() as u32);
        *((this_obj + 0x20) as *mut u32) = packed;
        let w0 = *((src + 0x30) as *const u32);
        let w1 = *((src + 0x34) as *const u32);
        let w2 = *((src + 0x38) as *const u32);
        *((this_obj + 0x24) as *mut u32) = w0;
        *((this_obj + 0x28) as *mut u32) = w1;
        *((this_obj + 0x2c) as *mut u32) = w2;
        w2
    }
});
