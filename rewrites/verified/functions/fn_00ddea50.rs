// original: 0x00DDEA50 UITextField scale propagation
/// UITextField helper (input-ui subsystem). `this` is the field object.
/// Rescale the field to `scale` and propagate it to the selected child.
/// Store `scale` (float bits) into the field word at `+0x21C`, divide it by
/// the previous value to get the ratio, fetch the child's factor pair
/// through its virtual slot `+0x210`, and call the child's slot `+0x1DC`
/// with (pair0 * ratio, pair1 * ratio, 1). The ratio spill reuses the
/// incoming argument slot (the stack check is off for this proof; the ratio
/// is observed in the call's float arguments). Returns the last call's
/// answer. Original: thiscall, two stack words (`scale`, `mode`).
lf_checker_rt::export!(thiscall, rw_00DDEA50(this: u32, scale: u32, mode: u32) -> u32 {
    unsafe {
        const FIELD: u32 = 0x21C;
        const SELECT: u32 = 1;
        const SELECT2: u32 = 2;
        const PAIR_SLOT: u32 = 0x210;
        const APPLY_SLOT: u32 = 0x1DC;
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let old = f32::from_bits(((this + FIELD) as *const u32).read_unaligned());
        ((this + FIELD) as *mut u32).write_unaligned(scale);
        let child1 = lf_checker_rt::callee_thiscall!(SELECT, u32, this, mode);
        let vt1 = ((child1) as *const u32).read_unaligned();
        let pslot = ((vt1 + PAIR_SLOT) as *const u32).read_unaligned();
        let p: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(pslot as usize) };
        let pair = p(child1);
        let ratio = div(f32::from_bits(scale), old);
        let child2 = lf_checker_rt::callee_thiscall!(SELECT2, u32, this, mode);
        let vt2 = ((child2) as *const u32).read_unaligned();
        let aslot = ((vt2 + APPLY_SLOT) as *const u32).read_unaligned();
        let app: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(aslot as usize) };
        let f0 = f32::from_bits(((pair) as *const u32).read_unaligned());
        let f1 = f32::from_bits(((pair + 4) as *const u32).read_unaligned());
        app(child2, mul(f0, ratio).to_bits(), mul(f1, ratio).to_bits(), 1)
    }
});
