// original: 0x009B6410 script_reset_component_defaults

/// Reset the component's readiness flags, three position vectors, and angle
/// defaults. The thiscall object is addressed in ECX. The routine preserves
/// only the low two bits of the status byte at `+0xd1`, clears bit zero at
/// `+0xd2`, sets mode byte `+0xd0` to two and active byte `+0x94` to one, and
/// zeros nine dwords at `+0x80..+0xb8`. Four float fields are initialized by
/// writing the exact bit patterns for 0.1 and 0.5. It returns one in AL.
lf_checker_rt::export!(thiscall, rw_009b6410(this: u32) -> u32 {
    unsafe {
        const STATUS_FLAGS: u32 = 0xd1;
        const READY_FLAGS: u32 = 0xd2;
        const ACTIVE: u32 = 0x94;
        const MODE: u32 = 0xd0;
        const ZERO_DWORDS: [u32; 9] = [0xa8, 0xa4, 0xa0, 0xb8, 0xb4, 0xb0, 0x88, 0x84, 0x80];
        const FLOAT_DEFAULTS: [(u32, u32); 4] = [
            (0xc0, 0x3dcc_cccd), (0xc4, 0x3dcc_cccd),
            (0xc8, 0x3f00_0000), (0xcc, 0x3f00_0000),
        ];

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let status_address = this.wrapping_add(STATUS_FLAGS);
        write_u8(status_address, read_u8(status_address) & 0x03);
        let ready_address = this.wrapping_add(READY_FLAGS);
        write_u8(ready_address, read_u8(ready_address) & !1);
        for offset in ZERO_DWORDS {
            write_u32(this.wrapping_add(offset), 0);
        }
        write_u8(this.wrapping_add(ACTIVE), 1);
        write_u8(this.wrapping_add(MODE), 2);
        for (offset, bits) in FLOAT_DEFAULTS {
            write_u32(this.wrapping_add(offset), bits);
        }
        1
    }
});
