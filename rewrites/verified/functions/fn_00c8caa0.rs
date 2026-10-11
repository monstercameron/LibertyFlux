// original: 0x00C8CAA0 audio_position_blend

/// Copies the input position to the output, then applies a scripted planar
/// rotation for selector values zero and one and advances the vertical
/// component. The contract records the accepted lane values and call limits.
const RATE_LANE_ZERO: u32 = 0x00E9F138;
const RATE_LANE_ONE: u32 = 0x00FE8A18;
const VERTICAL_STEP: u32 = 0x00FE8990;

#[inline(always)]
unsafe fn read_word(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_word(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_float(address: u32) -> f32 {
    f32::from_bits(unsafe { read_word(address) })
}

#[inline(always)]
unsafe fn write_float(address: u32, value: f32) {
    unsafe { write_word(address, value.to_bits()) }
}

#[inline(always)]
unsafe fn global_double_bits(address: u32) -> u64 {
    let relocated = lf_checker_rt::relocated(address);
    unsafe { (relocated as *const u64).read_unaligned() }
}

#[inline(always)]
fn add(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

#[inline(always)]
fn sub(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) - core::hint::black_box(right)
}

#[inline(always)]
fn mul(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) * core::hint::black_box(right)
}

#[inline(always)]
fn double_call(id: u32, rate_bits: u64) -> f32 {
    let low = rate_bits as u32;
    let high = (rate_bits >> 32) as u32;
    let answer: u64 = lf_checker_rt::callee_cdecl!(id, u64, low, high);
    f64::from_bits(answer) as f32
}

unsafe fn run(flags_pointer: u32, direction_pointer: u32, source: u32, output: u32) {
    for offset in [0, 4, 8, 12] {
        unsafe { write_word(output + offset, read_word(source + offset)) };
    }

    let flags = unsafe { read_word(flags_pointer) };
    let lane = flags & 7;
    let mode = flags & 0x60;
    if !matches!(lane, 1 | 2 | 3 | 4 | 5) || mode == 0x40 {
        let z = unsafe { read_float(output + 8) };
        unsafe { write_float(output + 8, add(z, unsafe { read_float(lf_checker_rt::relocated(VERTICAL_STEP)) })) };
        return;
    }

    let accepted: u32 = lf_checker_rt::callee_cdecl!(1, u32, flags_pointer, direction_pointer);
    if accepted & 0xff == 0 {
        let z = unsafe { read_float(output + 8) };
        unsafe { write_float(output + 8, add(z, unsafe { read_float(lf_checker_rt::relocated(VERTICAL_STEP)) })) };
        return;
    }

    let selector = (flags >> 5) & 3;
    if selector > 1 || flags & 0x18 != 0x18 {
        let z = unsafe { read_float(output + 8) };
        unsafe { write_float(output + 8, add(z, unsafe { read_float(lf_checker_rt::relocated(VERTICAL_STEP)) })) };
        return;
    }

    let mut local = [0u32; 4];
    let local_pointer = local.as_mut_ptr() as u32;
    let _: u32 = lf_checker_rt::callee_stdcall!(2, u32, local_pointer, direction_pointer);
    let first = f32::from_bits(local[0]);
    let second = f32::from_bits(local[1]);
    let vertical = f32::from_bits(local[2]);

    let (first_rate, second_rate, first_id, second_id) = if selector == 0 {
        (RATE_LANE_ZERO, RATE_LANE_ZERO, 3, 4)
    } else {
        (RATE_LANE_ONE, RATE_LANE_ONE, 5, 6)
    };
    let first_bits = unsafe { global_double_bits(first_rate) };
    let second_bits = unsafe { global_double_bits(second_rate) };
    let first_answer = double_call(first_id, first_bits);
    let second_answer = double_call(second_id, second_bits);

    let x_delta = sub(mul(first, second_answer), mul(second, first_answer));
    let y_delta = if selector == 0 {
        add(mul(first, first_answer), mul(second, second_answer))
    } else {
        add(mul(second, second_answer), mul(first, first_answer))
    };

    let x = unsafe { read_float(output) };
    let y = unsafe { read_float(output + 4) };
    unsafe {
        write_float(output, add(x, x_delta));
        write_float(output + 4, add(y, y_delta));
        let z = read_float(output + 8);
        let step = read_float(lf_checker_rt::relocated(VERTICAL_STEP));
        write_float(output + 8, add(add(z, vertical), step));
    }
}

lf_checker_rt::export!(cdecl, rw_audio_position_blend(flags: u32, direction: u32, source: u32, output: u32) -> () {
    unsafe { run(flags, direction, source, output) }
});
