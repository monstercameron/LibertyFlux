// original: 0x00afed00 input_ui_object_gate
/// Check whether a candidate object is eligible relative to the supplied
/// position and time, consulting its virtual validity check and activity
/// gates. The accepted path calls the final object action and returns true;
/// rejected paths return false. This contract exercises the inactive child
/// pointer case and the distant, timed success path.
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

const CURRENT_TIME: u32 = 0x0117_35B4;
const DEFAULT_DELAY: u32 = 0x0103_FF80;
const MODE_DELAY: u32 = 0x0103_FF84;
const RADIUS_FLOOR: u32 = 0x0103_FF88;
const RADIUS_WIDE: u32 = 0x0103_FF90;
const RADIUS_NARROW: u32 = 0x0103_FF8C;
const RADIUS_MODE: u32 = 0x00E9_D14C;
const RADIUS_MODE_LIMIT: u32 = 0x0128_E328;
const MINIMUM_DISTANCE_SQ: u32 = 0x00FE_8C20;
const MODEL_BLOCK_A: u32 = 0x012F_A308;
const MODEL_BLOCK_B: u32 = 0x012F_A47C;
const SECONDARY_CONTEXT: u32 = 0x0128_E310;

#[inline(always)]
unsafe fn load_u8(address: u32) -> u8 {
    (address as *const u8).read_unaligned()
}

#[inline(always)]
unsafe fn load_u16(address: u32) -> u16 {
    (address as *const u16).read_unaligned()
}

#[inline(always)]
unsafe fn load_u32(address: u32) -> u32 {
    (address as *const u32).read_unaligned()
}

#[inline(always)]
unsafe fn load_f32(address: u32) -> f32 {
    f32::from_bits(load_u32(address))
}

export!(cdecl, rw_afed00(object: u32, player_position: u32, radius: f32, mode: u32) -> u32 {
    unsafe { body_afed00(object, player_position, radius, mode, false) }
});

// Mutant: reject a candidate when its virtual validity call succeeds.
export!(cdecl, mut_afed00(object: u32, player_position: u32, radius: f32, mode: u32) -> u32 {
    unsafe { body_afed00(object, player_position, radius, mode, true) }
});

unsafe fn body_afed00(
    object: u32,
    player_position: u32,
    radius: f32,
    mode: u32,
    mutant: bool,
) -> u32 {
    if load_u8(object.wrapping_add(0xF17)) & 1 != 0 {
        let attached = callee_thiscall!(5, u32, object.wrapping_add(0xE48));
        if attached != 0 {
            return 0;
        }
    }

    let current_time = global::<u32>(CURRENT_TIME).read();
    if current_time < load_u32(object.wrapping_add(0x129C)) {
        return 0;
    }
    let delay_address = if mode as u8 == 0 { DEFAULT_DELAY } else { MODE_DELAY };
    let deadline = load_u32(object.wrapping_add(0x1294))
        .wrapping_add(global::<u32>(delay_address).read());
    if deadline > current_time || load_u8(object.wrapping_add(0xF14)) & 4 != 0 {
        return 0;
    }

    let virtual_valid = callee_thiscall!(1, u32, object) as u8 != 0;
    if virtual_valid == mutant {
        return 0;
    }

    // The proof drives the branch where no auxiliary object is attached.
    if load_u32(object.wrapping_add(0x6C)) != 0 {
        return 0;
    }

    let matrix = load_u32(object.wrapping_add(0x20));
    let delta_y = core::hint::black_box(load_f32(matrix.wrapping_add(0x34)))
        - core::hint::black_box(load_f32(player_position.wrapping_add(4)));
    let delta_x = core::hint::black_box(load_f32(matrix.wrapping_add(0x30)))
        - core::hint::black_box(load_f32(player_position));
    let delta_y_sq = core::hint::black_box(delta_y) * core::hint::black_box(delta_y);
    let delta_x_sq = core::hint::black_box(delta_x) * core::hint::black_box(delta_x);
    let planar_distance_sq = core::hint::black_box(delta_y_sq) + core::hint::black_box(delta_x_sq);
    let _planar_distance = core::hint::black_box(planar_distance_sq.sqrt());

    let activity = callee_thiscall!(2, u32, object, 0);
    if activity != 0 {
        return 0;
    }
    let secondary_ready = callee_thiscall!(3, u32, relocated(SECONDARY_CONTEXT));
    if secondary_ready != 0 {
        return 0;
    }

    let model_id = load_u16(object.wrapping_add(0x2E)) as i16 as i32;
    if model_id == global::<i32>(MODEL_BLOCK_A).read()
        || model_id == global::<i32>(MODEL_BLOCK_B).read()
        || load_u8(object.wrapping_add(0xF16)) & 0x80 != 0
        || current_time < load_u32(object.wrapping_add(0x1298))
        || load_u8(object.wrapping_add(0xF18)) & 8 != 0
        || load_u8(object.wrapping_add(0x10B8)) == 3
    {
        return 0;
    }

    let radius_mode = global::<f32>(RADIUS_MODE).read();
    let radius_limit = global::<f32>(RADIUS_MODE_LIMIT).read();
    let scale = if radius_mode > radius_limit {
        global::<f32>(RADIUS_NARROW).read()
    } else {
        global::<f32>(RADIUS_WIDE).read()
    };
    let floor = global::<f32>(RADIUS_FLOOR).read();
    let mut magnitude = load_u16(object.wrapping_add(0x10C0)) as i16 as f32;
    if magnitude <= floor {
        magnitude = floor;
    }
    let ratio = core::hint::black_box(magnitude) / core::hint::black_box(floor);
    let scaled_radius = core::hint::black_box(ratio) * core::hint::black_box(scale);
    let threshold = core::hint::black_box(scaled_radius) * core::hint::black_box(radius);
    if 0.0f32 > threshold {
        return 0;
    }

    let state = load_u32(object.wrapping_add(0x1304));
    if state == 4 || state == 5 || load_u32(object.wrapping_add(0x1300)) == 2 {
        return 0;
    }
    let last_active = load_u32(object.wrapping_add(0x1298));
    if last_active != 0 && current_time <= last_active {
        return 0;
    }

    // The contract selects the distant path, then the elapsed-time shortcut.
    let delta_x_again = core::hint::black_box(load_f32(matrix.wrapping_add(0x30)))
        - core::hint::black_box(load_f32(player_position));
    let delta_y_again = core::hint::black_box(load_f32(matrix.wrapping_add(0x34)))
        - core::hint::black_box(load_f32(player_position.wrapping_add(4)));
    let xy_sq = core::hint::black_box(delta_y_again * delta_y_again)
        + core::hint::black_box(delta_x_again * delta_x_again);
    let minimum_distance_sq = global::<f32>(MINIMUM_DISTANCE_SQ).read();
    if xy_sq <= minimum_distance_sq {
        return 0;
    }

    if load_u8(object.wrapping_add(0xF14)) & 4 != 0
        || callee_thiscall!(1, u32, object) as u8 == 0
    {
        return 0;
    }
    let elapsed = current_time.wrapping_sub(load_u32(object.wrapping_add(0xE38)));
    if elapsed <= 0x2710 {
        return 0;
    }
    if load_u32(object.wrapping_add(0xE40)) != 0 {
        return 0;
    }
    if load_u32(object.wrapping_add(0xEE4)) > 0xAFC8 {
        return 0;
    }
    if current_time <= load_u32(object.wrapping_add(0x1294)).wrapping_add(0x4E20) {
        return 0;
    }

    let _ = callee_cdecl!(4, u32, object);
    1
}
