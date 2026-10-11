// original: 0x009EACE0 Ped_HasSpottedPedImpl
use lf_checker_rt::callee_cdecl;
use lf_checker_rt::export;

const TARGET_POSITION_PTR: u32 = 0x20;
const TARGET_POSITION_INLINE: u32 = 0x10;
const TARGET_FLAGS: u32 = 0x28;
const PED_POSITION_PTR: u32 = 0x20;
const PED_INTELLIGENCE_PTR: u32 = 0x224;
const POSITION_DATA: u32 = 0x30;
const ORIENTATION_X: u32 = 0x10;
const ORIENTATION_Y: u32 = 0x14;
const VIEW_RANGE_NEAR: u32 = 0xE0;
const VIEW_RANGE_FAR: u32 = 0xE4;
const QUERY_HEIGHT: u32 = 0x00FE_888C;
const QUERY_ORIGIN: u32 = 0x01B4_B320;
const QUERY_HELPER: u32 = 1;

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
fn sqrt(value: f32) -> f32 {
    unsafe {
        let input = core::arch::x86::_mm_set_ss(core::hint::black_box(value));
        core::arch::x86::_mm_cvtss_f32(core::arch::x86::_mm_sqrt_ss(input))
    }
}

#[inline(always)]
unsafe fn read_word(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn read_float(address: u32) -> f32 {
    f32::from_bits(unsafe { read_word(address) })
}

#[inline(always)]
unsafe fn write_float(address: u32, value: f32) {
    unsafe { (address as *mut u32).write_unaligned(value.to_bits()) }
}

#[inline(always)]
unsafe fn read_global_float(file_va: u32) -> f32 {
    unsafe { read_float(lf_checker_rt::relocated(file_va)) }
}

/// Tests whether a target is spotted and, when it passes the local view test,
/// asks the world query helper to confirm visibility. `ped` owns the position
/// pointer at `+0x20` and intelligence pointer at `+0x224`; `target` owns an
/// optional position pointer at `+0x20` and inline position at `+0x10`.
/// The local view test uses the XY displacement and the ped's XY orientation,
/// then compares planar distance against the larger of the two intelligence
/// range values. A query is built from the ped and target four-float positions;
/// the target height receives the query offset for the two flagged movement
/// modes. The cdecl query takes nine arguments and returns a nonzero result
/// when the target is not found. The original maps a zero helper result to AL=1
/// and a nonzero result to AL=0. The second stack argument is a byte-sized flag:
/// nonzero bypasses the local view/range rejection; its high bytes do not matter.
unsafe fn has_spotted_ped(ped: u32, target: u32, bypass_local_test: u32) -> u32 {
    let target_position_ptr = unsafe { read_word(target.wrapping_add(TARGET_POSITION_PTR)) };
    let target_source = if target_position_ptr == 0 {
        target.wrapping_add(TARGET_POSITION_INLINE)
    } else {
        target_position_ptr.wrapping_add(POSITION_DATA)
    };
    let ped_position_ptr = unsafe { read_word(ped.wrapping_add(PED_POSITION_PTR)) };
    let ped_source = ped_position_ptr.wrapping_add(POSITION_DATA);

    let target_x = unsafe { read_float(target_source) };
    let target_y = unsafe { read_float(target_source.wrapping_add(4)) };
    let target_z = unsafe { read_float(target_source.wrapping_add(8)) };
    let target_w = unsafe { read_float(target_source.wrapping_add(12)) };
    let ped_x = unsafe { read_float(ped_source) };
    let ped_y = unsafe { read_float(ped_source.wrapping_add(4)) };
    let ped_z = unsafe { read_float(ped_source.wrapping_add(8)) };
    let ped_w = unsafe { read_float(ped_source.wrapping_add(12)) };
    let delta_x = sub(target_x, ped_x);
    let delta_y = sub(target_y, ped_y);

    if bypass_local_test as u8 == 0 {
        let orientation_x = unsafe { read_float(ped_position_ptr.wrapping_add(ORIENTATION_X)) };
        let orientation_y = unsafe { read_float(ped_position_ptr.wrapping_add(ORIENTATION_Y)) };
        let forward_dot = add(mul(orientation_y, delta_y), mul(orientation_x, delta_x));
        if !(forward_dot >= 0.0) {
            return 0;
        }

        let intelligence = unsafe { read_word(ped.wrapping_add(PED_INTELLIGENCE_PTR)) };
        let near_limit = unsafe { read_float(intelligence.wrapping_add(VIEW_RANGE_NEAR)) };
        let far_limit = unsafe { read_float(intelligence.wrapping_add(VIEW_RANGE_FAR)) };
        let delta_y_squared = mul(delta_y, delta_y);
        let delta_x_squared = mul(delta_x, delta_x);
        let distance = sqrt(add(delta_y_squared, delta_x_squared));
        let selected_limit = if near_limit > far_limit { near_limit } else { far_limit };
        if !(selected_limit > distance) {
            return 0;
        }
    }

    let height_offset = unsafe { read_global_float(QUERY_HEIGHT) };
    let mut ped_query = [0u32; 4];
    ped_query[0] = ped_x.to_bits();
    ped_query[1] = ped_y.to_bits();
    ped_query[2] = add(ped_z, height_offset).to_bits();
    ped_query[3] = ped_w.to_bits();

    let movement_mode = (unsafe { read_word(target.wrapping_add(TARGET_FLAGS)) } >> 6) & 0xF;
    let query_target_z = if movement_mode == 3 || movement_mode == 6 {
        add(target_z, height_offset)
    } else {
        target_z
    };
    let mut target_query = [0u32; 4];
    target_query[0] = target_x.to_bits();
    target_query[1] = target_y.to_bits();
    target_query[2] = query_target_z.to_bits();
    target_query[3] = target_w.to_bits();

    let mut query_data = [0u32; 21];
    let origin_x = unsafe { read_global_float(QUERY_ORIGIN) };
    let origin_y = unsafe { read_global_float(QUERY_ORIGIN + 4) };
    let origin_z = unsafe { read_global_float(QUERY_ORIGIN + 8) };
    for offset in [4usize, 8, 12] {
        query_data[offset] = origin_x.to_bits();
        query_data[offset + 1] = origin_y.to_bits();
        query_data[offset + 2] = origin_z.to_bits();
    }
    query_data[19] = 0xFFFF;

    let answer = unsafe {
        let ped_query_ptr = ped_query.as_mut_ptr() as u32;
        let target_query_ptr = target_query.as_mut_ptr() as u32;
        let query_data_ptr = query_data.as_mut_ptr() as u32;
        callee_cdecl!(
            QUERY_HELPER,
            u32,
            ped_query_ptr,
            target_query_ptr,
            0,
            query_data_ptr,
            0x86,
            1,
            0,
            2,
            4,
        )
    };
    u32::from(answer == 0)
}

export!(thiscall, rw_009eace0(ped: u32, target: u32, bypass_local_test: u32) -> u32 {
    unsafe { has_spotted_ped(ped, target, bypass_local_test) }
});

