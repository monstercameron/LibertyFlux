// original: 0x00c3e0c0 train_init_transform_params (proposed)
/// Initialise this car's transform block and tuning parameters.
///
/// `this` (ECX) is the car. Writes a 3x3 identity matrix at `+0x00..0x28`
/// (1.0 on the diagonal, 0.0 elsewhere), zeros at `+0x30..0x38` and
/// `+0x40..0x48`, the parameters 45.0, step, 800.0, 0.1, 150.0 at
/// `+0x50..0x60` (step is the global 0.05), zeros at `+0x64..0x6c` and
/// the byte at `+0x70`. Straight-line stores, no branches. Returns this.
///
/// Original: 0x00c3e0c0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00c3e0c0(this: u32) -> u32 {
    unsafe {
        const UNIT: u32 = 0x3f80_0000; // 1.0
        const P50: u32 = 0x4234_0000; // 45.0
        const P58: u32 = 0x4448_0000; // 800.0
        const P5C: u32 = 0x3dcc_cccd; // 0.1
        const P60: u32 = 0x4316_0000; // 150.0
        const STEP: u32 = 0x1048ab4;
        #[inline(always)]
        unsafe fn w(p: u32, v: u32) {
            unsafe { (p as *mut u32).write_unaligned(v) }
        }
        w(this, UNIT);
        w(this + 4, 0);
        w(this + 8, 0);
        w(this + 0x10, 0);
        w(this + 0x14, UNIT);
        w(this + 0x18, 0);
        w(this + 0x20, 0);
        w(this + 0x24, 0);
        w(this + 0x28, UNIT);
        w(this + 0x38, 0);
        w(this + 0x34, 0);
        w(this + 0x30, 0);
        w(this + 0x50, P50);
        w(this + 0x54, lf_checker_rt::global::<u32>(STEP).read_unaligned());
        w(this + 0x58, P58);
        w(this + 0x5c, P5C);
        w(this + 0x60, P60);
        w(this + 0x64, 0);
        ((this + 0x70) as *mut u8).write(0);
        w(this + 0x6c, 0);
        w(this + 0x68, 0);
        w(this + 0x48, 0);
        w(this + 0x44, 0);
        w(this + 0x40, 0);
        this
    }
});
