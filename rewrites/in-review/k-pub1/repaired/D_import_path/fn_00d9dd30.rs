// original: 0x00D9DD30 vehicle_target_within_heading_gate

/// Accept immediately when the object's enable byte is zero. Otherwise reject
/// a null second argument and ask the heading-range helper whether the target
/// is within range. The helper's one-bit answer determines the result.
///
/// The x86 thiscall receives the object in ECX and four stack words. The
/// helper is called with `this + 8` in ECX and five stack words in source
/// order: the four original arguments with the object's word at `+4` inserted
/// between the second and third arguments.
use support::{read_u8, read_u32};

lf_checker_rt::export!(thiscall, rw_00D9DD30(this: u32, first: u32, target: u32, third: u32, fourth: u32) -> u8 {
    const ENABLED: u32 = 0;
    const TARGET_SLOT: u32 = 4;
    const HELPER_THIS: u32 = 8;
    const HEADING_HELPER: u32 = 3;
    unsafe {
        if read_u8(this.wrapping_add(ENABLED)) == 0 {
            return 1;
        }
        if target == 0 {
            return 0;
        }
        let object_word = read_u32(this.wrapping_add(TARGET_SLOT));
        let accepted = lf_checker_rt::callee_thiscall!(
            HEADING_HELPER, u8, this.wrapping_add(HELPER_THIS), first, target,
            object_word, third, fourth
        );
        u8::from(accepted != 0)
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
