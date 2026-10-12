// original: 0x0059C7C0 CNetBlenderDummyPed::vf3

/// Refresh the dummy ped's sampled blend data and publish its clock and heading.
///
/// This thiscall method reads a three-float position through vtable slot `+0x30`,
/// lets slot `+0x34` fill a second local vector, and passes both vectors and the
/// first network clock sample to the blend helper at `this + 0x20`. It then
/// clears the byte flag at `+0xB4`, reads the network clock again, and stores
/// that word at `+0xC8`, `+0xD0`, and `+0xD8`. The heading comes from `this + 0xC0`:
/// a null nested sample at `+0x20` selects the fallback float at `+0x1C`; a
/// present sample converts its two floats to doubles, flips the first float's
/// sign using the image mask, and calls the double-register atan2 helper. The
/// resulting single-precision bits are stored at `+0xC4`, `+0xCC`, and `+0xD4`.
/// The method has no semantic return value.
lf_checker_rt::export!(thiscall, rw_0059c7c0(this: u32) -> () {
    const VTABLE_POINTER: u32 = 0x00;
    const POSITION_SLOT: u32 = 0x30;
    const SECOND_SAMPLE_SLOT: u32 = 0x34;
    const BLEND_STATE: u32 = 0x20;
    const READY_FLAG: u32 = 0xB4;
    const ENTITY_POINTER: u32 = 0xC0;
    const FALLBACK_HEADING: u32 = 0x1C;
    const NESTED_SAMPLE: u32 = 0x20;
    const SAMPLE_X: u32 = 0x10;
    const SAMPLE_Y: u32 = 0x14;
    const POSITION_X: u32 = 0x00;
    const POSITION_Y: u32 = 0x04;
    const POSITION_Z: u32 = 0x08;
    const CLOCK_CONTEXT: u32 = 0x01981FA8;
    const ANGLE_SIGN_MASK: u32 = 0x00FE8FA0;
    const FIRST_HEADING_STORE: u32 = 0xC4;
    const FIRST_CLOCK_STORE: u32 = 0xC8;
    const STORE_STRIDE: u32 = 0x08;

    unsafe {
        let object_vtable = (this.wrapping_add(VTABLE_POINTER) as *const u32).read_unaligned();
        let position_method = (object_vtable.wrapping_add(POSITION_SLOT) as *const u32).read_unaligned();
        let _ = core::hint::black_box(position_method);
        let position = lf_checker_rt::callee_thiscall!(1, u32, this);
        let position_sample = [
            (position.wrapping_add(POSITION_X) as *const u32).read_unaligned(),
            (position.wrapping_add(POSITION_Y) as *const u32).read_unaligned(),
            (position.wrapping_add(POSITION_Z) as *const u32).read_unaligned(),
        ];

        let second_method = (object_vtable.wrapping_add(SECOND_SAMPLE_SLOT) as *const u32).read_unaligned();
        let _ = core::hint::black_box(second_method);
        let mut second_sample = [0u32; 3];
        let second_sample_pointer = second_sample.as_mut_ptr() as u32;
        let _ = lf_checker_rt::callee_thiscall!(2, u32, this, second_sample_pointer);

        let clock_context = lf_checker_rt::relocated(CLOCK_CONTEXT);
        let first_clock = lf_checker_rt::callee_thiscall!(3, u32, clock_context);
        let position_pointer = position_sample.as_ptr() as u32;
        let second_pointer = second_sample.as_ptr() as u32;
        let _ = lf_checker_rt::callee_thiscall!(
            4,
            u32,
            this.wrapping_add(BLEND_STATE),
            position_pointer,
            second_pointer,
            first_clock
        );

        (this.wrapping_add(READY_FLAG) as *mut u8).write_unaligned(0);
        let second_clock = lf_checker_rt::callee_thiscall!(3, u32, clock_context);
        let entity = (this.wrapping_add(ENTITY_POINTER) as *const u32).read_unaligned();
        let nested_sample = (entity.wrapping_add(NESTED_SAMPLE) as *const u32).read_unaligned();

        let heading_bits = if nested_sample == 0 {
            (entity.wrapping_add(FALLBACK_HEADING) as *const u32).read_unaligned()
        } else {
            let x_bits = (nested_sample.wrapping_add(SAMPLE_X) as *const u32).read_unaligned();
            let y_bits = (nested_sample.wrapping_add(SAMPLE_Y) as *const u32).read_unaligned();
            let sign_mask = lf_checker_rt::global::<[u32; 4]>(ANGLE_SIGN_MASK).read_unaligned();
            let x_double_bits = (f32::from_bits(x_bits ^ sign_mask[0]) as f64).to_bits();
            let y_double_bits = (f32::from_bits(y_bits) as f64).to_bits();
            let angle_double_bits = lf_checker_rt::callee_cdecl!(
                5,
                u64,
                x_double_bits as u32,
                (x_double_bits >> 32) as u32,
                y_double_bits as u32,
                (y_double_bits >> 32) as u32
            );
            (f64::from_bits(angle_double_bits) as f32).to_bits()
        };

        (this.wrapping_add(FIRST_CLOCK_STORE) as *mut u32).write_unaligned(second_clock);
        (this.wrapping_add(FIRST_CLOCK_STORE + STORE_STRIDE) as *mut u32).write_unaligned(second_clock);
        (this.wrapping_add(FIRST_CLOCK_STORE + STORE_STRIDE * 2) as *mut u32).write_unaligned(second_clock);
        (this.wrapping_add(FIRST_HEADING_STORE) as *mut u32).write_unaligned(heading_bits);
        (this.wrapping_add(FIRST_HEADING_STORE + STORE_STRIDE) as *mut u32).write_unaligned(heading_bits);
        (this.wrapping_add(FIRST_HEADING_STORE + STORE_STRIDE * 2) as *mut u32).write_unaligned(heading_bits);
    }
});
