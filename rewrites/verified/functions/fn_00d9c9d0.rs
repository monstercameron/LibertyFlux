// original: 0x00D9C9D0 copy_vehicle_transform_record

/// Copy a 28-byte record from the stack pointer `source` to `this`: three
/// eight-byte blocks at offsets 0, 8 and 16, followed by one dword at 24.
/// Return `this`; the callee removes the source pointer argument.

use crate::support::{read_u32, read_u64, write_u32, write_u64};

lf_checker_rt::export!(thiscall, rw_00D9C9D0(this: u32, source: u32) -> u32 {
    unsafe {
        for offset in [0, 8, 16] {
            write_u64(this.wrapping_add(offset), read_u64(source.wrapping_add(offset)));
        }
        write_u32(this.wrapping_add(24), read_u32(source.wrapping_add(24)));
        this
    }
});
