// original: 0x00c07550 stream_group_release
/// Release every slot array in a `count`-element group, then the group.
///
/// For each of the `count` records at `group` (stride 0x28), reads the slot
/// count at +0x26; a non-zero count refreshes each slot of the array at +0x20
/// through the slot helper (thiscall/0) and releases the array (cdecl/1).
/// Afterwards releases `group` itself (cdecl/1). A non-positive `count` skips
/// the loop but still releases the group. Stdcall: two stack words, callee
/// cleans 8.
lf_checker_rt::export!(stdcall, rw_00c07550(group: u32, count: u32) -> u32 {
    unsafe {
#[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        const SLOT_FN: u32 = 1;
        const RELEASE: u32 = 2;
        const STRIDE: u32 = 0x28;
        const ARR_OFF: u32 = 0x20;
        const COUNT_OFF: u32 = 0x26;
        const SLOT_STRIDE: u32 = 80;
        if (count as i32) > 0 {
            let mut rec = group;
            let mut left = count as i32;
            while left != 0 {
                let n = rd16(rec + COUNT_OFF);
                if n != 0 {
                    let arr = rd32(rec + ARR_OFF);
                    let mut slot = arr;
                    let mut k = n;
                    while k != 0 {
                        let _: u32 = lf_checker_rt::callee_thiscall!(SLOT_FN, u32, slot);
                        slot = slot.wrapping_add(SLOT_STRIDE);
                        k -= 1;
                    }
                    let _: u32 = lf_checker_rt::callee_cdecl!(RELEASE, u32, arr);
                }
                rec = rec.wrapping_add(STRIDE);
                left -= 1;
            }
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(RELEASE, u32, group);
        0
    }
});
