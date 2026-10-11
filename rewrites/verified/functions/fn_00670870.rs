// original: 0x00670870 streaming_unpack_rgb_u32

/// Read an opaque packed color through `this` and write red, green and blue
/// floats to the one stack pointer. Each byte is converted to binary32 and
/// multiplied by the original binary32 reciprocal of 255, in that order.
/// The alpha byte is ignored and the output pointer is returned.
///
/// Original: 0x00670870 (thiscall, one stack pointer, pointer return).
lf_checker_rt::export!(thiscall, rw_00670870(this: u32, output: u32) -> u32 {
    unsafe {
        const INV_255: f32 = f32::from_bits(0x3B80_8081);
        let packed = (this as *const u32).read_unaligned();
        let red = ((packed >> 16) & 0xFF) as f32;
        let green = ((packed >> 8) & 0xFF) as f32;
        let blue = (packed & 0xFF) as f32;
        let out = output as *mut f32;
        out.add(0).write_unaligned(core::hint::black_box(red) * core::hint::black_box(INV_255));
        out.add(1).write_unaligned(core::hint::black_box(green) * core::hint::black_box(INV_255));
        out.add(2).write_unaligned(core::hint::black_box(blue) * core::hint::black_box(INV_255));
        output
    }
});
