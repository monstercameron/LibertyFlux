// original: 0x0099ce50 audio_envelope_update
/// Update the audio envelope for one entity and submit its final level.
///
/// The entity's timing fields select a scaled double-precision response when
/// the owning audio state is active. An envelope helper supplies the current
/// level, a mixer helper receives the combined level, and a voice-update
/// helper may write the active voice pointer into the object. A missing voice
/// exits with that helper's status. Otherwise the function selects the runtime
/// mixer row, treating an empty tag as row zero, and returns the row helper's
/// result.
lf_checker_rt::export!(thiscall, rw_0099ce50(object: u32, entity: u32) -> u32 {
    const INNER_STATE_OFFSET: u32 = 0x9e0;
    const VOICE_SLOT_OFFSET: u32 = 0x864;
    const VOICE_ARGUMENT_OFFSET: u32 = 0xa20;
    const ENTITY_TAG_OFFSET: u32 = 4;
    const ENTITY_ROW_OFFSET: u32 = 0x40;
    const ROW_STRIDE: u32 = 0x6f40;
    const ROW_VALUE_OFFSET: u32 = 0x6f14;
    const EMPTY_TAG: u8 = 0xff;

    #[inline(always)]
    fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read_unaligned() }
    }

    #[inline(always)]
    fn read_f32(address: u32) -> f32 {
        f32::from_bits(read_u32(address))
    }

    #[inline(always)]
    fn read_f64(address: u32) -> f64 {
        unsafe { (address as *const f64).read_unaligned() }
    }

    #[inline(always)]
    fn add_in_order(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }

    #[inline(always)]
    fn sub_in_order(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    }

    #[inline(always)]
    fn mul_in_order(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }

    let inner = read_u32(object.wrapping_add(INNER_STATE_OFFSET));
    let active = read_u32(inner.wrapping_add(0xce)) != 0;
    let mut final_level = 0.0f32;
    if active {
        let mut response = 0.0f32;
        let elapsed = add_in_order(
            read_f32(entity.wrapping_add(0x58)),
            read_f32(entity.wrapping_add(0x28)),
        );
        let relative = sub_in_order(
            elapsed,
            read_f32(lf_checker_rt::relocated(0x00fe8df8)),
        );
        if relative >= 0.0 {
            let scaled = mul_in_order(
                elapsed,
                read_f32(lf_checker_rt::relocated(0x00fe876c)),
            );
            let base_bits = read_f64(lf_checker_rt::relocated(0x00fe8a78)).to_bits();
            let exponent_bits = f64::from(scaled).to_bits();
            let power_bits: u64 = __lf_callee_cdecl!(
                1,
                u64,
                base_bits as u32,
                (base_bits >> 32) as u32,
                exponent_bits as u32,
                (exponent_bits >> 32) as u32,
            );
            response = f64::from_bits(power_bits) as f32;
        }

        let envelope = __lf_callee_thiscall!(
            2,
            f32,
            lf_checker_rt::relocated(0x01283ee0),
            read_u32(entity.wrapping_add(0x64)),
        );
        let envelope_gain = mul_in_order(envelope, read_f32(entity.wrapping_add(0x60)));
        final_level = mul_in_order(envelope_gain, response);
    }
    let mixer_result = __lf_callee_thiscall!(
        3,
        f32,
        object.wrapping_add(0x4dc),
        final_level.to_bits(),
        read_u32(entity),
    );

    let volume = mul_in_order(
        read_f32(entity.wrapping_add(0x50)),
        read_f32(entity.wrapping_add(0x14)),
    );
    let voice_status = __lf_callee_thiscall!(
        4,
        u32,
        object,
        object.wrapping_add(VOICE_SLOT_OFFSET),
        read_u32(inner.wrapping_add(0xce)),
        mixer_result.to_bits(),
        volume.to_bits(),
        read_u32(object.wrapping_add(VOICE_ARGUMENT_OFFSET)),
        read_u32(lf_checker_rt::relocated(0x0128347c)),
        0,
        1,
        0x3f80_0000,
    );

    let voice = read_u32(object.wrapping_add(VOICE_SLOT_OFFSET));
    if voice == 0 {
        return voice_status;
    }

    let row = if read_u8(voice.wrapping_add(ENTITY_TAG_OFFSET)) == EMPTY_TAG {
        0
    } else {
        let tag = u32::from(read_u8(voice.wrapping_add(ENTITY_TAG_OFFSET)));
        let row_index = u32::from(read_u8(voice.wrapping_add(ENTITY_ROW_OFFSET)));
        let stride = read_u32(lf_checker_rt::relocated(0x0115d968));
        let table = read_u32(lf_checker_rt::relocated(0x0115d988));
        let cell = read_u32(
            table
                .wrapping_add(row_index.wrapping_mul(ROW_STRIDE))
                .wrapping_add(ROW_VALUE_OFFSET),
        );
        stride.wrapping_mul(tag).wrapping_add(cell)
    };

    __lf_callee_thiscall!(5, u32, row, 0)
});
