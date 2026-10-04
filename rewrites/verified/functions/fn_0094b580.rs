// original: 0x0094b580 T_CB_Generic_2Args<void(*)(int, bool), int, bool>::vf1
/// Invoke the stored two-argument callback with the stored arguments.
///
/// Loads the callback pointer, integer and boolean from the callback object
/// and calls through the pointer (cdecl). The boolean travels zero-extended.
lf_checker_rt::export!(thiscall, rw_0094b580(this_ptr: u32) -> () {
    unsafe {
        let arg0 = *(this_ptr.wrapping_add(0xc) as *const u32);
        let arg1 = *(this_ptr.wrapping_add(0x10) as *const u8) as u32;
        let tgt = *(this_ptr.wrapping_add(8) as *const u32);
        let f: extern "cdecl" fn(u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        f(arg0, arg1);
    }
});
