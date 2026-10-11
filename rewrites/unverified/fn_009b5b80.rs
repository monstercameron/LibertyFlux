// original: 0x009B5B80 script_state_check_ready_flags

/// Clear and rebuild bit one in the script-state status byte.
///
/// The object pointer is passed in ECX and its status byte is at `+0xd2`.
/// The routine first clears bit one, then performs two manager lookups using
/// selectors two and nine. A non-null first record with a nonzero byte at
/// `+0x34a`, or a non-null second record with a nonzero byte at `+0x2c4`, sets
/// that status bit. The function returns the resulting bit in AL; the upper
/// return-register bytes are not part of the return value.
lf_checker_rt::export!(thiscall, rw_009b5b80(this: u32) -> u32 {
    unsafe {
        const STATUS_FLAGS: u32 = 0xd2;
        const STATUS_READY: u8 = 0x02;
        const GLOBAL_MANAGER: u32 = 0x0103_e49c;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }

        let status_address = this.wrapping_add(STATUS_FLAGS);
        write_u8(status_address, read_u8(status_address) & !STATUS_READY);
        let manager = read_u32(lf_checker_rt::relocated(GLOBAL_MANAGER));

        let first_record = lf_checker_rt::callee_thiscall!(1, u32, manager, 2u32, 0u32);
        if first_record != 0 && read_u8(first_record.wrapping_add(0x34a)) != 0 {
            write_u8(status_address, read_u8(status_address) | STATUS_READY);
        }

        let second_record = lf_checker_rt::callee_thiscall!(2, u32, manager, 9u32, 0u32);
        if second_record != 0 && read_u8(second_record.wrapping_add(0x2c4)) != 0 {
            write_u8(status_address, read_u8(status_address) | STATUS_READY);
        }
        u32::from((read_u8(status_address) >> 1) & 1)
    }
});
