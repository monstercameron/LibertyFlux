// original: 0x00D9C4C0 initialize_vehicle_vector_record

/// Initialize the leading 24-byte record: clear its first byte and the
/// five dwords at offsets `+4` through `+0x14`, then return `this`. The upper
/// three bytes of the first word are left untouched.

use support::{write_u8, write_u32};

lf_checker_rt::export!(thiscall, rw_00D9C4C0(this: u32) -> u32 {
    unsafe {
        write_u8(this, 0);
        for offset in [0x04, 0x08, 0x0C, 0x10, 0x14] {
            write_u32(this.wrapping_add(offset), 0);
        }
        this
    }
});

pub(crate) mod support {
    #[inline(always)]
    pub(crate) unsafe fn read_u8(address: u32) -> u8 { unsafe { (address as *const u8).read() } }
    #[inline(always)]
    pub(crate) unsafe fn read_u16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
    #[inline(always)]
    pub(crate) unsafe fn write_u8(address: u32, value: u8) { unsafe { (address as *mut u8).write(value) } }
    #[inline(always)]
    pub(crate) unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    #[inline(always)]
    pub(crate) unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }
    #[inline(always)]
    pub(crate) unsafe fn read_u64(address: u32) -> u64 { unsafe { (address as *const u64).read_unaligned() } }
    #[inline(always)]
    pub(crate) unsafe fn write_u64(address: u32, value: u64) { unsafe { (address as *mut u64).write_unaligned(value) } }
}

    #[inline(always)]
    pub(crate) unsafe fn read_u16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }

    #[inline(always)]
    pub(crate) unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }

    #[inline(always)]
    pub(crate) unsafe fn read_u64(address: u32) -> u64 { unsafe { (address as *const u64).read_unaligned() } }

    #[inline(always)]
    pub(crate) unsafe fn read_u8(address: u32) -> u8 { unsafe { (address as *const u8).read() } }

    #[inline(always)]
    pub(crate) unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }

    #[inline(always)]
    pub(crate) unsafe fn write_u64(address: u32, value: u64) { unsafe { (address as *mut u64).write_unaligned(value) } }

    #[inline(always)]
    pub(crate) unsafe fn write_u8(address: u32, value: u8) { unsafe { (address as *mut u8).write(value) } }
