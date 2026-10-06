// original: 0x009bb770 T_CB_Generic_3Args<void(*)(int, rage::Vector4&, rage::Matrix34&), int, rage::Vector4, rage::Matrix34>::vf1

/// Invoke the stored three-argument callback of a generic callback holder.
///
/// `this` points to the holder: the function pointer at `+0x08`, an integer
/// at `+0x0c`, a four-float vector at `+0x10` and a twelve-float matrix at
/// `+0x20`. Calls the stored pointer as `target(code, this+0x10, this+0x20)`
/// with the cdecl convention and returns nothing (whatever the callee leaves
/// in EAX is discarded, matching the void signature).
///
/// Edge cases: none in this function; the target is always called exactly
/// once with the three words in object order.
///
/// Original: thiscall, no stack arguments, one indirect callee through
/// the heap object (id 1, cdecl, three arguments).
lf_checker_rt::export!(thiscall, rw_009bb770(this: u32) -> u32 {
    unsafe {
        const TARGET: u32 = 0x08;
        const CODE: u32 = 0x0c;
        const VECTOR: u32 = 0x10;
        const MATRIX: u32 = 0x20;
        let target = ((this + TARGET) as *const u32).read_unaligned();
        let f: extern "cdecl" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let code = ((this + CODE) as *const u32).read_unaligned();
        f(code, this + VECTOR, this + MATRIX);
    }
    0
});
