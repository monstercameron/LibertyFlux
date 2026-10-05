// original: 0x00AC5F30 CCustomShaderEffectPedBoneDamageFX::vf5 (symbols)

/// Clone the bone-damage effect into a freshly allocated object.
///
/// The original allocates a new object, constructs it through the construct
/// callee, zeroes its parameter rows, copies the four colour words, flag
/// bits 0-1, mode byte and six handle words from `this`, and returns the new
/// object (thiscall, one unread stack word). A null allocation faults on
/// both sides identically while zeroing through the null pointer.
lf_checker_rt::export!(thiscall, rw_00AC5F30(this: u32, _flag: u32) -> u32 {
    unsafe {
        const ALLOC: u32 = 1;
        const CONSTRUCT: u32 = 2;
        const SIZE: u32 = 0x370;
        const ROWS: u32 = 0x2C0;
        const ROW_COUNT: u32 = 11;
        const ROW_STRIDE: u32 = 0x2C;
        const COLOR: u32 = 0x350;
        let p = lf_checker_rt::callee_cdecl!(ALLOC, u32, SIZE);
        let fresh = if p == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(CONSTRUCT, u32, p)
        };
        let mut a = fresh.wrapping_add(ROWS);
        for _ in 0..ROW_COUNT {
            (a.wrapping_add(0x58) as *mut u32).write_unaligned(0);
            (a as *mut u32).write_unaligned(0);
            (a.wrapping_add(ROW_STRIDE) as *mut u32).write_unaligned(0);
            a = a.wrapping_add(4);
        }
        for i in 0..4u32 {
            let v = (this.wrapping_add(COLOR + i * 4) as *const u32).read_unaligned();
            (fresh.wrapping_add(COLOR + i * 4) as *mut u32).write_unaligned(v);
        }
        let de = (fresh.wrapping_add(0x360) as *const u8).read();
        let se = (this.wrapping_add(0x360) as *const u8).read();
        (fresh.wrapping_add(0x360) as *mut u8).write(de ^ ((de ^ se) & 1));
        let de2 = (fresh.wrapping_add(0x360) as *const u8).read();
        (fresh.wrapping_add(0x360) as *mut u8).write(((se ^ de2) & 2) ^ de2);
        let mb = (this.wrapping_add(0x361) as *const u8).read();
        (fresh.wrapping_add(0x361) as *mut u8).write(mb);
        for off in [0x18u32, 0x10, 0x14, 0x08, 0x0c, 0x1c] {
            let v = (this.wrapping_add(off) as *const u32).read_unaligned();
            (fresh.wrapping_add(off) as *mut u32).write_unaligned(v);
        }
        fresh
    }
});
