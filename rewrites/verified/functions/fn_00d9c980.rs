// original: 0x00D9C980 initialize_vehicle_component_slot

/// Initialize a component slot with sentinel `u32::MAX`, mode `0x10`, and
/// zeroed scalar and byte fields. Write byte fields only at `+8`, `+0x10`,
/// `+0x18` and `+0x20`, and dword fields at `+0x0C`, `+0x14`, `+0x1C`,
/// `+0x30`, `+0x34` and `+0x38`; gaps between these fields are preserved.
/// Return `this`.

use crate::support::{write_u8, write_u32};

lf_checker_rt::export!(thiscall, rw_00D9C980(this: u32) -> u32 {
    unsafe {
        write_u32(this, u32::MAX);
        write_u32(this.wrapping_add(4), 0x10);
        for offset in [8, 0x10, 0x18, 0x20] {
            write_u8(this.wrapping_add(offset), 0);
        }
        for offset in [0x0C, 0x14, 0x1C, 0x30, 0x34, 0x38] {
            write_u32(this.wrapping_add(offset), 0);
        }
        this
    }
});
