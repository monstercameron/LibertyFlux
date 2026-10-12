// original: 0x00877230 weighted_average_frame_vectors

/// Accumulate the weighted three-component samples into the output vector.
///
/// The receiver contains sample start/end pointers at `+0/+4`, a weight
/// pointer at `+0x0c`, and the divisor at `+0x40`. The output pointer is the
/// sole stack argument. Three output words are zeroed first. Each 16-byte
/// sample contributes `(sample.xyz * (image_multiplier / divisor) * weight)`;
/// X is accumulated through the output word each iteration while Y and Z are
/// committed after the loop. All float operations retain the original
/// operand order, including NaN payload selection.
///
/// Original calling convention: thiscall with one output pointer; returns void.
lf_checker_rt::export!(thiscall, rw_00877230(this: u32, output: u32) -> () {
    const START_OFFSET: u32 = 0x00;
    const END_OFFSET: u32 = 0x04;
    const WEIGHTS_OFFSET: u32 = 0x0c;
    const DIVISOR_OFFSET: u32 = 0x40;
    const SAMPLE_STRIDE: u32 = 0x10;
    const IMAGE_MULTIPLIER_VA: u32 = 0x00fe88e8;
    unsafe {
        let output_x = output as *mut f32;
        let output_y = output.wrapping_add(4) as *mut f32;
        let output_z = output.wrapping_add(8) as *mut f32;
        output_x.write_unaligned(0.0);
        output_y.write_unaligned(0.0);
        output_z.write_unaligned(0.0);

        let sample_start = (this.wrapping_add(START_OFFSET) as *const u32).read_unaligned();
        let sample_end = (this.wrapping_add(END_OFFSET) as *const u32).read_unaligned();
        if sample_start == sample_end {
            return;
        }
        let mut sample = sample_start;
        let mut remaining = (sample_end.wrapping_sub(sample_start) as i32) >> 4;
        if remaining == 0 {
            return;
        }

        let divisor = f32::from_bits((this.wrapping_add(DIVISOR_OFFSET) as *const u32).read_unaligned());
        let multiplier = f32::from_bits(lf_checker_rt::global::<u32>(IMAGE_MULTIPLIER_VA).read_unaligned());
        let inverse_divisor = core::hint::black_box(multiplier) / core::hint::black_box(divisor);
        let mut weight = (this.wrapping_add(WEIGHTS_OFFSET) as *const u32).read_unaligned();
        let mut accumulated_y = 0.0f32;
        let mut accumulated_z = 0.0f32;
        while remaining != 0 {
            let x = f32::from_bits((sample as *const u32).read_unaligned());
            let y = f32::from_bits((sample.wrapping_add(4) as *const u32).read_unaligned());
            let z = f32::from_bits((sample.wrapping_add(8) as *const u32).read_unaligned());
            let sample_weight = f32::from_bits((weight as *const u32).read_unaligned());
            let scale = core::hint::black_box(inverse_divisor) * core::hint::black_box(sample_weight);
            let scaled_x = core::hint::black_box(x) * core::hint::black_box(scale);
            let scaled_y = core::hint::black_box(y) * core::hint::black_box(scale);
            let scaled_z = core::hint::black_box(z) * core::hint::black_box(scale);
            let old_x = output_x.read_unaligned();
            output_x.write_unaligned(core::hint::black_box(old_x) + core::hint::black_box(scaled_x));
            accumulated_y = core::hint::black_box(accumulated_y) + core::hint::black_box(scaled_y);
            accumulated_z = core::hint::black_box(accumulated_z) + core::hint::black_box(scaled_z);
            sample = sample.wrapping_add(SAMPLE_STRIDE);
            weight = weight.wrapping_add(4);
            remaining -= 1;
        }
        output_y.write_unaligned(accumulated_y);
        output_z.write_unaligned(accumulated_z);
    }
});
