// original: 0x00a49a70 vehicle_any_door_flag_clear
/// True when any door-linked object has its +0x211 byte clear, or the flag.
///
/// Returns 1 when bit 0x40 of `this+0xF1D` is set, when the object at
/// `this+0xF50` is non-null with a zero byte at `+0x211`, or when any of the
/// eight objects at `this+0xF54..` qualifies the same way; else 0 (thiscall,
/// no stack arguments). Null slots are skipped. Only AL is compared.
export!(thiscall, rw_00a49a70(this: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0xf1d;
        const FLAG_BIT: u8 = 0x40;
        const FIRST_OFF: u32 = 0xf50;
        const REST_OFF: u32 = 0xf54;
        const REST_N: u32 = 8;
        const TEST_OFF: u32 = 0x211;
        if ((this.wrapping_add(FLAG_OFF)) as *const u8).read() & FLAG_BIT != 0 {
            return 1;
        }
        let p = (this.wrapping_add(FIRST_OFF) as *const u32).read_unaligned();
        if p != 0 && ((p.wrapping_add(TEST_OFF)) as *const u8).read() == 0 {
            return 1;
        }
        let mut q = this.wrapping_add(REST_OFF);
        let mut i = 0u32;
        while i < REST_N {
            let r = (q as *const u32).read_unaligned();
            if r != 0 && ((r.wrapping_add(TEST_OFF)) as *const u8).read() == 0 {
                return 1;
            }
            q = q.wrapping_add(4);
            i += 1;
        }
        0
    }
});
