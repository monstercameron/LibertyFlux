// original: 0x00DDE130 UITextField pair scaler
/// UITextField helper (input-ui subsystem). `this` is the field object.
/// Scale two values into `out`: copy the pair the primary child's virtual
/// slot `+0x210` points at, then divide each by the product of a fresh field
/// measurement (virtual slot `+0xB8` on `this`, x87 float) and the active
/// size (one of two shared words, picked by a flag call; converted SIGNED).
/// Only the two heap words are written; the float spill reuses the incoming
/// argument slot (the stack check is off for this proof; the values are
/// observed in the heap writes). Returns `out`.
/// Original: thiscall, one stack word (out pointer), result in eax.
lf_checker_rt::export!(thiscall, rw_00DDE130(this: u32, out: u32) -> u32 {
    unsafe {
        const CHILD: u32 = 0x1E8;
        const PAIR_SLOT: u32 = 0x210;
        const MEASURE_SLOT: u32 = 0xB8;
        const FLAG_CALLEE: u32 = 2;
        const SIZE_A: u32 = 0x0105C884;
        const SIZE_B: u32 = 0x0105C888;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        unsafe fn measure(this: u32) -> f32 {
            let vt = ((this) as *const u32).read_unaligned();
            let slot = ((vt + MEASURE_SLOT) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32) -> f32 =
                unsafe { core::mem::transmute(slot as usize) };
            f(this)
        }
        let child = ((this + CHILD) as *const u32).read_unaligned();
        let vt = ((child) as *const u32).read_unaligned();
        let pslot = ((vt + PAIR_SLOT) as *const u32).read_unaligned();
        let p: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(pslot as usize) };
        let pair = p(child);
        let v0 = ((pair) as *const u32).read_unaligned();
        let v1 = ((pair + 4) as *const u32).read_unaligned();
        ((out) as *mut u32).write_unaligned(v0);
        ((out + 4) as *mut u32).write_unaligned(v1);
        for i in 0..2u32 {
            let flag = lf_checker_rt::callee_cdecl!(FLAG_CALLEE, u32,);
            let size_ptr = if (flag as u8) != 0 { SIZE_B } else { SIZE_A };
            let size = ((lf_checker_rt::global::<u32>(size_ptr)) as *const u32).read_unaligned();
            let m = measure(this);
            let denom = mul(m, (size as i32) as f32);
            let cur = f32::from_bits(((out + 4 * i) as *const u32).read_unaligned());
            ((out + 4 * i) as *mut u32).write_unaligned(div(cur, denom).to_bits());
        }
        out
    }
});
