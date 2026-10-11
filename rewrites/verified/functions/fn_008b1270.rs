// original: 0x008B1270 biquad coefficient update
/// Recompute one biquad coefficient set in the object at `this`.
///
/// The bypass flag returns before reading the rate manager. Otherwise the
/// unsigned sample rate is converted to f32, the doubled frequency is
/// clamped against the object's limit, and omega is formed as
/// `(frequency * two_pi) * (1 / sample_rate)`. Types 0 through 10 share six
/// coefficient formulas; each writes five words into the selected set.
/// Trigonometric and auxiliary math calls use scripted checker callees.

#[inline(always)]
fn quiet_nan(value: f32) -> f32 {
    f32::from_bits(value.to_bits() | 0x0040_0000)
}

/// Preserve the SSE destination operand when both inputs are NaN.
#[inline(always)]
fn add_sse(dest: f32, source: f32) -> f32 {
    if dest.is_nan() {
        quiet_nan(dest)
    } else if source.is_nan() {
        quiet_nan(source)
    } else {
        dest + core::hint::black_box(source)
    }
}

/// Preserve the SSE destination operand when both inputs are NaN.
#[inline(always)]
fn sub_sse(dest: f32, source: f32) -> f32 {
    if dest.is_nan() {
        quiet_nan(dest)
    } else if source.is_nan() {
        quiet_nan(source)
    } else {
        dest - core::hint::black_box(source)
    }
}

/// Preserve the SSE destination operand when both inputs are NaN.
#[inline(always)]
fn mul_sse(dest: f32, source: f32) -> f32 {
    if dest.is_nan() {
        quiet_nan(dest)
    } else if source.is_nan() {
        quiet_nan(source)
    } else {
        dest * core::hint::black_box(source)
    }
}

/// Preserve the SSE destination operand when both inputs are NaN.
#[inline(always)]
fn div_sse(dest: f32, source: f32) -> f32 {
    if dest.is_nan() {
        quiet_nan(dest)
    } else if source.is_nan() {
        quiet_nan(source)
    } else {
        dest / core::hint::black_box(source)
    }
}

#[inline(always)]
unsafe fn read_f32(address: u32) -> f32 {
    unsafe { lf_checker_rt::global::<f32>(address).read() }
}

#[inline(always)]
fn cosine(value: f32) -> f32 {
    let bits = core::hint::black_box(value.to_bits());
    let answer: u32 = lf_checker_rt::callee_cdecl!(1, u32, bits);
    f32::from_bits(answer)
}

#[inline(always)]
fn sine(value: f32) -> f32 {
    let bits = core::hint::black_box(value.to_bits());
    let answer: u32 = lf_checker_rt::callee_cdecl!(2, u32, bits);
    f32::from_bits(answer)
}

#[inline(always)]
fn auxiliary_xmm(value: f32) -> f32 {
    let bits = core::hint::black_box(value.to_bits());
    let answer: u32 = lf_checker_rt::callee_cdecl!(4, u32, bits);
    f32::from_bits(answer)
}

#[inline(always)]
fn auxiliary_x87(value: f64) -> f64 {
    let bits = value.to_bits();
    let low = core::hint::black_box(bits as u32);
    let high = core::hint::black_box((bits >> 32) as u32);
    lf_checker_rt::callee_cdecl!(3, f64, low, high)
}

#[inline(always)]
unsafe fn write_coefficients(this: *mut u8, slot: u32, coeff: [f32; 5]) {
    let base = this.add(0x0C + (slot as usize).wrapping_mul(4));
    for (index, value) in coeff.into_iter().enumerate() {
        unsafe { (base.add(index * 0x0C) as *mut u32).write(value.to_bits()) };
    }
}

lf_checker_rt::export!(thiscall, rw_008B1270(this: *mut u8) -> () {
    unsafe {
        if this.add(0xB8).read() != 0 {
            return;
        }

        let manager = lf_checker_rt::global::<u32>(0x115A448).read() as *const u8;
        let sample_rate_bits = (manager.add(0x80) as *const u32).read();
        // The original converts a signed dword to double, adds 2^32 when
        // its sign bit is set, then rounds once to f32: this is uint32->f32.
        let sample_rate = (sample_rate_bits as f64) as f32;
        let frequency = (this.add(0xA8) as *const f32).read();
        let damping = (this.add(0xAC) as *const f32).read();
        let limit = (this.add(0xB0) as *const f32).read();
        let two = read_f32(0x00FE8A24);
        let one = read_f32(0x00FE88E8);
        let half = read_f32(0x00FE8830);
        let two_pi = read_f32(0x00FE8AEC);
        let pi = read_f32(0x00FE8AA0);
        let negative_two = read_f32(0x00FE8DB0);

        let doubled_frequency = mul_sse(frequency, two);
        // COMISS followed by JA skips the local update when 2*frequency is
        // above the bound; unordered values take the update path.
        let bounded_frequency = if doubled_frequency > limit {
            limit
        } else {
            doubled_frequency
        };
        let reciprocal_rate = div_sse(one, sample_rate);
        let omega = mul_sse(mul_sse(frequency, two_pi), reciprocal_rate);
        let kind = (this.add(0xBC) as *const u32).read();
        if kind > 10 {
            return;
        }
        let slot = (this.add(0x08) as *const u32).read();
        let half_scale = this.add(0xC3).read() != 0;
        let scale = if half_scale { half } else { one };

        let coeff = match kind {
            0 | 1 => {
                let cos_omega = cosine(omega);
                let damping_scaled = mul_sse(damping, scale);
                let ratio32 = div_sse(half, damping_scaled);
                let response = auxiliary_x87(ratio32 as f64) as f32;
                let sin_omega = sine(omega);
                let product = mul_sse(response, sin_omega);
                let normalizer = div_sse(one, add_sse(product, one));
                let cosine_delta = sub_sse(one, cos_omega);
                let first_term = mul_sse(cosine_delta, half);
                let first = mul_sse(normalizer, first_term);
                let second = mul_sse(normalizer, cosine_delta);
                let third = first;
                let fourth = mul_sse(mul_sse(cos_omega, negative_two), normalizer);
                let fifth = mul_sse(sub_sse(one, product), normalizer);
                [first, second, third, fourth, fifth]
            }
            2 | 3 => {
                let cos_omega = cosine(omega);
                let damping_scaled = mul_sse(damping, scale);
                let ratio32 = div_sse(half, damping_scaled);
                let response = auxiliary_x87(ratio32 as f64) as f32;
                let sin_omega = sine(omega);
                let product = mul_sse(response, sin_omega);
                let normalizer = div_sse(one, add_sse(product, one));
                let cosine_plus_one = mul_sse(add_sse(cos_omega, one), half);
                let first = mul_sse(normalizer, cosine_plus_one);
                let case2_constant = read_f32(0x00FE8D94);
                let second = mul_sse(sub_sse(case2_constant, cos_omega), normalizer);
                let third = first;
                let fourth = mul_sse(mul_sse(cos_omega, negative_two), normalizer);
                let fifth = mul_sse(sub_sse(one, product), normalizer);
                [first, second, third, fourth, fifth]
            }
            4 | 5 => {
                let helper_input = mul_sse(mul_sse(reciprocal_rate, bounded_frequency), pi);
                let response = auxiliary_xmm(helper_input);
                let reciprocal = div_sse(one, response);
                let first = div_sse(one, add_sse(reciprocal, one));
                let cos_bounded = cosine(omega);
                let negative_first = f32::from_bits(first.to_bits() ^ 0x8000_0000);
                let cos_twice = mul_sse(cos_bounded, two);
                let scaled_first = mul_sse(first, reciprocal);
                let fourth = f32::from_bits(
                    mul_sse(cos_twice, scaled_first).to_bits() ^ 0x8000_0000,
                );
                let fifth = mul_sse(sub_sse(reciprocal, one), first);
                [first, 0.0, negative_first, fourth, fifth]
            }
            6 | 7 => {
                let helper_input = mul_sse(mul_sse(reciprocal_rate, bounded_frequency), pi);
                let response = auxiliary_xmm(helper_input);
                let first = div_sse(one, add_sse(response, one));
                let cos_bounded = cosine(omega);
                let second = f32::from_bits(
                    mul_sse(mul_sse(cos_bounded, two), first).to_bits() ^ 0x8000_0000,
                );
                let fourth = second;
                let fifth = mul_sse(sub_sse(one, response), first);
                [first, second, first, fourth, fifth]
            }
            8 | 9 | 10 => {
                let sin_omega = sine(omega);
                let relative_frequency = div_sse(frequency, bounded_frequency);
                let doubled_relative_frequency = mul_sse(relative_frequency, two);
                let response = div_sse(sin_omega, doubled_relative_frequency);
                let cos_bounded = cosine(omega);
                let damping_scaled = if kind == 10 {
                    mul_sse(damping, read_f32(0x00E7CD38))
                } else {
                    damping
                };
                let ratio = div_sse(response, damping_scaled);
                let product = mul_sse(damping_scaled, response);
                let normalizer = div_sse(one, add_sse(ratio, one));
                let first = mul_sse(add_sse(product, one), normalizer);
                let second_term = mul_sse(cos_bounded, negative_two);
                let second = mul_sse(second_term, normalizer);
                let third = mul_sse(sub_sse(one, product), normalizer);
                let fourth = second;
                let fifth = mul_sse(sub_sse(one, ratio), normalizer);
                [first, second, third, fourth, fifth]
            }
            _ => return,
        };
        write_coefficients(this, slot, coeff);
    }
});
