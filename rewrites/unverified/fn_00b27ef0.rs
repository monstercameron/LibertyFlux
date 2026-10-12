// original: 0x00B27EF0 toggle_motion_flag_and_request_update
/// Toggle the receiver's update-request bit when the supplied byte's low bit
/// is set, then call its update dispatch object with event code 0x10 and the
/// original argument. Return the dispatch result. The dispatch object is
/// reached through receiver offsets +0x24 and +0x7b0.
const REQUEST_UPDATE_BIT: u32 = 0x40;
const UPDATE_DISPATCH_OFFSET: u32 = 0x24;
const UPDATE_METHOD_OFFSET: u32 = 0x7b0;

lf_checker_rt::export!(thiscall, rw_b27ef0(this: u32, amount: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 { (address as *const u32).read_unaligned() }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) { (address as *mut u32).write_unaligned(value); }
        let flags_address = this.wrapping_add(0x50);
        if amount as u8 & 1 != 0 {
            write_word(flags_address, read_word(flags_address) ^ REQUEST_UPDATE_BIT);
        }
        let dispatch = read_word(this.wrapping_add(UPDATE_DISPATCH_OFFSET));
        let target = read_word(dispatch.wrapping_add(UPDATE_METHOD_OFFSET));
        lf_checker_rt::callee_thiscall!(1, u32, target, 0x10, amount)
    }
});
