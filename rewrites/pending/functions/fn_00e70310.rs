// original: 0x00e70310 hashctx_quad_init
/// Initialise four hash-context records ending at 0x01A05E44.
///
/// Walks four records of 0xD20 bytes from the top down, clearing their
/// counter, state and flag fields, stamping the MD5 initial chaining values
/// and the handler tag, then invoking the per-record setup callee
/// (thiscall/0) on each. Returns the last setup answer, matching the value
/// the original leaves in EAX.
export!(cdecl, rw_00e70310() -> u32 {
    unsafe {
        const TOP: u32 = 0x01A05E44;
        const COUNT: u32 = 4;
        const STRIDE: u32 = 0xD20;
        const TAG_FIRST: u32 = 0x00FE588C;
        const TAG_FINAL: u32 = 0x00FE585C;
        let mut ptr = relocated(TOP);
        let mut last = 0u32;
        let mut remaining = COUNT;
        loop {
            ptr = ptr.wrapping_sub(STRIDE);
            let base = ptr;
            core::ptr::write_unaligned((base.wrapping_sub(0x11C)) as *mut u32, relocated(TAG_FIRST));
            let flags = (base + 0x21F) as *mut u8;
            core::ptr::write_unaligned(flags, core::ptr::read_unaligned(flags) & 0xF8);
            core::ptr::write_unaligned((base + 0x104) as *mut u32, 0);
            core::ptr::write_unaligned((base + 0x108) as *mut u32, 0);
            core::ptr::write_unaligned((base + 0x20E) as *mut u8, 0);
            core::ptr::write_unaligned((base + 0x2B4) as *mut u32, 0);
            core::ptr::write_unaligned((base.wrapping_sub(0x114)) as *mut u32, 0);
            core::ptr::write_unaligned((base.wrapping_sub(0x10C)) as *mut u32, 0);
            core::ptr::write_unaligned((base.wrapping_sub(0x110)) as *mut u32, 0);
            core::ptr::write_unaligned((base.wrapping_sub(6)) as *mut u8, 0);
            core::ptr::write_unaligned((base + 0x14) as *mut u32, 0);
            core::ptr::write_unaligned((base + 0x18) as *mut u32, 0);
            core::ptr::write_unaligned((base.wrapping_sub(4)) as *mut u32, 0x67452301);
            core::ptr::write_unaligned(base as *mut u32, 0xEFCDAB89);
            core::ptr::write_unaligned((base + 4) as *mut u32, 0x98BADCFE);
            core::ptr::write_unaligned((base + 8) as *mut u32, 0x10325476);
            core::ptr::write_unaligned((base + 0xC) as *mut u32, 0xC3D2E1F0);
            core::ptr::write_unaligned((base + 0xDC) as *mut u32, 0);
            core::ptr::write_unaligned((base + 0xE4) as *mut u8, 0);
            core::ptr::write_unaligned((base.wrapping_sub(0x11C)) as *mut u32, relocated(TAG_FINAL));
            last = lf_checker_rt::callee_thiscall!(1, u32, base.wrapping_sub(0x718));
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
        last
    }
});
