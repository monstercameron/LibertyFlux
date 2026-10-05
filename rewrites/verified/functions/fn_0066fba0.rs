// original: 0X0066FBA0 rage::fiTokenizer::vf7
/// Read 4 floats from the tokenizer into an output vector.
///
/// `this` points to the tokenizer, `out` to room for 4 floats and `flag` is
/// passed unchanged to every read. Each value comes from virtual slot
/// `+0x18` (the float reader), which returns it on the floating-point stack;
/// slot answers vary per call. The original stages the values through frame temporaries (the first even in its incoming-argument slot, so this proof runs with the stack comparison off) and then copies them; only the final stores are observable. The value returned in `eax` is the output pointer (reloaded for the final copy).
///
/// Original: 0X0066FBA0 (thiscall, two stack arguments, 4 calls).
lf_checker_rt::export!(thiscall, rw_0066fba0(this: u32, out: u32, flag: u32) -> u32 {
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
        let f3 = read_float(this, flag);
        ((out + 12) as *mut u32).write_unaligned(f3.to_bits());
        out
    }
});
