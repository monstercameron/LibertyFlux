// original: 0x009FFFF0 save_block_dispatch_save
/// Searches a signed count of 0x2c-byte records for an exact inline name at +8.
/// It calls the selected record's cdecl callback and returns its full EAX value.
/// A miss clears the low return byte and retains the last unsigned byte-compare
/// result in the upper bytes (or the registry pointer when count is nonpositive).
lf_checker_rt::export!(thiscall, rw_009FFFF0(registry: u32, requested_name: u32) -> u32 {
    unsafe {
        const RECORD_COUNT: u32 = 0x2EC;
        const RECORD_STRIDE: u32 = 0x2C;
        const RECORD_NAME: u32 = 8;
        const CALLBACK: u32 = 0x4;
        const FALSE_LOW_BYTE_MASK: u32 = 0xFFFF_FF00;
        #[inline(always)] unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
        #[inline(always)] unsafe fn read_u8(address: u32) -> u8 { unsafe { (address as *const u8).read() } }

        let signed_count = read_u32(registry.wrapping_add(RECORD_COUNT)) as i32;
        if signed_count <= 0 { return registry & FALSE_LOW_BYTE_MASK; }

        let mut last_mismatch = requested_name;
        for index in 0..signed_count as u32 {
            let record = registry.wrapping_add(index.wrapping_mul(RECORD_STRIDE));
            let record_name = record.wrapping_add(RECORD_NAME);
            let mut byte_offset = 0u32;
            loop {
                let record_byte = read_u8(record_name.wrapping_add(byte_offset));
                let requested_byte = read_u8(requested_name.wrapping_add(byte_offset));
                if record_byte != requested_byte {
                    last_mismatch = if record_byte < requested_byte { u32::MAX } else { 1 };
                    break;
                }
                if record_byte == 0 {
                    let callback_address = read_u32(record.wrapping_add(CALLBACK));
                    let callback: extern "cdecl" fn() -> u32 = core::mem::transmute(callback_address as usize);
                    return callback();
                }
                byte_offset = byte_offset.wrapping_add(1);
            }
        }
        last_mismatch & FALSE_LOW_BYTE_MASK
    }
});
