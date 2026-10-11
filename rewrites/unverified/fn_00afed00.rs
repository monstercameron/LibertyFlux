// original: 0x00AFED00 input_ui_object_gate
/// Checks a world object against its timer gates, visibility and distance
/// windows, then asks the placement helpers whether the object should be
/// activated. A successful placement sends the object to the action helper
/// and returns one; rejected or ineligible objects return zero.
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

const GAME_CLOCK: u32 = 0x0117_35B4;
const COOLDOWN_WITHOUT_SIDE: u32 = 0x0103_FF84;
const COOLDOWN_WITH_SIDE: u32 = 0x0103_FF80;
const RANGE_FLOOR: u32 = 0x0103_FF88;
const RANGE_FACTOR_NEAR: u32 = 0x0103_FF90;
const RANGE_FACTOR_ALT: u32 = 0x0103_FF8C;
const RANGE_FACTOR_SPECIAL: u32 = 0x0103_FF94;
const DISTANCE_SQUARED_FLOOR: u32 = 0x00FE_8C20;
const TYPE_TABLE: u32 = 0x0129_5CD8;
const EXCLUDED_MODEL_A: u32 = 0x012F_A308;
const EXCLUDED_MODEL_B: u32 = 0x012F_A47C;
const OBJECT_QUERY: u32 = 0x0190_8EF0;
const RANGE_MANAGER: u32 = 0x0128_E310;
const QUERY_SWITCH: u32 = 0x012B_4138;
const REFERENCE_RANGE_A: u32 = 0x0128_E328;
const REFERENCE_RANGE_B: u32 = 0x00E9_D14C;

#[inline(always)]
unsafe fn word(address: u32) -> u32 {
    (address as *const u32).read_unaligned()
}

#[inline(always)]
unsafe fn half(address: u32) -> i16 {
    (address as *const i16).read_unaligned()
}

#[inline(always)]
unsafe fn byte(address: u32) -> u8 {
    (address as *const u8).read()
}

#[inline(always)]
unsafe fn real(address: u32) -> f32 {
    (address as *const f32).read_unaligned()
}

#[inline(always)]
unsafe fn object_method0(stub: u32, object: u32) -> u32 {
    let call: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(stub as usize);
    call(object)
}

#[inline(always)]
unsafe fn child_method1(stub: u32, object: u32, argument: u32) -> u32 {
    let call: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(stub as usize);
    call(object, argument)
}

export!(cdecl, rw_afed00(object: u32, position: u32, scale: f32, side: u32) -> u32 {
    unsafe { object_gate(object, position, scale, side, false) }
});

// Wrong version for the proof contract: reverses the explicit cooldown exit.
export!(cdecl, mut_afed00(object: u32, position: u32, scale: f32, side: u32) -> u32 {
    unsafe { object_gate(object, position, scale, side, true) }
});

unsafe fn object_gate(object: u32, position: u32, scale: f32, side: u32, mutant: bool) -> u32 {
    if byte(object.wrapping_add(0xF17)) & 1 != 0 {
        let busy = callee_thiscall!(1, u32, object.wrapping_add(0xE48));
        if busy != 0 {
            return 0;
        }
    }

    let now = global::<u32>(GAME_CLOCK).read();
    if now < word(object.wrapping_add(0x129C)) {
        return 0;
    }

    let previous = word(object.wrapping_add(0x1294));
    let cooldown = if side as u8 == 0 {
        global::<u32>(COOLDOWN_WITHOUT_SIDE).read()
    } else {
        global::<u32>(COOLDOWN_WITH_SIDE).read()
    };
    let cooldown_end = previous.wrapping_add(cooldown);
    if now < cooldown_end {
        return if mutant { 1 } else { 0 };
    }

    if byte(object.wrapping_add(0xF14)) & 4 != 0 {
        return 0;
    }

    let vtable = word(object);
    let visible = object_method0(word(vtable.wrapping_add(0x34)), object);
    if visible as u8 == 0 {
        return 0;
    }

    let mut range_hit = 0u8;
    let mut query_value = 0u32;
    if word(object.wrapping_add(0x6C)) != 0 {
        let model = half(object.wrapping_add(0x2E)) as i32;
        let model_info = global::<u32>(TYPE_TABLE.wrapping_add((model as u32).wrapping_mul(4))).read();
        let model_range = word(model_info.wrapping_add(0x1C));
        let transform = word(object.wrapping_add(0x20));
        range_hit = callee_thiscall!(
            2,
            u32,
            relocated(OBJECT_QUERY),
            transform.wrapping_add(0x30),
            model_range,
            0x42C8_0000u32,
            0x41F0_0000u32,
            &mut query_value as *mut u32 as u32,
            1,
        ) as u8;
    }

    let transform = word(object.wrapping_add(0x20));
    let x_delta = core::hint::black_box(real(transform.wrapping_add(0x30)) - real(position));
    let y_delta = core::hint::black_box(real(transform.wrapping_add(0x34)) - real(position.wrapping_add(4)));
    let x_squared = core::hint::black_box(x_delta * x_delta);
    let y_squared = core::hint::black_box(y_delta * y_delta);
    let planar_squared = core::hint::black_box(y_squared + x_squared);
    let planar_distance = core::hint::black_box(planar_squared).sqrt();
    let local_gate = callee_thiscall!(3, u32, object, 0);

    let model = half(object.wrapping_add(0x2E)) as i32;
    let mut range_factor = global::<f32>(RANGE_FACTOR_NEAR).read();
    let special_policy = local_gate as u8 == 0
        && callee_thiscall!(4, u32, relocated(RANGE_MANAGER)) as u8 == 0
        && model != global::<u32>(EXCLUDED_MODEL_A).read() as i32
        && model != global::<u32>(EXCLUDED_MODEL_B).read() as i32
        && byte(object.wrapping_add(0xF16)) & 0x80 == 0
        && word(object.wrapping_add(0x1298)) <= now
        && byte(object.wrapping_add(0xF18)) & 8 == 0
        && (byte(object.wrapping_add(0x10B8)) != 3
            || callee_cdecl!(5, u32, model as u32, global::<u32>(QUERY_SWITCH).read()) as u8 != 0);
    if special_policy {
        range_factor = global::<f32>(RANGE_FACTOR_ALT).read();
    }
    if global::<u32>(REFERENCE_RANGE_A).read() < global::<u32>(REFERENCE_RANGE_B).read() {
        range_factor = global::<f32>(RANGE_FACTOR_ALT).read();
    }

    let authored_radius = half(object.wrapping_add(0x10C0)) as i32 as f32;
    let minimum_radius = global::<f32>(RANGE_FLOOR).read();
    let radius = if authored_radius <= minimum_radius { minimum_radius } else { authored_radius };
    let normalized_radius = core::hint::black_box(radius / minimum_radius);
    let scaled_radius = core::hint::black_box(normalized_radius * range_factor);
    let effective_radius = core::hint::black_box(scaled_radius * scale);

    if effective_radius < planar_distance {
        let mut transform_values = [0u32; 4];
        for index in 0..4u32 {
            transform_values[index as usize] = word(transform.wrapping_add(0x30 + index * 4));
        }
        let has_placement = callee_cdecl!(6, u32, transform_values.as_mut_ptr() as u32) as u8;
        if has_placement == 0 || callee_cdecl!(7, u32, object) as u8 == 0 {
            let _: u32 = callee_cdecl!(8, u32, object);
            return 1;
        }

        if range_hit != 0 {
            let child = word(object.wrapping_add(0x6C));
            let child_vtable = word(child);
            if child_method1(word(child_vtable.wrapping_add(0x100)), child, query_value) as u8 == 0 {
                return 0;
            }
        }
    }

    let class = word(object.wrapping_add(0x28));
    let time_since = now.wrapping_sub(word(object.wrapping_add(0xE38)));
    let recheck = class & 0x7C00 == 0x0C00
        && word(object.wrapping_add(0x12B8)) != 0
        && word(object.wrapping_add(0x12B8)).wrapping_add(60_000) < now
        && word(object.wrapping_add(0x1298)) < now
        && callee_thiscall!(3, u32, object, 0) as u8 == 0;
    let far_time = time_since > 10_000
        && word(object.wrapping_add(0xE40)) != 0
        && word(word(object.wrapping_add(0xE40)).wrapping_add(0xE40)) == object
        && callee_thiscall!(3, u32, object, 0) as u8 == 0;
    let secondary_time = word(object.wrapping_add(0xEE4)) > 0xAFC8
        && now > word(object.wrapping_add(0x1294)).wrapping_add(60_000)
        && now > word(object.wrapping_add(0x1298));

    if (recheck || far_time || secondary_time) && byte(object.wrapping_add(0xF14)) & 4 == 0 {
        let second_visible = object_method0(word(vtable.wrapping_add(0x34)), object) as u8 != 0;
        if second_visible {
            let dz = core::hint::black_box(real(transform.wrapping_add(0x38)) - real(position.wrapping_add(8)));
            let x3 = core::hint::black_box(x_delta * x_delta);
            let y3 = core::hint::black_box(y_delta * y_delta);
            let z3 = core::hint::black_box(dz * dz);
            let distance_squared = core::hint::black_box(core::hint::black_box(y3 + x3) + z3);
            let reach = global::<f32>(RANGE_FACTOR_SPECIAL).read();
            let limit = core::hint::black_box(core::hint::black_box(reach * scale) * core::hint::black_box(reach * scale));
            if distance_squared > limit {
                let mut transform_values = [0u32; 4];
                for index in 0..4u32 {
                    transform_values[index as usize] = word(transform.wrapping_add(0x30 + index * 4));
                }
                if callee_cdecl!(6, u32, transform_values.as_mut_ptr() as u32) as u8 == 0 {
                    let _: u32 = callee_cdecl!(8, u32, object);
                    return 1;
                }
            }
        }
    }

    0
}
