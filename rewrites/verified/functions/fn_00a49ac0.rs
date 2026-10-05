// original: 0x00a49ac0 vehicle_any_slot_state_two
/// True when any of nine linked objects has byte +0xA60 equal to 2.
///
/// The objects are the pointer at `this+0xF50` plus the eight at
/// `this+0xF54..`; null slots are skipped (thiscall, no stack arguments).
/// Only AL is compared: the upper bytes keep the loop counter bytes.
export!(thiscall, rw_00a49ac0(this: u32) -> u32 {
    unsafe {
        const FIRST_OFF: u32 = 0xf50;
        const REST_OFF: u32 = 0xf54;
        const REST_N: u32 = 8;
        const TEST_OFF: u32 = 0xa60;
        const WANT: u8 = 2;
        let p = (this.wrapping_add(FIRST_OFF) as *const u32).read_unaligned();
        if p != 0 && ((p.wrapping_add(TEST_OFF)) as *const u8).read() == WANT {
            return 1;
        }
        let mut q = this.wrapping_add(REST_OFF);
        let mut i = 0u32;
        while i < REST_N {
            let r = (q as *const u32).read_unaligned();
            if r != 0 && ((r.wrapping_add(TEST_OFF)) as *const u8).read() == WANT {
                return 1;
            }
            q = q.wrapping_add(4);
            i += 1;
        }
        0
    }
});
