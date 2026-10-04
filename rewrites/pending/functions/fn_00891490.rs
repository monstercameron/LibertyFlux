// original: 0x00891490 audio_register_slot
/// Register slot `b` of this object and notify the table owner.
///
/// Unless `a` is -1, slot `b` is first marked 0xff; bit 5 is set at +0x3a.
/// The slot index is recovered arithmetically as `(this - row[variant]) /
/// stride`, then the owner helper (stubbed by the checker) is invoked with
/// the variant, recovered index, the three arguments and the +0x3c field
/// pointer. Returns the helper's answer.
export!(thiscall, rw_00891490(this: u32, a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        if a != 0xffffffff {
            ((b.wrapping_add(this).wrapping_add(0x48)) as *mut u8).write(0xff);
        }
        let p3a = (this + 0x3a) as *mut u8;
        p3a.write(p3a.read() | 0x20);
        let variant = ((this + 0x40) as *const u8).read() as u32;
        let base = *global::<u32>(0x0115D988);
        let row = ((base
            .wrapping_add(variant.wrapping_mul(0x6f40))
            .wrapping_add(0x6f10)) as *const u32)
            .read();
        let stride = *global::<u32>(0x0115D964);
        let index = this.wrapping_sub(row) / stride;
        callee_thiscall!(
            1,
            u32,
            relocated(0x0115D8A0),
            variant,
            index & 0xff,
            a,
            this + 0x3c,
            b,
            c
        )
    }
});
