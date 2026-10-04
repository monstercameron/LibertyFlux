// original: 0x00a96c70 fade_copy
/// Copies a channel record field by field, skipping the live cursor words.
///
/// Transfers the state word, bounds, ratio, opaque words and the three tag
/// bytes; the words at offsets 4, 8 and 0xC keep the destination's values.
/// Returns the destination pointer.
export!(thiscall, rw_00a96c70(this: u32, src: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = *(src as *const u32);
        *((this + 0x10) as *mut u32) = *((src + 0x10) as *const u32);
        *((this + 0x14) as *mut u32) = *((src + 0x14) as *const u32);
        *((this + 0x18) as *mut u32) = *((src + 0x18) as *const u32);
        *((this + 0x1C) as *mut u32) = *((src + 0x1C) as *const u32);
        *((this + 0x20) as *mut u32) = *((src + 0x20) as *const u32);
        *((this + 0x24) as *mut u32) = *((src + 0x24) as *const u32);
        *((this + 0x28) as *mut u32) = *((src + 0x28) as *const u32);
        *((this + 0x2C) as *mut u32) = *((src + 0x2C) as *const u32);
        *((this + 0x30) as *mut u32) = *((src + 0x30) as *const u32);
        *((this + 0x34) as *mut u32) = *((src + 0x34) as *const u32);
        *((this + 0x38) as *mut u32) = *((src + 0x38) as *const u32);
        *((this + 0x3C) as *mut u32) = *((src + 0x3C) as *const u32);
        *((this + 0x40) as *mut u32) = *((src + 0x40) as *const u32);
        *((this + 0x44) as *mut u32) = *((src + 0x44) as *const u32);
        *((this + 0x48) as *mut u32) = *((src + 0x48) as *const u32);
        *((this + 0x4C) as *mut u32) = *((src + 0x4C) as *const u32);
        *((this + 0x50) as *mut u8) = *((src + 0x50) as *const u8);
        *((this + 0x51) as *mut u8) = *((src + 0x51) as *const u8);
        *((this + 0x52) as *mut u8) = *((src + 0x52) as *const u8);
        this
    }
});
