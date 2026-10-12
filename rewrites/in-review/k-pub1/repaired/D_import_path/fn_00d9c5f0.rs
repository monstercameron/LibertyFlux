// original: 0x00D9C5F0 initialize_vehicle_handling_record

/// Initialize a 48-byte handling record. Set the leading kind dword to 3,
/// clear the dword at `+4`, clear the byte at `+8`, and clear each dword from
/// `+0x0C` through `+0x2C`. Return `this`.

use lf_checker_rt::support::{write_u8, write_u32};

lf_checker_rt::export!(thiscall, rw_00D9C5F0(this: u32) -> u32 {
    unsafe {
        write_u32(this, 3);
        write_u32(this.wrapping_add(4), 0);
        write_u8(this.wrapping_add(8), 0);
        for offset in (0x0C..=0x2C).step_by(4) {
            write_u32(this.wrapping_add(offset), 0);
        }
        this
    }
});

    #[inline(always)]
    pub(crate) unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }

    #[inline(always)]
    pub(crate) unsafe fn write_u8(address: u32, value: u8) { unsafe { (address as *mut u8).write(value) } }
