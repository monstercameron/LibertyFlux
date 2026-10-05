// original: 0x00a49280 vehicle_window_state_check
/// True when either of two byte-pair windows differs by more than 0x7F.
///
/// A null object returns 0. Otherwise a probe callee (cdecl/0, called up to
/// twice) sets `active` when the mode word is 2 and its second answer's
/// dword at +0xEE4 is positive. The first window (bytes at +0x295E/+0x295C)
/// returning above 0x7F gives 1; a set override byte or `active` gives 0;
/// else the second window (+0x296E/+0x296C) decides (stdcall, one stack
/// argument). Only AL is compared.
export!(stdcall, rw_00a49280(obj: u32) -> u32 {
    unsafe {
        if obj == 0 {
            return 0;
        }
        let mut active = 0u32;
        if (relocated(0x011d6fd4) as *const u32).read_unaligned() == 2 {
            let probe: u32 = callee_cdecl!(1, u32,);
            if probe != 0 {
                let probe2: u32 = callee_cdecl!(1, u32,);
                if (probe2.wrapping_add(0xee4) as *const i32).read_unaligned() > 0 {
                    active = 1;
                }
            }
        }
        let first = ((obj.wrapping_add(0x295e)) as *const u8).read()
            ^ ((obj.wrapping_add(0x295c)) as *const u8).read();
        if first > 0x7f {
            return 1;
        }
        if (relocated(0x018b6ed7) as *const u8).read() != 0 {
            return 0;
        }
        if active != 0 {
            return 0;
        }
        let second = ((obj.wrapping_add(0x296e)) as *const u8).read()
            ^ ((obj.wrapping_add(0x296c)) as *const u8).read();
        if second > 0x7f {
            return 1;
        }
        0
    }
});
