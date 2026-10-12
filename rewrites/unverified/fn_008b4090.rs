// original: 0x008B4090 stick_response_apply
/// Convert each of two response channels into a scaled double helper call,
/// store the helper results, copy the associated integer and byte fields, and
/// advance the object's three-position counter.
///
/// A channel below the image's lower bound stores zero without a helper call.
/// Otherwise its value is scaled before promotion to a double. The shared
/// double helper receives the image's ten-valued argument and the promoted
/// channel value through XMM0 and XMM1. A changed response code triggers a
/// scripted thiscall only when the copied flag byte is clear.
///
/// Original: `0x008B4090` (thiscall, response pointer on the stack).
lf_checker_rt::export!(thiscall, rw_008b4090(this: u32, response: u32) -> () {
    unsafe {
        let object = this as *mut u8;
        let input = response as *const u8;
        let ten = lf_checker_rt::global::<f64>(0x00FE_8A78).read_unaligned();
        let scale = lf_checker_rt::global::<f32>(0x00FE_876C).read_unaligned();
        let lower_bound = lf_checker_rt::global::<f32>(0x00FE_8DF8).read_unaligned();

        let first = (input as *const f32).read_unaligned();
        let first_delta = sub32(first, lower_bound);
        let first_result = if jb32(first_delta, 0.0) {
            0.0
        } else {
            let scaled = mul32(first, scale);
            let answer = call_curve(ten, scaled);
            core::hint::black_box(answer) as f32
        };
        (object.add(0x0c) as *mut f32).write_unaligned(first_result);

        let second = (input.add(4) as *const f32).read_unaligned();
        let second_delta = sub32(second, lower_bound);
        let second_result = if jb32(second_delta, 0.0) {
            0.0
        } else {
            let scaled = mul32(second, scale);
            let answer = call_curve(ten, scaled);
            core::hint::black_box(answer) as f32
        };
        (object.add(0x10) as *mut f32).write_unaligned(second_result);

        let input_word = (input.add(8) as *const u32).read_unaligned();
        (object.add(0x14) as *mut u32).write_unaligned(input_word);
        let copied_flag = input.add(0x11).read();
        object.add(0x1c).write(copied_flag);
        let update_flag = input.add(0x10).read();
        object.add(0x20).write(update_flag);
        let response_code = (input.add(0x0c) as *const u32).read_unaligned();
        let old_code = (object.add(0x18) as *const u32).read_unaligned();
        if response_code != old_code {
            (object.add(0x18) as *mut u32).write_unaligned(response_code);
            if update_flag == 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, this);
            }
        }

        let counter_ptr = object.add(8) as *mut u32;
        let next = counter_ptr.read_unaligned().wrapping_add(1);
        counter_ptr.write_unaligned(next % 3);
    }
});

#[inline(never)]
fn call_curve(ten: f64, value: f32) -> f64 {
    let ten_bits = ten.to_bits();
    let value_bits = f64::from(value).to_bits();
    let bits: u64 = lf_checker_rt::callee_cdecl!(
        1,
        u64,
        ten_bits as u32,
        (ten_bits >> 32) as u32,
        value_bits as u32,
        (value_bits >> 32) as u32,
    );
    f64::from_bits(bits)
}

#[inline(never)]
fn sub32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) - core::hint::black_box(right)
}

#[inline(never)]
fn mul32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) * core::hint::black_box(right)
}

#[inline(always)]
fn jb32(left: f32, right: f32) -> bool {
    left.is_nan() || right.is_nan() || left < right
}
