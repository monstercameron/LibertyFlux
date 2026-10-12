// original: 0x00D99B00 CBoat::vf101

/// Invoke the vehicle's refresh helper with this object, then clear six
/// consecutive 32-bit state slots in the component and collision records.
/// The helper's 32-bit return value is forwarded unchanged.
///
/// This is a 32-bit thiscall method: the object is in ECX and there are no
/// stack arguments. The cleared slots are at byte offsets 0x1650, 0x1654,
/// 0x1658, 0x1660, 0x1664 and 0x1668.
use support::write_u32;

lf_checker_rt::export!(thiscall, rw_00D99B00(this: u32) -> u32 {
    const REFRESH_CALL: u32 = 1;
    const REFRESH_SLOTS: [u32; 6] = [0x1650, 0x1654, 0x1658, 0x1660, 0x1664, 0x1668];
    let result = lf_checker_rt::callee_thiscall!(REFRESH_CALL, u32, this);
    unsafe {
        for offset in REFRESH_SLOTS {
            write_u32(this.wrapping_add(offset), 0);
        }
    }
    result
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
