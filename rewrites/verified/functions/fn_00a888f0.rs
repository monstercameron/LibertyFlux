// original: 0x00a888f0 conditional_release_chain
/// Conditionally release two fields, reset a sub-object, tail on.
///
/// When the byte at `this + 0xC0A` is set, releases the pointer at
/// `this + 0xB84` (intercepted, cdecl/1); resets the sub-object at
/// `this + 0x9F0` (intercepted, thiscall/0); when the byte at
/// `this + 0x1CA` is set, releases the pointer at `this + 0x144`;
/// then tail-calls the reset helper on `this` and returns its answer.
export!(thiscall, rw_00a888f0(this_obj: u32) -> u32 {
    unsafe {
        if *((this_obj.wrapping_add(0xc0a)) as *const u8) != 0 {
            let arg = *((this_obj.wrapping_add(0xb84)) as *const u32);
            let _: u32 = callee_cdecl!(1, u32, arg);
        }
        let _: u32 = callee_thiscall!(2, u32, this_obj.wrapping_add(0x9f0));
        if *((this_obj.wrapping_add(0x1ca)) as *const u8) != 0 {
            let arg = *((this_obj.wrapping_add(0x144)) as *const u32);
            let _: u32 = callee_cdecl!(3, u32, arg);
        }
        callee_thiscall!(4, u32, this_obj)
    }
});
