// original: 0x00D9CA40 copy_vehicle_transform_vectors

/// Copy three eight-byte blocks (24 bytes total) from `source` to `this`.
/// Return `this`; the callee removes the source pointer argument.

use lf_checker_rt::support::{read_u64, write_u64};

lf_checker_rt::export!(thiscall, rw_00D9CA40(this: u32, source: u32) -> u32 {
    unsafe {
        for offset in [0, 8, 16] {
            write_u64(this.wrapping_add(offset), read_u64(source.wrapping_add(offset)));
        }
        this
    }
});

    #[inline(always)]
    pub(crate) unsafe fn write_u64(address: u32, value: u64) { unsafe { (address as *mut u64).write_unaligned(value) } }

    #[inline(always)]
    pub(crate) unsafe fn read_u64(address: u32) -> u64 { unsafe { (address as *const u64).read_unaligned() } }
