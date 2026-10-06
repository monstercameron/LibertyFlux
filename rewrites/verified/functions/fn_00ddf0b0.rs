// original: 0x00DDF0B0 UITextField scale validator
/// UITextField helper (input-ui subsystem). `this` is the field object.
/// Validate the field scale, returning 1 when acceptable, else 0. Measure
/// the primary child twice: when the second measurement is below the first
/// (or either is NaN, which also takes the branch), accept at once. Otherwise
/// measure again plus the secondary child, widen both to doubles and accept
/// unless the primary exceeds the secondary scaled by 0.95 (NaN accepts).
/// Only al is written on return, so the upper 24 bits of the last measurement
/// are preserved in the result.
/// Original: thiscall, no stack words, result in eax.
lf_checker_rt::export!(thiscall, rw_00DDF0B0(this: u32) -> u32 {
    unsafe {
        const CHILD_A: u32 = 0x1E8;
        const CHILD_B: u32 = 0x1E0;
        const MEASURE_SLOT: u32 = 0x8C;
        const TOUCH_SLOT: u32 = 0x14C;
        const SCALE: f64 = f64::from_bits(0x3FEE666666666666); // 0.95
        unsafe fn measure(child: u32) -> f32 {
            let vt = ((child) as *const u32).read_unaligned();
            let slot = ((vt + MEASURE_SLOT) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32) -> f32 =
                unsafe { core::mem::transmute(slot as usize) };
            f(child)
        }
        let a = ((this + CHILD_A) as *const u32).read_unaligned();
        let b = ((this + CHILD_B) as *const u32).read_unaligned();
        let f1 = measure(a);
        let vt = ((a) as *const u32).read_unaligned();
        let tslot = ((vt + TOUCH_SLOT) as *const u32).read_unaligned();
        let t: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(tslot as usize) };
        t(a);
        let f2 = measure(a);
        if !(f2 >= f1) {
            return (f2.to_bits() & 0xFFFF_FF00) | 1;
        }
        let f3 = measure(a);
        let g = measure(b);
        let scaled = core::hint::black_box(g as f64) * core::hint::black_box(SCALE);
        if !((f3 as f64) > scaled) {
            (g.to_bits() & 0xFFFF_FF00) | 1
        } else {
            g.to_bits() & 0xFFFF_FF00
        }
    }
});
