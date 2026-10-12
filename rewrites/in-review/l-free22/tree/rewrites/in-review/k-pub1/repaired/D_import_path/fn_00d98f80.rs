// original: 0x00D98F80 zero_vehicle_state_vectors

/// Clear the scalar mode word and three eight-byte vectors in a vehicle state block.
///
/// `this` points to the block. The routine writes zero to `+0x00` and to
/// the 64-bit ranges at `+0x190`, `+0x198` and `+0x1A0`, then returns without
/// changing any other byte. The zero vectors are bit patterns, not floats.

use support::write_u32;

lf_checker_rt::export!(thiscall, rw_00D98F80(this: u32) -> () {
    unsafe {
        const MODE_WORD: u32 = 0x00;
        const VECTOR0: u32 = 0x190;
        const VECTOR1: u32 = 0x198;
        const VECTOR2: u32 = 0x1A0;
        write_u32(this.wrapping_add(MODE_WORD), 0);
        core::ptr::write_unaligned(this.wrapping_add(VECTOR0) as *mut u64, 0);
        core::ptr::write_unaligned(this.wrapping_add(VECTOR1) as *mut u64, 0);
        core::ptr::write_unaligned(this.wrapping_add(VECTOR2) as *mut u64, 0);
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
