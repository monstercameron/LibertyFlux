// original: 0x00D9C580 copy_vehicle_physics_record

/// Copy a 48-byte record from `source` to `this`. Copy the dword at `+0`,
/// the five individual bytes at `+4..+8`, then the nine dwords at
/// `+0x0C..+0x2C`. Return `this`; the callee removes the one stack argument.

use lf_checker_rt::support::{read_u8, read_u32, write_u8, write_u32};

lf_checker_rt::export!(thiscall, rw_00D9C580(this: u32, source: u32) -> u32 {
    unsafe {
        write_u32(this, read_u32(source));
        for offset in 4..=8 {
            write_u8(this.wrapping_add(offset), read_u8(source.wrapping_add(offset)));
        }
        for offset in (0x0C..=0x2C).step_by(4) {
            write_u32(this.wrapping_add(offset), read_u32(source.wrapping_add(offset)));
        }
        this
    }
});

    #[inline(always)]
    pub(crate) unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }

    #[inline(always)]
    pub(crate) unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }

    #[inline(always)]
    pub(crate) unsafe fn write_u8(address: u32, value: u8) { unsafe { (address as *mut u8).write(value) } }

    #[inline(always)]
    pub(crate) unsafe fn read_u8(address: u32) -> u8 { unsafe { (address as *const u8).read() } }
