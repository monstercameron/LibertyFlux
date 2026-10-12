// original: 0x00C7A0D0 module_record_check_items

/// Run the record checker over module row a0's items. Returns (entry EAX high bytes) | 1 at once when the row starts at or past its limit (unsigned); the proof pins entry EAX to zero so this reads 1. Otherwise calls the row's checker on each item from the start, stepping by the row stride while below the limit (unsigned), returning the checker's full answer word on the first zero low byte or after the last item. Original: thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_c7a0d0(this: u32, row: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
        #[inline(always)]
        unsafe fn wr32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }
        #[inline(always)]
        unsafe fn rd16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
        #[inline(always)]
        unsafe fn wr16(address: u32, value: u16) { unsafe { (address as *mut u16).write_unaligned(value) } }
        #[inline(always)]
        unsafe fn rd8(address: u32) -> u8 { unsafe { (address as *const u8).read() } }
        #[inline(always)]
        unsafe fn wr8(address: u32, value: u8) { unsafe { (address as *mut u8).write(value) } }

        const ROW: u32 = 60;
                let base = this.wrapping_add(row.wrapping_mul(ROW));
                let check: extern "cdecl" fn(u32) -> u32 = unsafe { core::mem::transmute(rd32(base)) };
                let mut item = rd32(base.wrapping_add(0x2c));
                let limit = rd32(base.wrapping_add(0x34));
                if item >= limit {
                    return 1;
                }
                let step = rd32(base.wrapping_add(0x38));
                loop {
                    let ans = check(item);
                    if ans & 0xff == 0 {
                        return ans;
                    }
                    item = item.wrapping_add(step);
                    if item >= limit {
                        return ans;
                    }
                }
        }
});
