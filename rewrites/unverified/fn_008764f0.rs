// original: 0x008764F0 crmt_blend_request_init

/// Initialise a blend request: set up the embedded member at +4 (vtables 0xfe7fc8/0xfe7fb4, cleared words) and run the data-table setup hook from slot 0xfe7fb8 on it; install vtable 0xfe81f4, store `a0`/`a1` at +0x14/+0x18, clamp `f2` into [0, 1.0] (the float constant from 0xfe88e8, Verified 0x3f800000) at +0x1c with NaN passing through, store `f3` at +0x20 and set the byte flag at +0x24 when `f3` is nonzero (NaN counts as nonzero). Returns `this`.
///
/// Original: 0x008764F0 (thiscall, four stack words (two integers, two float bit patterns)).
lf_checker_rt::export!(thiscall, rw_008764f0(this: u32, a0: u32, a1: u32, f2: u32, f3: u32) -> u32 {
    const SETUP_TABLE: u32 = 0x00fe7fb8;
    const CLAMP_HI: u32 = 0x00fe88e8;
    unsafe {
        let sub = this + 4;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(0x00fe7fc8));
        ((sub + 4) as *mut u32).write_unaligned(0);
        (sub as *mut u32).write_unaligned(lf_checker_rt::relocated(0x00fe7fb4));
        ((sub + 8) as *mut u32).write_unaligned(0);
        ((sub + 0xc) as *mut u32).write_unaligned(0);
        let tgt = lf_checker_rt::global::<u32>(SETUP_TABLE).read_unaligned();
        let setup: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(tgt as usize);
        let _: u32 = setup(sub);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(0x00fe81f4));
        ((this + 0x1c) as *mut u32).write_unaligned(0);
        ((this + 0x20) as *mut u32).write_unaligned(0);
        ((this + 0x24) as *mut u8).write(0);
        ((this + 0x28) as *mut u32).write_unaligned(0);
        ((this + 0x2c) as *mut u32).write_unaligned(0);
        ((this + 0x14) as *mut u32).write_unaligned(a0);
        ((this + 0x18) as *mut u32).write_unaligned(a1);
        let f2v = f32::from_bits(f2);
        let khi = f32::from_bits(lf_checker_rt::global::<u32>(CLAMP_HI).read_unaligned());
        let clamped = if f2v < 0.0 { 0.0 } else if f2v > khi { khi } else { f2v };
        ((this + 0x1c) as *mut u32).write_unaligned(clamped.to_bits());
        ((this + 0x20) as *mut u32).write_unaligned(f3);
        if f32::from_bits(f3) != 0.0 {
            ((this + 0x24) as *mut u8).write(1);
        }
        this
    }
});
