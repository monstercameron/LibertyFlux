// original: 0x00880A30 audio_convert_planar_floats_to_int16

/// Convert interleaved `f32` samples to sequential signed 16-bit samples.
///
/// The receiver holds the input pointer at byte offset `0x30` and output
/// pointer at `0x40`. Channel count, frame count, and the final scale are read
/// from their image globals; the positive conversion factor is loaded from
/// read-only image data. The outer loop walks channels and the inner loop walks
/// frames, reading input index `frame * channels + channel` and writing output
/// sequentially. Each sample is multiplied by the image factor first and the
/// global scale second, then converted with x86 truncation semantics; NaN and
/// out-of-range values produce `i32::MIN`, of which the low 16 bits are stored.
/// EAX is not a function result and is excluded from the contract comparison.
lf_checker_rt::export!(thiscall, rw_00880a30(this: u32) -> u32 {
    unsafe {
        const INPUT_POINTER_OFFSET: u32 = 0x30;
        const OUTPUT_POINTER_OFFSET: u32 = 0x40;
        const SCALE_GLOBAL_VA: u32 = 0x01030000;
        const FRAME_COUNT_GLOBAL_VA: u32 = 0x01030004;
        const CHANNEL_COUNT_GLOBAL_VA: u32 = 0x01030008;
        const CONVERSION_FACTOR_VA: u32 = 0x00E758FC;

        let mut channel_limit = lf_checker_rt::global::<u32>(CHANNEL_COUNT_GLOBAL_VA)
            .read_unaligned();
        let scale = lf_checker_rt::global::<f32>(SCALE_GLOBAL_VA).read_unaligned();
        if channel_limit != 0 {
            let conversion_factor = (lf_checker_rt::relocated(CONVERSION_FACTOR_VA)
                as *const f32)
                .read_unaligned();
            let mut frame_limit = lf_checker_rt::global::<u32>(FRAME_COUNT_GLOBAL_VA)
                .read_unaligned();
            let mut channel_index = 0u32;
            let mut output_index = 0u32;

            loop {
                let mut frame_index = 0u32;
                if frame_limit != 0 {
                    loop {
                        let input_index = frame_index
                            .wrapping_mul(channel_limit)
                            .wrapping_add(channel_index);
                        let input_base = (this.wrapping_add(INPUT_POINTER_OFFSET)
                            as *const u32)
                            .read_unaligned();
                        let sample = (input_base.wrapping_add(input_index.wrapping_mul(4))
                            as *const f32)
                            .read_unaligned();
                        let output_base = (this.wrapping_add(OUTPUT_POINTER_OFFSET)
                            as *const u32)
                            .read_unaligned();

                        let scaled_by_factor = core::hint::black_box(sample)
                            * core::hint::black_box(conversion_factor);
                        let scaled = core::hint::black_box(scaled_by_factor)
                            * core::hint::black_box(scale);
                        let converted = if !scaled.is_finite()
                            || scaled >= 2147483648.0f32
                            || scaled < -2147483648.0f32
                        {
                            i32::MIN
                        } else {
                            // The preceding bounds match CVTTSS2SI's valid signed range.
                            scaled.trunc() as i32
                        };
                        (output_base.wrapping_add(output_index.wrapping_mul(2))
                            as *mut u16)
                            .write_unaligned(converted as u16);
                        output_index = output_index.wrapping_add(1);

                        frame_index = frame_index.wrapping_add(1);
                        frame_limit = lf_checker_rt::global::<u32>(FRAME_COUNT_GLOBAL_VA)
                            .read_unaligned();
                        channel_limit = lf_checker_rt::global::<u32>(CHANNEL_COUNT_GLOBAL_VA)
                            .read_unaligned();
                        if frame_index >= frame_limit {
                            break;
                        }
                    }
                }

                channel_index = channel_index.wrapping_add(1);
                if channel_index >= channel_limit {
                    break;
                }
            }
        }

        0
    }
});
