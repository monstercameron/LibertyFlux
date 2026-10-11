#![allow(unsafe_code)]

// original: 0x0097BEF0 audio_voice_update

/// Updates the voice state flags and float fields using scripted random
/// decisions, a state-dependent blend, and the final duration helper.
/// The nested voice state controls whether the early state reset runs; the
/// routine always refreshes the blend and duration fields before returning.
const VOICE_STATE_PTR: u32 = 0x120;
const VOICE_GATE_BYTE: u32 = 0x219;
const VOICE_MODE: u32 = 0x78;
const VOICE_FLAGS: u32 = 0x128;
const VOICE_PUBLIC_FLAGS: u32 = 0x74;
const VOICE_BLEND: u32 = 0xC0;
const VOICE_BLEND_SEED: u32 = 0xBC;
const VOICE_DURATION: u32 = 0x12C;
const VOICE_RATE_DOUBLE: u32 = 0x00FE8A28;
const VOICE_RATE_SCALE: u32 = 0x00E78574;
const RANDOM_LOW: u32 = 0xFFFFFDA8;
const RANDOM_HIGH: u32 = 0x190;
const MODE_LIMIT: i32 = 6;
const MODE_FALLBACK: u32 = 2;
const THRESHOLD_INITIAL: u32 = 0x3ECCCCCD;
const THRESHOLD_ROUTE: u32 = 0x3F000000;
const THRESHOLD_BRANCH: u32 = 0x3DCCCCCD;
const THRESHOLD_PUBLIC: u32 = 0x3E99999A;
const BLEND_SEED_X: u32 = 0xC0800000;
const BLEND_SEED_Y: u32 = 0;
const DURATION_X: u32 = 0;
const DURATION_Y: u32 = 0x3F800000;
const FLAG_ROUTE_LEFT: u32 = 0x1;
const FLAG_ROUTE_RIGHT: u32 = 0x2;
const FLAG_EVENT_LEFT: u32 = 0x8;
const FLAG_EVENT_RIGHT: u32 = 0x4;

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_u8(address: u32) -> u8 {
    unsafe { (address as *const u8).read() }
}

#[inline(always)]
unsafe fn write_f32(address: u32, value: f32) {
    unsafe { (address as *mut f32).write_unaligned(value) }
}

unsafe fn audio_voice_update_impl(this: u32, wrong_duration: bool) {
    let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
    let voice_state = unsafe { read_u32(this + VOICE_STATE_PTR) };
    let disabled = unsafe { read_u8(voice_state + VOICE_GATE_BYTE) } != 0;
    if disabled {
        unsafe { write_u32(this + 0xBC, 0) };
    } else {
        let mode = unsafe { read_u32(this + VOICE_MODE) };
        if mode == 0 {
            let draw: u8 = lf_checker_rt::callee_cdecl!(2, u8, THRESHOLD_INITIAL);
            unsafe { write_u32(this + VOICE_MODE, if draw == 0 { MODE_FALLBACK } else { 1 }) };
        } else if (mode as i32) > MODE_LIMIT {
            unsafe { write_u32(this + VOICE_MODE, MODE_FALLBACK) };
        }

        let seed: f32 = lf_checker_rt::callee_cdecl!(
            3,
            f32,
            BLEND_SEED_X,
            BLEND_SEED_Y,
        );
        unsafe { write_f32(this + VOICE_BLEND_SEED, seed) };

        let active: u8 = lf_checker_rt::callee_thiscall!(4, u8, this);
        if active != 0 {
            let left: u8 = lf_checker_rt::callee_cdecl!(2, u8, THRESHOLD_ROUTE);
            let branch_a: u8 = lf_checker_rt::callee_cdecl!(2, u8, THRESHOLD_BRANCH);
            let branch_b: u8 = lf_checker_rt::callee_cdecl!(2, u8, THRESHOLD_BRANCH);
            let old_flags = unsafe { read_u32(this + VOICE_FLAGS) };

            if left != 0 {
                let routed = if branch_a != 0 { FLAG_ROUTE_LEFT } else { 0 };
                unsafe { write_u32(this + VOICE_FLAGS, old_flags | routed) };
                if branch_b != 0 {
                    let flags = unsafe { read_u32(this + VOICE_FLAGS) };
                    unsafe { write_u32(this + VOICE_FLAGS, flags | FLAG_EVENT_LEFT) };
                }
            } else {
                let routed = if branch_a != 0 { FLAG_ROUTE_RIGHT } else { 0 };
                unsafe { write_u32(this + VOICE_FLAGS, old_flags | routed) };
                if branch_b != 0 {
                    let flags = unsafe { read_u32(this + VOICE_FLAGS) };
                    unsafe { write_u32(this + VOICE_FLAGS, flags | FLAG_EVENT_RIGHT) };
                }
            }
        }
    }

    let random_value: u32 = lf_checker_rt::callee_cdecl!(
        5,
        u32,
        RANDOM_LOW,
        RANDOM_HIGH,
    );
    let random_f32 = (random_value as i32) as f32;
    let rate_scale = unsafe {
        f32::from_bits(read_u32(lf_checker_rt::relocated(VOICE_RATE_SCALE)))
    };
    let scaled_rate = core::hint::black_box(random_f32)
        * core::hint::black_box(rate_scale);
    let rate = f64::from(scaled_rate);
    let base = unsafe {
        f64::from_bits(read_u32(lf_checker_rt::relocated(VOICE_RATE_DOUBLE)) as u64
            | ((read_u32(lf_checker_rt::relocated(VOICE_RATE_DOUBLE + 4)) as u64) << 32))
    };
    let base_bits = base.to_bits();
    let rate_bits = rate.to_bits();
    let blend_bits: u64 = lf_checker_rt::callee_cdecl!(
        6,
        u64,
        base_bits as u32,
        (base_bits >> 32) as u32,
        rate_bits as u32,
        (rate_bits >> 32) as u32,
    );
    let blend = core::hint::black_box(f64::from_bits(blend_bits)) as f32;
    unsafe { write_f32(this + VOICE_BLEND, blend) };

    if !disabled && unsafe { read_u32(this + VOICE_MODE) } == MODE_FALLBACK {
        let low: u8 = lf_checker_rt::callee_cdecl!(2, u8, THRESHOLD_PUBLIC);
        let high: u8 = lf_checker_rt::callee_cdecl!(2, u8, THRESHOLD_PUBLIC);
        let prior = unsafe { read_u32(this + VOICE_PUBLIC_FLAGS) };
        unsafe {
            write_u32(this + VOICE_PUBLIC_FLAGS, prior | u32::from(low));
            write_u32(this + VOICE_PUBLIC_FLAGS, prior | u32::from(low) | (u32::from(high) << 1));
        }
    }

    let duration: f32 = lf_checker_rt::callee_cdecl!(3, f32, DURATION_X, DURATION_Y);
    unsafe {
        write_f32(
            this + VOICE_DURATION,
            if wrong_duration { 0.0 } else { duration },
        )
    };
}

lf_checker_rt::export!(thiscall, rw_audio_voice_update(this: u32) -> () {
    unsafe { audio_voice_update_impl(this, false) }
});

lf_checker_rt::export!(thiscall, mut_audio_voice_update(this: u32) -> () {
    unsafe { audio_voice_update_impl(this, true) }
});
