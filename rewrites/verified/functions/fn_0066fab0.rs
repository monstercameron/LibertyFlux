// original: 0X0066FAB0 rage::fiTokenizer::vf10
/// Read 3 floats from the tokenizer into an output vector.
///
/// `this` points to the tokenizer, `out` to room for 3 floats and `flag` is
/// passed unchanged to every read. Each value comes from virtual slot
/// `+0x18` (the float reader), which returns it on the floating-point stack;
/// slot answers vary per call. Each value is stored straight to the output slot. The value returned in `eax` is the bit pattern of the last float read (the stub leaves the answer word there).
///
/// Original: 0X0066FAB0 (thiscall, two stack arguments, 3 calls).
lf_checker_rt::export!(thiscall, rw_0066fab0(this: u32, out: u32, flag: u32) -> u32 {
    unsafe {
        const FLOAT_SLOT: u32 = 0x18;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let vtable = rd32(this);
        let read_float: extern "thiscall" fn(u32, u32) -> f32 =
            core::mem::transmute(rd32(vtable + FLOAT_SLOT) as usize);
        let f0 = read_float(this, flag);
        ((out + 0) as *mut u32).write_unaligned(f0.to_bits());
        let f1 = read_float(this, flag);
        ((out + 4) as *mut u32).write_unaligned(f1.to_bits());
        let f2 = read_float(this, flag);
        ((out + 8) as *mut u32).write_unaligned(f2.to_bits());
        f2.to_bits()
    }
});
