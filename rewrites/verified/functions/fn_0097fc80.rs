// original: 0x0097FC80 ped_task_add_float_progress

/// Scales an incoming progress value, keeps the larger of it and the task field,
/// and advances a second field by 0.4 up to 1.0.
///
/// The argument is one 32-bit float on the stack. The product is evaluated in
/// input-times-0.1 order. Ordered `>` comparisons reproduce the source's
/// unordered COMISS branches: a NaN product leaves the first field unchanged,
/// and a NaN second sum is stored as produced. The routine uses exact f32
/// arithmetic and pops its one stack word on return.
lf_checker_rt::export!(thiscall, rw_0097FC80(this: u32, input_bits: u32) -> () {
    const FIRST_PROGRESS: u32 = 0x40;
    const SECOND_PROGRESS: u32 = 0x144;
    const SCALE: f32 = f32::from_bits(0x3DCCCCCD);
    const LIMIT: f32 = f32::from_bits(0x3F800000);
    const STEP: f32 = f32::from_bits(0x3ECCCCCD);
    unsafe fn read_float(address: u32) -> f32 {
        f32::from_bits(unsafe { (address as *const u32).read_unaligned() })
    }
    unsafe fn write_float(address: u32, value: f32) {
        unsafe { (address as *mut u32).write_unaligned(value.to_bits()); }
    }
    let input = core::hint::black_box(f32::from_bits(input_bits));
    let scale = core::hint::black_box(SCALE);
    let mut scaled = input * scale;
    if scaled > LIMIT { scaled = LIMIT; }
    let first_address = this.wrapping_add(FIRST_PROGRESS);
    let current_first = core::hint::black_box(unsafe { read_float(first_address) });
    if !(scaled > current_first) { scaled = current_first; }
    unsafe { write_float(first_address, scaled); }
    let second_address = this.wrapping_add(SECOND_PROGRESS);
    let previous_second = core::hint::black_box(unsafe { read_float(second_address) });
    let step = core::hint::black_box(STEP);
    let advanced = previous_second + step;
    let next_second = if advanced > LIMIT { LIMIT } else { advanced };
    unsafe { write_float(second_address, next_second); }
});
