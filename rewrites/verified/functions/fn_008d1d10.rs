// original: 0x008D1D10 PlayerTargetCheckNear
/// Tests whether a target is in the player's forward region and within the
/// configured per-axis distance. It returns one in AL on every path. The
/// routine may call the target classifier after its geometric gates pass.
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

const C_POSITION_OUTPUT: u32 = 1;
const C_HEIGHT: u32 = 2;
const C_VECTOR_DATA: u32 = 3;
const C_CLASSIFY_TARGET: u32 = 4;

const VA_TARGET_LIMITS: u32 = 0x1295_CD8;
const VA_MAX_DISTANCE: u32 = 0x0103_2360;
const VA_DOT_SCALE: u32 = 0x00FE_8628;
const VA_DOT_BIAS: u32 = 0x00FE_88E8;
const VA_ANGLE_LIMIT: u32 = 0x00FE_8A24;
const VA_ABS_MASK: u32 = 0x00FE_8F80;

#[inline(always)]
unsafe fn read_word(address: u32, offset: u32) -> u32 {
    unsafe { (address.wrapping_add(offset) as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn read_byte(address: u32, offset: u32) -> u8 {
    unsafe { (address.wrapping_add(offset) as *const u8).read() }
}

#[inline(always)]
unsafe fn read_half(address: u32, offset: u32) -> u16 {
    unsafe { (address.wrapping_add(offset) as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn read_float(address: u32, offset: u32) -> f32 {
    f32::from_bits(unsafe { read_word(address, offset) })
}

#[inline(always)]
unsafe fn relocated_float(file_va: u32) -> f32 {
    f32::from_bits(unsafe { global::<u32>(file_va).read_unaligned() })
}

#[inline(always)]
fn add_sse(left: f32, right: f32) -> f32 {
    core::hint::black_box(core::hint::black_box(left) + core::hint::black_box(right))
}

#[inline(always)]
fn sub_sse(left: f32, right: f32) -> f32 {
    core::hint::black_box(core::hint::black_box(left) - core::hint::black_box(right))
}

#[inline(always)]
fn mul_sse(left: f32, right: f32) -> f32 {
    core::hint::black_box(core::hint::black_box(left) * core::hint::black_box(right))
}

#[inline(always)]
unsafe fn abs_sse(value: f32) -> f32 {
    let mask = unsafe { global::<u32>(VA_ABS_MASK).read_unaligned() };
    f32::from_bits(value.to_bits() & mask)
}

#[inline(always)]
fn unordered(left: f32, right: f32) -> bool {
    left.is_nan() || right.is_nan()
}

#[inline(always)]
fn comiss_jbe(left: f32, right: f32) -> bool {
    unordered(left, right) || left <= right
}

#[inline(always)]
fn comiss_ja(left: f32, right: f32) -> bool {
    !unordered(left, right) && left > right
}

#[inline(always)]
fn comiss_jb(left: f32, right: f32) -> bool {
    unordered(left, right) || left < right
}

#[inline(always)]
fn point_base(player: u32) -> u32 {
    let linked = unsafe { read_word(player, 0x20) };
    if linked == 0 {
        player.wrapping_add(0x10)
    } else {
        linked.wrapping_add(0x30)
    }
}

#[inline(always)]
fn vector_dot(matrix: u32, offset: u32, x: f32, y: f32, z: f32) -> f32 {
    unsafe {
        let y_part = mul_sse(read_float(matrix, offset.wrapping_add(4)), y);
        let x_part = mul_sse(x, read_float(matrix, offset));
        let xy = add_sse(y_part, x_part);
        let z_part = mul_sse(z, read_float(matrix, offset.wrapping_add(8)));
        add_sse(xy, z_part)
    }
}

export!(cdecl, rw_008d1d10(player: u32, target: u32) -> u32 {
    unsafe {
        if read_byte(player, 0x24) & 1 == 0
            || read_word(player, 0x28) & 0x3C0 != 0x80
        {
            return 1;
        }

        let mode = read_word(player, 0x1304);
        if mode == 3 {
            let target_object = read_word(target, 0);
            let target_coords = read_word(target_object, 0x20);
            let player_axes = read_word(player, 0x20);
            let dx = sub_sse(read_float(target_coords, 0x34), read_float(player_axes, 0x34));
            let dy = sub_sse(read_float(target_coords, 0x30), read_float(player_axes, 0x30));
            let dz = sub_sse(read_float(target_coords, 0x38), read_float(player_axes, 0x38));
            let mut projection = mul_sse(read_float(player_axes, 4), dx);

            if read_byte(player, 0x14E5) & 0x10 != 0 {
                let x_projection = mul_sse(dy, read_float(player_axes, 0));
                let z_projection = mul_sse(read_float(player_axes, 8), dz);
                projection = add_sse(projection, x_projection);
                projection = add_sse(projection, z_projection);
                if !comiss_jbe(0.0, projection) {
                    return 1;
                }
            } else {
                let x_projection = mul_sse(read_float(player_axes, 0), dy);
                projection = add_sse(projection, x_projection);
                let z_projection = mul_sse(read_float(player_axes, 8), dz);
                projection = add_sse(projection, z_projection);
                if comiss_ja(projection, 0.0) {
                    return 1;
                }
            }
        }

        let mut unused_output = [0_u32; 4];
        let output_pointer = unused_output.as_mut_ptr() as u32;
        let _ = callee_thiscall!(C_POSITION_OUTPUT, u32, player, output_pointer);

        let player_position = point_base(player);
        let player_x = read_float(player_position, 0);
        let player_y = read_float(player_position, 4);
        let player_z = read_float(player_position, 8);
        let height = callee_thiscall!(C_HEIGHT, f32, player);
        let adjusted_z = add_sse(sub_sse(player_z, height), relocated_float(VA_DOT_BIAS));

        let target_object = read_word(target, 0);
        let target_coords = read_word(target_object, 0x20);
        let dx = sub_sse(read_float(target_coords, 0x30), player_x);
        let dy = sub_sse(read_float(target_coords, 0x34), player_y);
        let dz = sub_sse(read_float(target_coords, 0x38), adjusted_z);

        let horizontal = add_sse(dy, dx);
        let scaled_horizontal = mul_sse(horizontal, relocated_float(VA_DOT_SCALE));
        let bearing = add_sse(scaled_horizontal, dz);
        let bearing = abs_sse(bearing);
        if comiss_jbe(relocated_float(VA_ANGLE_LIMIT), bearing) {
            return 1;
        }
        if read_byte(player, 0xF1A) & 0x80 == 0 {
            return 1;
        }

        let index = read_half(player, 0x2E) as i16 as i32;
        let table_address = VA_TARGET_LIMITS.wrapping_add((index.wrapping_mul(4)) as u32);
        let limits = global::<u32>(table_address).read_unaligned();

        let mut dx = sub_sse(read_float(target_coords, 0x30), player_x);
        let mut dy = sub_sse(read_float(target_coords, 0x34), player_y);
        let mut dz = sub_sse(read_float(target_coords, 0x38), adjusted_z);

        let matrix0 = callee_thiscall!(C_VECTOR_DATA, u32, player);
        let mut x_limit = sub_sse(abs_sse(vector_dot(matrix0, 0, dx, dy, dz)), read_float(limits, 0x30));
        if x_limit < 0.0 {
            x_limit = 0.0;
        }

        let target_axes = read_word(target_object, 0x20);
        dx = sub_sse(read_float(target_axes, 0x30), read_float(player_position, 0));
        dy = sub_sse(read_float(target_axes, 0x34), read_float(player_position, 4));
        dz = sub_sse(read_float(target_axes, 0x38), read_float(player_position, 8));
        let matrix1 = callee_thiscall!(C_VECTOR_DATA, u32, player);
        let mut y_limit = sub_sse(abs_sse(vector_dot(matrix1, 0x10, dx, dy, dz)), read_float(limits, 0x34));
        if y_limit < 0.0 {
            y_limit = 0.0;
        }
        let partial = add_sse(y_limit, x_limit);

        let target_axes = read_word(target_object, 0x20);
        dx = sub_sse(read_float(target_axes, 0x30), read_float(player_position, 0));
        dy = sub_sse(read_float(target_axes, 0x34), read_float(player_position, 4));
        dz = sub_sse(read_float(target_axes, 0x38), read_float(player_position, 8));
        let matrix2 = callee_thiscall!(C_VECTOR_DATA, u32, player);
        let third_projection = abs_sse(vector_dot(matrix2, 0x20, dx, dy, dz));
        let third_limit = sub_sse(third_projection, read_float(limits, 0x38));
        let third_contribution = if third_limit < 0.0 { third_limit } else { 0.0 };
        let total = add_sse(third_contribution, partial);
        let max_distance = relocated_float(VA_MAX_DISTANCE);
        if comiss_jb(max_distance, total) {
            return 1;
        }

        let target_head = read_word(target, 0);
        let target_byte = u32::from(read_byte(target, 0x20));
        let mut scratch_value = total.to_bits();
        let scratch_pointer = (&mut scratch_value as *mut u32) as u32;
        let _ = callee_cdecl!(
            C_CLASSIFY_TARGET,
            u32,
            player,
            target_head,
            scratch_pointer,
            player,
            target.wrapping_add(8),
            target.wrapping_add(4),
            target.wrapping_add(0x10),
            target_byte
        );

        1
    }
});
