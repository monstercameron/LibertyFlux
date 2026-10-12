// original: 0x00D9C550 initialize_vehicle_physics_record

/// Initialize a 28-byte record by clearing the first byte, the dword at
/// `+4`, the byte at `+8`, and the dwords at `+0x0C`, `+0x10`, `+0x14` and
/// `+0x18`. Bytes `+1..+3` and `+9..+0x0B` remain unchanged. Return `this`.

use lf_checker_rt::support::{write_u8, write_u32};

lf_checker_rt::export!(thiscall, rw_00D9C550(this: u32) -> u32 {
    unsafe {
        write_u8(this, 0);
        write_u32(this.wrapping_add(0x04), 0);
        write_u8(this.wrapping_add(0x08), 0);
        for offset in [0x0C, 0x10, 0x14, 0x18] {
            write_u32(this.wrapping_add(offset), 0);
        }
        this
    }
});

    #[inline(always)]
    pub(crate) unsafe fn write_u8(address: u32, value: u8) { unsafe { (address as *mut u8).write(value) } }

    #[inline(always)]
    pub(crate) unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }
