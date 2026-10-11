// original: 0x00665AE0 rage::evtInstance::vf2

/// Copy the mutable portion of an event instance into the destination, leaving
/// its leading vtable word intact. Four words at offsets 4 through 0x10, one
/// 16-bit field at 0x14 and bytes at 0x16 and 0x17 are copied. The 32-bit
/// thiscall returns the final copied byte in EAX and pops its source pointer.
lf_checker_rt::export!(thiscall, rw_00665AE0(destination: u32, source: u32) -> u32 {
    unsafe {
        for offset in [4u32, 8, 12, 16] {
            let value = (source.wrapping_add(offset) as *const u32).read_unaligned();
            (destination.wrapping_add(offset) as *mut u32).write_unaligned(value);
        }
        let short_value = (source.wrapping_add(20) as *const u16).read_unaligned();
        (destination.wrapping_add(20) as *mut u16).write_unaligned(short_value);
        let penultimate = (source.wrapping_add(22) as *const u8).read();
        (destination.wrapping_add(22) as *mut u8).write(penultimate);
        let final_byte = (source.wrapping_add(23) as *const u8).read();
        (destination.wrapping_add(23) as *mut u8).write(final_byte);
        u32::from(final_byte)
    }
});
