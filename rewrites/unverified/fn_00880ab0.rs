// original: 0x00880AB0 audio_interleave_floats_to_int16

/// Convert float samples in eight-sample blocks, then reorder the packed
/// 16-bit results from frame-major input into channel-major output.
///
/// The receiver stores input, packed temporary, and final output pointers at
/// offsets `0x30`, `0x3c`, and `0x40`. The vector stage rounds products using
/// the worker's default MXCSR mode, saturates packed signed words, and writes
/// complete blocks of eight, including the final padding block. The second
/// stage copies exactly `frames * channels` words using source index
/// `frame * channels + channel`. The global scale is multiplied by the image
/// factor before each sample is multiplied by that combined scale. The void
/// routine's EAX return is not compared.
lf_checker_rt::export!(thiscall, rw_00880ab0(this: u32) -> u32 {
    unsafe {
        const INPUT_POINTER_OFFSET: u32 = 0x30;
        const PACKED_TEMP_POINTER_OFFSET: u32 = 0x3C;
        const OUTPUT_POINTER_OFFSET: u32 = 0x40;
        const SCALE_GLOBAL_VA: u32 = 0x01030000;
        const FRAME_COUNT_GLOBAL_VA: u32 = 0x01030004;
        const CHANNEL_COUNT_GLOBAL_VA: u32 = 0x01030008;
        const CONVERSION_FACTOR_VA: u32 = 0x00E758FC;
        const VECTOR_BLOCK_SAMPLES: u32 = 8;

        let scale = lf_checker_rt::global::<f32>(SCALE_GLOBAL_VA).read_unaligned();
        let conversion_factor = (lf_checker_rt::relocated(CONVERSION_FACTOR_VA)
            as *const f32)
            .read_unaligned();
        let combined_scale = core::hint::black_box(scale)
            * core::hint::black_box(conversion_factor);
        let mut channel_count = lf_checker_rt::global::<u32>(CHANNEL_COUNT_GLOBAL_VA)
            .read_unaligned();
        let mut frame_count = lf_checker_rt::global::<u32>(FRAME_COUNT_GLOBAL_VA)
            .read_unaligned();
        let total_samples = frame_count.wrapping_mul(channel_count);

        if total_samples != 0 {
            let input_base = (this.wrapping_add(INPUT_POINTER_OFFSET) as *const u32)
                .read_unaligned();
            let packed_temp_base = (this.wrapping_add(PACKED_TEMP_POINTER_OFFSET)
                as *const u32)
                .read_unaligned();
            let mut block_start = 0u32;

            loop {
                // The original loads all eight samples before storing the block.
                let mut samples = [0.0f32; VECTOR_BLOCK_SAMPLES as usize];
                for lane in 0..VECTOR_BLOCK_SAMPLES {
                    let sample_index = block_start.wrapping_add(lane);
                    samples[lane as usize] = (input_base
                        .wrapping_add(sample_index.wrapping_mul(4))
                        as *const f32)
                        .read_unaligned();
                }

                let mut packed = [0i16; VECTOR_BLOCK_SAMPLES as usize];
                for lane in 0..VECTOR_BLOCK_SAMPLES {
                    let product = core::hint::black_box(samples[lane as usize])
                        * core::hint::black_box(combined_scale);
                    let converted = if !product.is_finite()
                        || product >= 2147483648.0f32
                        || product < -2147483648.0f32
                    {
                        i32::MIN
                    } else {
                        // CVTPS2DQ follows the default round-to-nearest-even mode.
                        product.round_ties_even() as i32
                    };
                    packed[lane as usize] = converted
                        .clamp(i16::MIN as i32, i16::MAX as i32) as i16;
                }

                for lane in 0..VECTOR_BLOCK_SAMPLES {
                    let sample_index = block_start.wrapping_add(lane);
                    (packed_temp_base.wrapping_add(sample_index.wrapping_mul(2))
                        as *mut u16)
                        .write_unaligned(packed[lane as usize] as u16);
                }

                block_start = block_start.wrapping_add(VECTOR_BLOCK_SAMPLES);
                if block_start >= total_samples {
                    break;
                }
            }
        }

        let output_base = (this.wrapping_add(OUTPUT_POINTER_OFFSET) as *const u32)
            .read_unaligned();
        let packed_temp_base = (this.wrapping_add(PACKED_TEMP_POINTER_OFFSET)
            as *const u32)
            .read_unaligned();
        let mut output_index = 0u32;
        for channel_index in 0..channel_count {
            for frame_index in 0..frame_count {
                let input_index = frame_index
                    .wrapping_mul(channel_count)
                    .wrapping_add(channel_index);
                let sample = (packed_temp_base.wrapping_add(input_index.wrapping_mul(2))
                    as *const u16)
                    .read_unaligned();
                (output_base.wrapping_add(output_index.wrapping_mul(2)) as *mut u16)
                    .write_unaligned(sample);
                output_index = output_index.wrapping_add(1);

                frame_count = lf_checker_rt::global::<u32>(FRAME_COUNT_GLOBAL_VA)
                    .read_unaligned();
                channel_count = lf_checker_rt::global::<u32>(CHANNEL_COUNT_GLOBAL_VA)
                    .read_unaligned();
            }
        }

        0
    }
});
