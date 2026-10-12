// original: 0x00AAC350 stream_packed_fields_unpack

/// Decode one packed stream field into four output words.
///
/// The C calling convention receives `(output, stream, item_index, byte_offset)`.
/// The stream stores a data base at `+8`, a byte stride at `+12`, and a
/// descriptor pointer at `+16`. The selected source address is the wrapping
/// sum of the base, stride times the item index, and byte offset. When the
/// descriptor nibble at `+8` is ten, the source word is split into signed
/// 11-bit, 22-bit, and 10-bit values and scaled by two read-only image floats;
/// otherwise, four source words are copied unchanged. The original's fourth
/// output float comes from an unwritten frame word at `[esp+0x0c]` after its
/// 16-byte local allocation. This proof supplies zero there, so the fourth
/// output word is zero only under that narrowed stack condition. The output
/// pointer is returned in EAX.
lf_checker_rt::export!(cdecl, rw_00aac350(output: u32, stream: u32, item_index: u32, byte_offset: u32) -> u32 {
    const STREAM_DATA_BASE: u32 = 0x08;
    const STREAM_ITEM_STRIDE: u32 = 0x0c;
    const STREAM_DESCRIPTOR: u32 = 0x10;
    const DESCRIPTOR_KIND: u32 = 0x08;
    const PACKED_KIND: u32 = 10;
    const SCALE_XY_RVA: u32 = 0x00be_86b0;
    const SCALE_Z_RVA: u32 = 0x00be_86cc;

    #[inline(always)]
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_word(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }
    #[inline(always)]
    fn multiply(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }

    unsafe {
        let stream_base = read_word(stream.wrapping_add(STREAM_DATA_BASE));
        let item_stride = read_word(stream.wrapping_add(STREAM_ITEM_STRIDE));
        let data_address = stream_base
            .wrapping_add(item_stride.wrapping_mul(item_index))
            .wrapping_add(byte_offset);
        let descriptor = read_word(stream.wrapping_add(STREAM_DESCRIPTOR));
        let kind = (read_word(descriptor.wrapping_add(DESCRIPTOR_KIND)) >> 12) & 0x0f;
        let output_bytes = output as *mut u8;

        if kind != PACKED_KIND {
            for word_index in 0..4u32 {
                let value = read_word(data_address.wrapping_add(word_index * 4));
                write_word(output.wrapping_add(word_index * 4), value);
            }
            return output;
        }

        let packed = read_word(data_address);
        let first = ((packed << 21) as i32) >> 21;
        let second = ((packed << 10) as i32) >> 21;
        let third = (packed as i32) >> 22;
        let image_base = lf_checker_rt::xbase();
        let xy_scale = f32::from_bits(read_word(image_base.wrapping_add(SCALE_XY_RVA)));
        let z_scale = f32::from_bits(read_word(image_base.wrapping_add(SCALE_Z_RVA)));

        let x = multiply(first as f32, xy_scale);
        let y = multiply(second as f32, xy_scale);
        let z = multiply(third as f32, z_scale);
        output_bytes.cast::<f32>().write_unaligned(x);
        output_bytes.add(4).cast::<f32>().write_unaligned(y);
        output_bytes.add(8).cast::<f32>().write_unaligned(z);
        output_bytes.add(12).cast::<u32>().write_unaligned(0);
    }

    output
});
