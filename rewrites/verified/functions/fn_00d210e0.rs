// original: 0x00d210e0 tick_slot_classifier
// Adds a global tick to the second argument, divides by a mode-dependent
// period and classifies the remainder into bands 0, 1 or 2. The low byte of
// the first argument selects the mode; the division never overflows.
export!(cdecl, rw_00d210e0(flag: u32, addend: u32) -> u32 {
    unsafe {
        const TICK_G: u32 = 0x01173608;
        let n = (*(global::<u32>(TICK_G))).wrapping_add(addend);
        if (flag & 0xFF) == 0 {
            let r = n % 0x61A8;
            if r < 0x2710 {
                return 0;
            }
            if r < 0x30D4 {
                return 1;
            }
            return 2;
        }
        let r = n % 0x9C40;
        if r < 0x61A8 {
            return 0;
        }
        if r < 0x6B6C {
            return 1;
        }
        2
    }
});
