// original: 0x00D95700 mainloop_timing_advance_serial_b

/// Increment the adjacent 16-bit maintenance serial and dispatch its wrap event.
///
/// The object stores the serial at byte offset `0xc40`. Incrementing wraps as
/// a `u16`. When the increment reaches `0xffff`, the field is reset to one and
/// a zero-argument callback is tail-dispatched. The ordinary path returns the
/// 16-bit sentinel in EAX; the dispatched path returns the callback result.
lf_checker_rt::export!(thiscall, rw_00d95700(object: u32) -> u32 {
    unsafe {
        const SERIAL_FIELD: u32 = 0xc40;
        const WRAP_SENTINEL: u16 = 0xffff;
        const RESET_SERIAL: u16 = 1;
        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u16(address: u32, value: u16) {
            unsafe { (address as *mut u16).write_unaligned(value) }
        }

        let serial_address = object.wrapping_add(SERIAL_FIELD);
        let next_serial = read_u16(serial_address).wrapping_add(1);
        write_u16(serial_address, next_serial);
        if next_serial == WRAP_SENTINEL {
            write_u16(serial_address, RESET_SERIAL);
            lf_checker_rt::callee_cdecl!(2, u32,)
        } else {
            u32::from(WRAP_SENTINEL)
        }
    }
});
