// original: 0x00D9C9D0 copy_vehicle_transform_record

/// Copy a 28-byte record from the stack pointer `source` to `this`: three
/// eight-byte blocks at offsets 0, 8 and 16, followed by one dword at 24.
/// Return `this`; the callee removes the source pointer argument.

use support::{read_u32, read_u64, write_u32, write_u64};

lf_checker_rt::export!(thiscall, rw_00D9C9D0(this: u32, source: u32) -> u32 {
    unsafe {
        for offset in [0, 8, 16] {
            write_u64(this.wrapping_add(offset), read_u64(source.wrapping_add(offset)));
        }
        write_u32(this.wrapping_add(24), read_u32(source.wrapping_add(24)));
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
