// original: 0x00BE75A0 CTaskSimpleSitDown::vf17
/// Updates the task's pose from its anchor and submits the interpolated point
/// to the attached entity. The transition flag selects a completion request;
/// the ordinary path advances the pose and passes the anchor's double lanes
/// to the vector helper. The return byte reports only the completion request.

use core::arch::x86::{
    __m128d, _mm_castsi128_pd, _mm_castsi128_ps, _mm_cvtps_pd, _mm_cvtss_sd,
    _mm_set_epi32,
};

const FLAGS: u32 = 0x0c;
const ANCHOR: u32 = 0x20;
const COMPLETION_REQUEST: u32 = 0x64;
const OBJECT: u32 = 0x60;
const COMPLETION_STATE: u32 = 0x74;
const READY_STATE: u32 = 0x75;
const ENTRY_FLOAT: u32 = 0x70;
const SCALE: u32 = 0x4c;
const CURRENT_POINT: u32 = 0x30;
const START_POINT: u32 = 0x20;
const POINT_DELTA: u32 = 0x40;
const METHOD_SLOT: u32 = 8;
const ANCHOR_VECTOR: u32 = 0x10;
const SIGN_MASK_FILE_VA: u32 = 0x00fe_8fa0;
const UPDATE_EVENT: u32 = 0x3505;
const POSITION_EVENT: u32 = 0x1505;

const CALLEE_UPDATE: u32 = 1;
const CALLEE_CREATE_OBJECT: u32 = 2;
const CALLEE_VECTOR: u32 = 4;
const CALLEE_FINALIZE: u32 = 5;

#[inline(always)]
fn load_u32(base: u32, offset: u32) -> u32 {
    unsafe { (base.wrapping_add(offset) as *const u32).read_unaligned() }
}

#[inline(always)]
fn store_u32(base: u32, offset: u32, value: u32) {
    unsafe { (base.wrapping_add(offset) as *mut u32).write_unaligned(value) }
}

#[inline(always)]
fn load_u8(base: u32, offset: u32) -> u8 {
    unsafe { (base.wrapping_add(offset) as *const u8).read() }
}

#[inline(always)]
fn load_f32(base: u32, offset: u32) -> f32 {
    f32::from_bits(load_u32(base, offset))
}

#[inline(always)]
fn add_ss(destination: f32, source: f32) -> f32 {
    core::hint::black_box(core::hint::black_box(destination) + core::hint::black_box(source))
}

#[inline(always)]
fn sub_ss(destination: f32, source: f32) -> f32 {
    core::hint::black_box(core::hint::black_box(destination) - core::hint::black_box(source))
}

#[inline(always)]
fn mul_ss(destination: f32, source: f32) -> f32 {
    core::hint::black_box(core::hint::black_box(destination) * core::hint::black_box(source))
}

#[inline(always)]
fn read_sign_mask_word(index: usize) -> u32 {
    unsafe { lf_checker_rt::global::<u32>(SIGN_MASK_FILE_VA).add(index).read_unaligned() }
}

#[inline(always)]
fn call_vector_helper(first: __m128d, second: __m128d) -> u64 {
    #[allow(improper_ctypes_definitions)]
    type VectorHelper = extern "C" fn(__m128d, __m128d) -> u64;
    let helper: VectorHelper = unsafe {
        core::mem::transmute(lf_checker_rt::callee_addr(CALLEE_VECTOR) as usize)
    };
    helper(first, second)
}

#[inline(always)]
fn call_position_method(object: u32, point: &[u32; 3]) {
    let vtable = load_u32(object, 0);
    let method_address = load_u32(vtable, METHOD_SLOT);
    let method: extern "thiscall" fn(u32, u32, u32, u32) -> u32 = unsafe {
        core::mem::transmute(method_address as usize)
    };
    let _ = method(object, point.as_ptr() as usize as u32, 0, 1);
}

fn rewrite(task: u32, object: u32, mutate_vector_high: bool) -> u32 {
    if load_u32(task, FLAGS) & 4 == 0 {
        let anchor = load_u32(object, ANCHOR);
        for field in [0x30, 0x34, 0x38, 0x3c] {
            store_u32(task, field, load_u32(anchor, field));
        }
    }

    if load_u8(task, COMPLETION_STATE) != 0 {
        let request = load_u32(task, COMPLETION_REQUEST);
        if request != 0 {
            let _ = lf_checker_rt::callee_thiscall!(
                CALLEE_UPDATE,
                u32,
                object,
                request,
                0,
                UPDATE_EVENT,
                task.wrapping_add(POINT_DELTA),
                load_u32(task, ENTRY_FLOAT),
                0,
            );
        }
        return 1;
    }

    let mut entity = load_u32(task, OBJECT);
    if entity == 0 {
        let _ = lf_checker_rt::callee_stdcall!(CALLEE_CREATE_OBJECT, u32, object);
        entity = load_u32(task, OBJECT);
    }
    if entity == 0 {
        return 0;
    }

    let request = load_u32(task, COMPLETION_REQUEST);
    if request != 0 {
        let scale = sub_ss(1.0, load_f32(entity, SCALE));
        let scaled_x = mul_ss(load_f32(task, 0x50), scale);
        let scaled_y = mul_ss(load_f32(task, 0x54), scale);
        let scaled_z = mul_ss(load_f32(task, 0x58), scale);
        let point = [
            add_ss(scaled_x, load_f32(task, 0x40)).to_bits(),
            add_ss(load_f32(task, 0x44), scaled_y).to_bits(),
            add_ss(load_f32(task, 0x48), scaled_z).to_bits(),
        ];
        let _ = lf_checker_rt::callee_thiscall!(
            CALLEE_UPDATE,
            u32,
            object,
            request,
            0,
            UPDATE_EVENT,
            point.as_ptr() as usize as u32,
            load_u32(task, ENTRY_FLOAT),
            0,
        );
        return 0;
    }

    if load_u8(task, READY_STATE) == 0 {
        return 0;
    }

    let scale = load_f32(entity, SCALE);
    let mut point = [0u32; 3];
    for index in 0..3 {
        let offset = (index as u32) * 4;
        let destination = load_f32(task, CURRENT_POINT + offset);
        let start = load_f32(task, START_POINT + offset);
        let delta = load_f32(task, POINT_DELTA + offset);
        let target = add_ss(start, delta);
        let difference = sub_ss(target, destination);
        let step = mul_ss(difference, scale);
        point[index] = add_ss(destination, step).to_bits();
    }

    call_position_method(object, &point);

    let anchor = load_u32(object, ANCHOR);
    let final_position = anchor.wrapping_add(CURRENT_POINT);
    let final_float = if anchor == 0 {
        load_f32(object, 0x1c)
    } else {
        let mask_words = [
            read_sign_mask_word(0),
            read_sign_mask_word(1),
            read_sign_mask_word(2),
            read_sign_mask_word(3),
        ];
        let raw_xmm0 = unsafe {
            _mm_set_epi32(
                (0 ^ mask_words[3]) as i32,
                (0 ^ mask_words[2]) as i32,
                (0 ^ mask_words[1]) as i32,
                (load_u32(anchor, ANCHOR_VECTOR) ^ mask_words[0]) as i32,
            )
        };
        let raw_xmm0_ps = unsafe { _mm_castsi128_ps(raw_xmm0) };
        let raw_xmm0_pd = unsafe { _mm_castsi128_pd(raw_xmm0) };
        let vector0 = unsafe { _mm_cvtss_sd(raw_xmm0_pd, raw_xmm0_ps) };

        let second_float = load_u32(anchor, ANCHOR_VECTOR + 4);
        let upper_float = if mutate_vector_high { 0x3f80_0000 } else { 0 };
        let raw_xmm1 = unsafe {
            _mm_set_epi32(0, 0, upper_float as i32, second_float as i32)
        };
        let raw_xmm1_ps = unsafe { _mm_castsi128_ps(raw_xmm1) };
        let vector1 = unsafe { _mm_cvtps_pd(raw_xmm1_ps) };

        let answer = call_vector_helper(vector0, vector1);
        f64::from_bits(answer) as f32
    };

    let _ = lf_checker_rt::callee_thiscall!(
        CALLEE_FINALIZE,
        u32,
        object,
        POSITION_EVENT,
        final_position,
        final_float.to_bits(),
        0,
    );
    0
}

lf_checker_rt::export!(thiscall, rw_fn_00be75a0(task: u32, object: u32) -> u32 {
    rewrite(task, object, false)
});

lf_checker_rt::export!(thiscall, mut_fn_00be75a0(task: u32, object: u32) -> u32 {
    rewrite(task, object, true)
});
