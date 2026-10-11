// original: 0x00D9C4C0 initialize_vehicle_vector_record

/// Initialize the leading 24-byte record: clear its first byte and the
/// five dwords at offsets `+4` through `+0x14`, then return `this`. The upper
/// three bytes of the first word are left untouched.

use crate::support::{write_u8, write_u32};

lf_checker_rt::export!(thiscall, rw_00D9C4C0(this: u32) -> u32 {
    unsafe {
        write_u8(this, 0);
        for offset in [0x04, 0x08, 0x0C, 0x10, 0x14] {
            write_u32(this.wrapping_add(offset), 0);
        }
        this
    }
});
