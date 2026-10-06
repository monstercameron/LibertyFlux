// original: 0x00DB6EB0 UIFontString::vf85

/// Resolve four layout metrics through virtual hooks, combine them into the
/// cached corners, then tail-dispatch to the render slot.
///
/// `thiscall`, no stack args. Hooks at vtable `+0xC8/+0xD0/+0xB8/+0xC0`
/// (thiscall, no stack args) each answer an f32; the mode word at `+0x1D8`
/// selects the combination with the global scale (equality tests, so
/// signedness is moot): mode 0 stores `m1 - m3*scale` at `+0x1F8`, mode 1
/// stores `m3*scale + m1`, any other mode stores `m1`; `+0x1FC` always gets
/// `m2 - m4*scale`. Float operations keep the original's operand order.
/// Finally ECX is reloaded with the object and control tail-jumps through
/// vtable `+0x74`; that answer is the return value.
lf_checker_rt::export!(thiscall, rw_00db6eb0(this_ptr: u32) -> u32 {
    const SCALE: u32 = 0x00FE8830;
    const MODE: u32 = 0x1D8;
    const OUT_A: u32 = 0x1F8;
    const OUT_B: u32 = 0x1FC;
    unsafe {
        let vtable = (this_ptr as *const u32).read_unaligned();
        let hook = |slot: u32| unsafe {
            let target = ((vtable + slot) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32) -> f32 =
                core::mem::transmute(target as usize);
            f(this_ptr)
        };
        let m1 = hook(0xC8);
        let m2 = hook(0xD0);
        let m3 = hook(0xB8);
        let m4 = hook(0xC0);
        let scale = f32::from_bits(lf_checker_rt::global::<u32>(SCALE).read());
        let mode = ((this_ptr + MODE) as *const u32).read_unaligned();
        let mul = |a: f32, b: f32| core::hint::black_box(a) * core::hint::black_box(b);
        let add = |a: f32, b: f32| core::hint::black_box(a) + core::hint::black_box(b);
        let sub = |a: f32, b: f32| core::hint::black_box(a) - core::hint::black_box(b);
        let r1 = if mode == 0 {
            sub(m1, mul(m3, scale))
        } else if mode == 1 {
            add(mul(m3, scale), m1)
        } else {
            m1
        };
        ((this_ptr + OUT_A) as *mut u32).write_unaligned(r1.to_bits());
        let r0 = sub(m2, mul(m4, scale));
        ((this_ptr + OUT_B) as *mut u32).write_unaligned(r0.to_bits());
        let target = ((vtable + 0x74) as *const u32).read_unaligned();
        let tail: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        tail(this_ptr)
    }
});
