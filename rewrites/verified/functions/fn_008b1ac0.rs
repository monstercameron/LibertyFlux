// original: 0x008B1AC0 biquad parameter apply
/// Applies one parameter block to a biquad filter effect.
///
/// Copies the mode and auxiliary value, caps the input level against the audio
/// limit, derives two coefficients through the double precision power helper,
/// stores the stage and bypass settings, and calls the coefficient updater.
/// When bypass is enabled it clears the four state arrays using the object's
/// active-channel count. It then advances the three-way coefficient selector,
/// stores the new selector, and returns the quotient left by that calculation.
/// The coefficient updater and memory-clear helper are scripted contract callees.

const POW_ID: u32 = 0;
const COEFFICIENT_UPDATE_ID: u32 = 2;
const MEMORY_CLEAR_ID: u32 = 3;
const POW_BASE_VA: u32 = 0x00fe8a78;
const LEVEL_LIMIT_VA: u32 = 0x01030868;
const LEVEL_OFFSET_VA: u32 = 0x00e7cd3c;
const LEVEL_SCALE_VA: u32 = 0x00fe876c;

#[inline(always)]
fn fsub(lhs: f32, rhs: f32) -> f32 {
    core::hint::black_box(lhs) - core::hint::black_box(rhs)
}

#[inline(always)]
fn fmul(lhs: f32, rhs: f32) -> f32 {
    core::hint::black_box(lhs) * core::hint::black_box(rhs)
}

#[inline(always)]
fn read_global_f32(file_va: u32) -> f32 {
    unsafe { f32::from_bits(lf_checker_rt::global::<u32>(file_va).read_unaligned()) }
}

#[inline(always)]
fn read_global_f64_bits(file_va: u32) -> u64 {
    unsafe { lf_checker_rt::global::<u64>(file_va).read_unaligned() }
}

#[inline(always)]
fn call_pow10(exponent: f32) -> f32 {
    let base_bits = read_global_f64_bits(POW_BASE_VA);
    let exponent_bits = (exponent as f64).to_bits();
    let result_bits: u64 = lf_checker_rt::callee_cdecl!(
        POW_ID,
        u64,
        base_bits as u32,
        (base_bits >> 32) as u32,
        exponent_bits as u32,
        (exponent_bits >> 32) as u32
    );
    f64::from_bits(result_bits) as f32
}

#[inline(always)]
unsafe fn read32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read8(address: u32) -> u8 {
    unsafe { (address as *const u8).read() }
}

#[inline(always)]
unsafe fn write8(address: u32, value: u8) {
    unsafe { (address as *mut u8).write(value) }
}

unsafe fn apply_parameters(this: u32, parameters: u32, mutant: bool) -> u32 {
    unsafe {
        let mode = read32(parameters + 0x10);
        write32(this + 0xbc, mode);

        let input_level = f32::from_bits(read32(parameters));
        let limit = read_global_f32(LEVEL_LIMIT_VA);
        let capped_level = if limit > input_level { input_level } else { limit };
        write32(this + 0xa8, capped_level.to_bits());

        write8(
            this + 0xc3,
            matches!(mode, 1 | 3 | 5 | 7 | 9 | 10) as u8,
        );

        let first_delta = fsub(
            f32::from_bits(read32(parameters + 4)),
            read_global_f32(LEVEL_OFFSET_VA),
        );
        let first_exponent = fmul(first_delta, read_global_f32(LEVEL_SCALE_VA));
        write32(this + 0xac, call_pow10(first_exponent).to_bits());

        write32(this + 0xb0, read32(parameters + 8));

        let second_exponent = fmul(
            f32::from_bits(read32(parameters + 0x0c)),
            read_global_f32(LEVEL_SCALE_VA),
        );
        write32(this + 0xb4, call_pow10(second_exponent).to_bits());
        write8(this + 0xb8, read8(parameters + 0x14));

        let _: u32 = lf_checker_rt::callee_thiscall!(COEFFICIENT_UPDATE_ID, u32, this);

        if read8(this + 0xb8) != 0 {
            let byte_count = (read8(this + 0xc1) as u32) << 2;
            for offset in [0x48, 0x60, 0x78, 0x90] {
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    MEMORY_CLEAR_ID,
                    u32,
                    this + offset,
                    0,
                    byte_count
                );
            }
        }

        let next_value = read32(this + 8).wrapping_add(1);
        let quotient = next_value / 3;
        let remainder = next_value % 3;
        write32(this + 8, if mutant { next_value / 3 } else { remainder });
        quotient
    }
}

lf_checker_rt::export!(thiscall, rw_008B1AC0(this: u32, parameters: u32) -> u32 {
    unsafe { apply_parameters(this, parameters, false) }
});

lf_checker_rt::export!(thiscall, mut_008B1AC0(this: u32, parameters: u32) -> u32 {
    unsafe { apply_parameters(this, parameters, true) }
});
