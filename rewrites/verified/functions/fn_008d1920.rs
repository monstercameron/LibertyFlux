// original: 0x008D1920 PlayerTargetCheck
/// Checks player and target eligibility, projects their relative position,
/// applies the configured target range, and invokes the target action when
/// all geometric and state gates pass. The common tail returns one in AL.
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

const C_RULE_A: u32 = 1;
const C_RULE_B: u32 = 2;
const C_RULE_C: u32 = 3;
const C_RULE_D: u32 = 4;
const C_CREATE_TRANSFORM: u32 = 5;
const C_LINK_TRANSFORM: u32 = 6;
const C_POSITION: u32 = 7;
const C_ORIENTATION: u32 = 8;
const C_ACCEPT: u32 = 9;
const C_EXTRA_RANGE: u32 = 10;
const C_BUILD_RANGE: u32 = 11;
const C_TEST_RANGE: u32 = 12;
const C_APPLY_TARGET: u32 = 13;
const C_COOKIE_CHECK: u32 = 14;

const VA_SECURITY_COOKIE: u32 = 0x0105_7FB4;
const VA_TARGET_STATE: u32 = 0x012F_A50C;
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
unsafe fn relocated_float(address: u32) -> f32 {
    f32::from_bits(unsafe { global::<u32>(address).read_unaligned() })
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
unsafe fn abs_sse(value: f32) -> f32 {
    let mask = unsafe { global::<u32>(VA_ABS_MASK).read_unaligned() };
    f32::from_bits(value.to_bits() & mask)
}

#[inline(always)]
fn point_base(player: u32) -> u32 {
    let linked = unsafe { read_word(player, 0x20) };
    if linked == 0 { player.wrapping_add(0x10) } else { linked.wrapping_add(0x30) }
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

#[inline(never)]
unsafe fn finish() -> u32 {
    let cookie = unsafe { global::<u32>(VA_SECURITY_COOKIE).read_unaligned() };
    let stack_anchor = core::ptr::addr_of!(cookie) as u32;
    let _ = callee_thiscall!(C_COOKIE_CHECK, u32, cookie ^ stack_anchor);
    1
}

export!(cdecl, rw_008d1920(player: u32, target: u32) -> u32 {
    unsafe {
        if read_byte(player, 0x24) & 1 == 0
            || read_word(player, 0x28) & 0x3C0 != 0x80
        {
            return finish();
        }

        let mode = read_word(player, 0x1300);
        let mode_two = mode == 2;
        let mut direction_flag = 1_u8;

        if read_byte(player, 0xF17) & 8 == 0 {
            if mode == 0 {
                let allowed = callee_thiscall!(C_RULE_A, u32, player);
                if allowed == 0 && read_byte(player, 0x118) & 1 != 0 {
                    return finish();
                }
            }

            if mode == 1 {
                let transform = read_word(player, 0x20);
                if comiss_ja(read_float(transform, 0x28), relocated_float(0x00FE_8878)) {
                    let allowed = callee_thiscall!(C_RULE_A, u32, player);
                    if allowed == 0 && read_byte(player, 0x118) & 1 != 0 {
                        return finish();
                    }
                }
            }

            if callee_thiscall!(C_RULE_B, u32, player.wrapping_add(0x10D0), 1_u32) as u8 == 0
                || read_word(player, 0x28) & 0x7C00 == 0x0C00
            {
                return finish();
            }

            if mode_two {
                let target_head = read_word(target, 0);
                if callee_thiscall!(C_RULE_C, u32, player, target_head) as u8 == 0 {
                    return finish();
                }
                direction_flag = 0;
            } else {
                direction_flag = 0;
            }
        }

        if mode == 3 {
            let index = read_half(player, 0x2E) as i16 as i32;
            let state = global::<i32>(VA_TARGET_STATE).read_unaligned();
            if index == state {
                return finish();
            }

            let target_head = read_word(target, 0);
            let target_axes = read_word(target_head, 0x20);
            let player_axes = read_word(player, 0x20);
            let dx = sub_sse(read_float(target_axes, 0x34), read_float(player_axes, 0x34));
            let dy = sub_sse(read_float(target_axes, 0x30), read_float(player_axes, 0x30));
            let dz = sub_sse(read_float(target_axes, 0x38), read_float(player_axes, 0x38));
            let mut projection = mul_sse(read_float(player_axes, 4), dx);

            if read_byte(player, 0x14E5) & 0x10 != 0 {
                let x_projection = mul_sse(dy, read_float(player_axes, 0));
                let z_projection = mul_sse(read_float(player_axes, 8), dz);
                projection = add_sse(projection, x_projection);
                projection = add_sse(projection, z_projection);
                if !comiss_jbe(0.0, projection) {
                    return finish();
                }
            } else {
                let x_projection = mul_sse(read_float(player_axes, 0), dy);
                projection = add_sse(projection, x_projection);
                let z_projection = mul_sse(read_float(player_axes, 8), dz);
                projection = add_sse(projection, z_projection);
                if comiss_ja(projection, relocated_float(VA_DOT_SCALE)) || direction_flag != 0 {
                    return finish();
                }
            }
        }

        if read_word(player, 0x20) == 0 {
            let _ = callee_thiscall!(C_CREATE_TRANSFORM, u32, player);
            let _ = callee_thiscall!(C_LINK_TRANSFORM, u32, player.wrapping_add(0x10), 0_u32);
        }

        let transform = read_word(player, 0x20);
        if !comiss_ja(read_float(transform, 0x28), relocated_float(0x00FE_87E8)) {
            if mode != 1 {
                return finish();
            }
            let matrix = callee_thiscall!(C_RULE_D, u32, player);
            if comiss_jbe(read_float(matrix, 0x28), relocated_float(0x00FE_8D7C)) {
                return finish();
            }
        }

        let mut position_output = [0_u32; 4];
        let _ = callee_thiscall!(C_POSITION, u32, player, position_output.as_mut_ptr() as u32);
        let position = point_base(player);
        let player_x = read_float(position, 0);
        let player_y = read_float(position, 4);
        let player_z = read_float(position, 8);
        let orientation = callee_thiscall!(C_ORIENTATION, f32, player);
        let adjusted_z = add_sse(sub_sse(player_z, orientation), relocated_float(VA_DOT_BIAS));

        let target_head = read_word(target, 0);
        let target_axes = read_word(target_head, 0x20);
        let dx = sub_sse(read_float(target_axes, 0x30), player_x);
        let dy = sub_sse(read_float(target_axes, 0x34), player_y);
        let dz = sub_sse(read_float(target_axes, 0x38), adjusted_z);
        let horizontal = add_sse(dy, dx);
        let scaled = mul_sse(horizontal, relocated_float(VA_DOT_SCALE));
        let angle = abs_sse(add_sse(scaled, dz));
        let target_is_player = read_word(target_head, 0xAB0) == player;
        let angle_reaches_limit = comiss_jbe(relocated_float(VA_ANGLE_LIMIT), angle);

        if mode_two {
            if !target_is_player && read_byte(target_head, 0x118) & 1 == 0 {
                return finish();
            }
        } else if !angle_reaches_limit {
            return finish();
        }

        if read_byte(player, 0xF1A) & 0x80 == 0 {
            return finish();
        }

        let mut out_x = 0_u32;
        let mut out_y = 0_u32;
        let accepted = callee_cdecl!(
            C_ACCEPT,
            u32,
            target_head,
            player,
            (&mut out_x as *mut u32) as u32,
            (&mut out_y as *mut u32) as u32
        );
        if accepted as u8 == 0 {
            return finish();
        }

        let target_x = read_float(target_axes, 0x30);
        let target_y = read_float(target_axes, 0x34);
        let diff_x = sub_sse(target_y, f32::from_bits(out_x));
        let diff_y = sub_sse(target_x, f32::from_bits(out_y));
        let distance = add_sse(mul_sse(diff_x, diff_x), mul_sse(diff_y, diff_y)).sqrt();
        let mut max_distance = relocated_float(VA_MAX_DISTANCE);

        if mode_two {
            max_distance = add_sse(max_distance, callee_thiscall!(C_EXTRA_RANGE, f32, player));
        }

        let mut selected_distance = distance;
        if comiss_ja(distance, max_distance) {
            let mut range_builder = [0_u32; 32];
            let mut range_test = [0_u32; 16];
            let _ = callee_thiscall!(
                C_BUILD_RANGE,
                u32,
                range_builder.as_mut_ptr() as u32,
                player,
                read_float(target_axes, 0x38).to_bits(),
                relocated_float(0x00E9_D494).to_bits(),
                0_u32,
                0_u32,
                0_u32,
                0_u32,
                0_u32,
                0_u32
            );
            let in_range = callee_thiscall!(
                C_TEST_RANGE,
                u32,
                range_test.as_mut_ptr() as u32,
                target_axes.wrapping_add(0x30)
            ) as u8 != 0;
            if in_range {
                selected_distance = max_distance;
            } else if comiss_jb(max_distance, distance) {
                return finish();
            }
        }

        let target_byte = u32::from(read_byte(target, 0x20));
        let _ = callee_cdecl!(
            C_APPLY_TARGET,
            u32,
            player,
            target_head,
            selected_distance.to_bits(),
            target_head,
            target.wrapping_add(8),
            target.wrapping_add(4),
            target.wrapping_add(0x10),
            target_byte
        );
        finish()
    }
});
