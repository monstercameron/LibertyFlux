// original: 0x00971F00 aud_occlusion_group_update
//! Audio occlusion group update (thiscall, one object argument, result in EAX).
//!
//! Gate closed: the function returns without touching the object. The
//! original returns the incoming EAX here; this rewrite returns zero, which no
//! caller reads (the only direct caller discards the result).
//!
//! Gate open: advance the three-bit cursor at object+0x1354, then
//! 1. three samples at cursor, cursor+8, cursor+16 each query callee 1; a miss
//!    stores a negative distance and a unit vector and places a sample point
//!    around the saved position; a hit stores the distance to the global point,
//!    normalizes the point into the vector slot, and copies the fourth vector
//!    word from the callee's out-parameter buffer (word 11);
//! 2. when the sample index is a multiple of three, query callee 2 and store
//!    either a negative marker or the distance from a ray sample to the point;
//! 3. four radial samples per cursor query callee 3 and store a miss marker or
//!    the distance and point copies; lane 1 also stores the normalized point
//!    and the out-parameter word;
//! 4. two edge samples query callee 4 and store a miss marker or the distance
//!    and point copies into the edge tables, returning the last edge index.
//!
//! Callees 1 to 4 each receive the out-parameter buffer as their fourth stack
//! argument. The callee is assumed to write word 11 of that buffer (its byte 44)
//! on every call; this is the only word the function reads without writing
//! (see the lane's summary for the traced chain and the assumption).

use lf_checker_rt::{export, global};

pub(super) const GATE_VA: u32 = 0x0103_79D0;
const RADIUS_VA: u32 = 0x00FE_8C2C;
const POSITIVE_POINT_X_VA: u32 = 0x01B4_B320;
const POSITIVE_POINT_Y_VA: u32 = 0x01B4_B324;
const POSITIVE_POINT_Z_VA: u32 = 0x01B4_B328;
const HIT_DISTANCE_BIAS_VA: u32 = 0x0103_79F4;
const HIT_DISTANCE_LIMIT_VA: u32 = 0x0103_7A1C;
pub(super) const NEGATIVE_ONE: u32 = 0xBF80_0000;
const ONE: u32 = 0x3F80_0000;
/// Words in the out-parameter buffer passed as the fourth stack argument of
/// every gate-open callee call. The native caller's buffer is a frame area
/// whose word at byte 44 (buffer word 11) is never written by the function
/// itself; the callee writes it through the pointer, and the positive
/// result paths copy it into the object's fourth vector word.
const CALLEE_FRAME_WORDS: usize = 12;
const FRAME_WORD_INDEX: usize = 11;

#[inline(always)]
unsafe fn read_f32(object: *const u8, offset: usize) -> f32 {
    f32::from_bits(unsafe { core::ptr::read_unaligned(object.add(offset).cast::<u32>()) })
}

#[inline(always)]
pub(super) unsafe fn write_bits(object: *mut u8, offset: usize, bits: u32) {
    unsafe { core::ptr::write_unaligned(object.add(offset).cast::<u32>(), bits) };
}

/// Calls intercepted callee `id` with the shape every gate-open call site uses:
/// two unread pointers, a zero flag, the out-parameter buffer as the fourth
/// argument, then the constants 6, 1 and 3. The buffer's address is handed to
/// the callee, which may write through it; the caller reads the result words
/// afterwards.
#[inline(always)]
fn call_callee(id: u32, call_frame: &mut [u32; CALLEE_FRAME_WORDS]) -> u32 {
    lf_checker_rt::callee_cdecl!(
        id,
        u32,
        0u32,
        0u32,
        0u32,
        call_frame.as_mut_ptr() as u32,
        6u32,
        1u32,
        3u32
    )
}

#[inline(always)]
fn add32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

#[inline(always)]
fn sub32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) - core::hint::black_box(right)
}

#[inline(always)]
fn mul32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) * core::hint::black_box(right)
}

pub(super) unsafe fn run_stage(object_arg: u32, wrong_first_distance: bool, wrong_positive_x: bool, wrong_positive_id1_x: bool, wrong_positive_id2_distance: bool, wrong_positive_id3_distance: bool, wrong_frame_word: bool) -> u32 {
    let object = object_arg as *mut u8;
    let gate = unsafe { global::<u8>(GATE_VA).read_unaligned() };
    if gate == 0 {
        // This stage pins the gate open; the original's closed-gate EAX is its
        // incoming register and cannot be observed by a safe thiscall export.
        return 0;
    }

    let radius = unsafe { global::<f32>(RADIUS_VA).read_unaligned() };
    let clip_limit = unsafe { global::<f32>(HIT_DISTANCE_LIMIT_VA).read_unaligned() };
    let mut clip_distance = clip_limit;
    // One out-parameter buffer shared by every callee call, as the native
    // caller's frame area is; its word 11 is what the callee writes.
    let mut call_frame = [0u32; CALLEE_FRAME_WORDS];
    let saved_x = unsafe { read_f32(object, 0x580) };
    let saved_y = unsafe { read_f32(object, 0x584) };
    let saved_z = unsafe { read_f32(object, 0x588) };
    let cursor = unsafe { object.add(0x1354).read() }.wrapping_add(1) & 7;
    unsafe { object.add(0x1354).write(cursor) };
    let cursor = u32::from(cursor);

    // The first loop updates three angular samples and may query a paired
    // sample once when its selector is divisible by three.
    for step in [0u32, 8, 16] {
        let index = cursor + step;
        let hit = call_callee(1, &mut call_frame);
        let vector = (index as usize + 0x139) * 16;
        if hit == 0 {
            let distance_bits = if wrong_first_distance && step == 0 {
                0xC000_0000
            } else {
                NEGATIVE_ONE
            };
            unsafe {
                write_bits(object, 0x17E0 + index as usize * 4, distance_bits);
                write_bits(object, vector, ONE);
                write_bits(object, vector + 4, ONE);
                write_bits(object, vector + 8, ONE);
            }

            let y = if (6..=18).contains(&index) {
                if (7..=17).contains(&index) {
                    sub32(saved_y, radius)
                } else {
                    saved_y
                }
            } else {
                add32(saved_y, radius)
            };
            unsafe { write_bits(object, 0x1840 + index as usize * 4, y.to_bits()) };

            let x = if (1..=11).contains(&index) {
                add32(saved_x, radius)
            } else if index > 12 {
                sub32(saved_x, radius)
            } else {
                saved_x
            };
            unsafe { write_bits(object, 0x18A0 + index as usize * 4, x.to_bits()) };
        } else {
            let point_x_bits = unsafe { global::<u32>(POSITIVE_POINT_X_VA).read_unaligned() };
            let point_y_bits = unsafe { global::<u32>(POSITIVE_POINT_Y_VA).read_unaligned() };
            let point_z_bits = unsafe { global::<u32>(POSITIVE_POINT_Z_VA).read_unaligned() };
            let point_x = f32::from_bits(point_x_bits);
            let point_y = f32::from_bits(point_y_bits);
            let point_z = f32::from_bits(point_z_bits);

            let delta_y = sub32(point_y, saved_y);
            let delta_x = sub32(point_x, saved_x);
            let delta_z = sub32(point_z, saved_z);
            let distance_squared = add32(
                add32(mul32(delta_y, delta_y), mul32(delta_x, delta_x)),
                mul32(delta_z, delta_z),
            );
            let positive_distance = distance_squared.sqrt();
            unsafe { write_bits(object, 0x17E0 + index as usize * 4, positive_distance.to_bits()) };
            let distance_bias = unsafe { global::<f32>(HIT_DISTANCE_BIAS_VA).read_unaligned() };
            let distance_limit = unsafe { global::<f32>(HIT_DISTANCE_LIMIT_VA).read_unaligned() };
            let distance_from_hit = sub32(positive_distance, distance_bias);
            let limited_distance = if distance_limit > distance_from_hit {
                distance_from_hit
            } else {
                distance_limit
            };
            clip_distance = if limited_distance >= 0.0 {
                limited_distance
            } else {
                0.0
            };

            // The original initializes this local four-float vector from the
            // runtime-filled three-word vector, then normalizes X/Y/Z in place.
            // Its fourth source word is outside the mapped three-word vector and
            // has no observed initialization in the target prologue.
            let vector_x = point_x;
            let vector_y = point_y;
            let vector_z = point_z;
            let frame_word = if wrong_frame_word {
                0
            } else {
                call_frame[FRAME_WORD_INDEX]
            };
            unsafe {
                write_bits(object, vector, vector_x.to_bits());
                write_bits(object, vector + 4, vector_y.to_bits());
                write_bits(object, vector + 8, vector_z.to_bits());
                write_bits(object, vector + 12, frame_word);
            }
            let vector_squared = add32(
                add32(mul32(vector_x, vector_x), mul32(vector_y, vector_y)),
                mul32(vector_z, vector_z),
            );
            let normalization_scale = if vector_squared == 0.0 {
                0.0
            } else {
                f32::from_bits(ONE) / vector_squared.sqrt()
            };
            let normalized_x = mul32(vector_x, normalization_scale);
            let normalized_y = mul32(vector_y, normalization_scale);
            let normalized_z = mul32(vector_z, normalization_scale);
            let normalized_x_bits = if wrong_positive_id1_x {
                normalized_x.to_bits() ^ 0x8000_0000
            } else {
                normalized_x.to_bits()
            };
            unsafe {
                write_bits(object, vector, normalized_x_bits);
                write_bits(object, vector + 4, normalized_y.to_bits());
                write_bits(object, vector + 8, normalized_z.to_bits());
                write_bits(object, 0x1840 + index as usize * 4, point_y.to_bits());
                write_bits(object, 0x18A0 + index as usize * 4, point_x.to_bits());
            }
        }

        if index % 3 == 0 {
            let hit = call_callee(2, &mut call_frame);
            let distance_slot = 0x1690 + (index / 3) as usize * 4;
            if hit == 0 {
                unsafe { write_bits(object, distance_slot, NEGATIVE_ONE) };
            } else {
                let direction = index as usize * 16;
                let direction_x = unsafe { read_f32(object, direction) };
                let direction_y = unsafe { read_f32(object, direction + 4) };
                let direction_z = unsafe { read_f32(object, direction + 8) };
                let query_x = add32(mul32(direction_x, clip_distance), saved_x);
                let query_y = add32(mul32(direction_y, clip_distance), saved_y);
                // This distance sample uses the unshifted vertical coordinate.
                let query_z = add32(mul32(direction_z, clip_distance), saved_z);

                let point_x = f32::from_bits(unsafe {
                    global::<u32>(POSITIVE_POINT_X_VA).read_unaligned()
                });
                let point_y = f32::from_bits(unsafe {
                    global::<u32>(POSITIVE_POINT_Y_VA).read_unaligned()
                });
                let point_z = f32::from_bits(unsafe {
                    global::<u32>(POSITIVE_POINT_Z_VA).read_unaligned()
                });
                let delta_y = sub32(point_y, query_y);
                let delta_x = sub32(point_x, query_x);
                let delta_z = sub32(point_z, query_z);
                let squared_distance = add32(
                    add32(mul32(delta_y, delta_y), mul32(delta_x, delta_x)),
                    mul32(delta_z, delta_z),
                );
                let distance_bits = squared_distance.sqrt().to_bits();
                let distance_bits = if wrong_positive_id2_distance {
                    distance_bits ^ 0x8000_0000
                } else {
                    distance_bits
                };
                unsafe { write_bits(object, distance_slot, distance_bits) };
            }
        }
    }

    // Four radial samples use the caller's saved global-point locals after ID3.
    for lane in 0u32..4 {
        let index = cursor + lane * 8;
        // The native caller loads these values into stack locals before A536B0;
        // retain that lifetime across the scripted helper call.
        let point_x_bits = unsafe { global::<u32>(POSITIVE_POINT_X_VA).read_unaligned() };
        let point_y_bits = unsafe { global::<u32>(POSITIVE_POINT_Y_VA).read_unaligned() };
        let point_z_bits = unsafe { global::<u32>(POSITIVE_POINT_Z_VA).read_unaligned() };
        let point_x = f32::from_bits(point_x_bits);
        let point_y = f32::from_bits(point_y_bits);
        let point_z = f32::from_bits(point_z_bits);
        let hit = call_callee(3, &mut call_frame);

        if hit == 0 {
            unsafe { write_bits(object, 0x1960 + index as usize * 4, NEGATIVE_ONE) };

            let y = if (2..=6).contains(&cursor) {
                if (3..=5).contains(&cursor) {
                    sub32(saved_y, radius)
                } else {
                    saved_y
                }
            } else {
                add32(saved_y, radius)
            };
            unsafe { write_bits(object, 0x19E0 + index as usize * 4, y.to_bits()) };

            let x = if (1..=3).contains(&cursor) {
                add32(saved_x, radius)
            } else if cursor <= 4 {
                saved_x
            } else {
                sub32(saved_x, radius)
            };
            unsafe { write_bits(object, 0x1A60 + index as usize * 4, x.to_bits()) };

            let z = if lane < 2 {
                add32(saved_z, radius)
            } else {
                sub32(saved_z, radius)
            };
            unsafe { write_bits(object, 0x1AE0 + index as usize * 4, z.to_bits()) };

            if lane == 1 {
                unsafe { write_bits(object, 0x1358 + cursor as usize * 4, NEGATIVE_ONE) };
                let vector = (cursor as usize + 0x151) * 16;
                unsafe {
                    write_bits(object, vector, ONE);
                    write_bits(object, vector + 4, ONE);
                    write_bits(object, vector + 8, ONE);
                }
            }
        } else {
            // Native order: Y delta, X delta, Z delta; then Y^2 + X^2 + Z^2.
            let delta_y = sub32(point_y, saved_y);
            let delta_x = sub32(point_x, saved_x);
            let delta_z = sub32(point_z, saved_z);
            let distance_squared = add32(
                add32(mul32(delta_y, delta_y), mul32(delta_x, delta_x)),
                mul32(delta_z, delta_z),
            );
            let positive_distance = distance_squared.sqrt();
            let radial_distance_bits = if wrong_positive_id3_distance {
                positive_distance.to_bits() ^ 0x8000_0000
            } else {
                positive_distance.to_bits()
            };
            unsafe {
                write_bits(object, 0x1960 + index as usize * 4, radial_distance_bits);
                write_bits(object, 0x19E0 + index as usize * 4, point_y_bits);
                write_bits(object, 0x1A60 + index as usize * 4, point_x_bits);
                write_bits(object, 0x1AE0 + index as usize * 4, point_z_bits);
            }

            if lane == 1 {
                let vector_squared = add32(
                    add32(mul32(point_x, point_x), mul32(point_y, point_y)),
                    mul32(point_z, point_z),
                );
                let normalization_scale = if vector_squared == 0.0 {
                    0.0
                } else {
                    f32::from_bits(ONE) / vector_squared.sqrt()
                };
                let vector = (cursor as usize + 0x151) * 16;
                // The fourth word of the local vector is the callee's write
                // through the out-parameter buffer (see CALLEE_FRAME_WORDS).
                let frame_word = if wrong_frame_word {
                    0
                } else {
                    call_frame[FRAME_WORD_INDEX]
                };
                unsafe {
                    write_bits(object, 0x1358 + cursor as usize * 4, positive_distance.to_bits());
                    write_bits(object, vector, point_x_bits);
                    write_bits(object, vector + 4, point_y_bits);
                    write_bits(object, vector + 8, point_z_bits);
                    write_bits(object, vector + 12, frame_word);
                }
                // The native multiplies the scale destination by each vector
                // source; keep that operand order for NaN payload parity.
                let normalized_x = mul32(normalization_scale, point_x);
                let normalized_y = mul32(normalization_scale, point_y);
                let normalized_z = mul32(normalization_scale, point_z);
                unsafe {
                    write_bits(object, vector, normalized_x.to_bits());
                    write_bits(object, vector + 4, normalized_y.to_bits());
                    write_bits(object, vector + 8, normalized_z.to_bits());
                }
            }
        }
    }

    // The final two calls update selector-dependent edge samples. The zero-
    // result branch writes two Y bounds, one X bound and one Z bound per lane.
    let high = cursor >> 2;
    let low = cursor & 3;
    let mut return_index = 0u32;
    for step in 0u32..2 {
        let lane = step + high * 2;
        let index = low + lane * 4;
        let hit = call_callee(4, &mut call_frame);

        if hit == 0 {
            unsafe { write_bits(object, 0x1B60 + index as usize * 4, NEGATIVE_ONE) };

            let side_lane = lane == 1 || lane == 2;
            let y = match low {
                0 if side_lane => add32(saved_y, radius),
                2 if side_lane => sub32(saved_y, radius),
                _ => saved_y,
            };
            unsafe { write_bits(object, 0x1BA0 + index as usize * 4, y.to_bits()) };

            let x = match low {
                1 if side_lane => add32(saved_x, radius),
                3 if side_lane => sub32(saved_x, radius),
                _ => saved_x,
            };
            unsafe { write_bits(object, 0x1BE0 + index as usize * 4, x.to_bits()) };

            let z = if lane < 2 {
                add32(saved_z, radius)
            } else {
                sub32(saved_z, radius)
            };
            unsafe { write_bits(object, 0x1C20 + index as usize * 4, z.to_bits()) };
        } else {
            let point_x_bits = unsafe { global::<u32>(POSITIVE_POINT_X_VA).read_unaligned() };
            let point_y_bits = unsafe { global::<u32>(POSITIVE_POINT_Y_VA).read_unaligned() };
            let point_z_bits = unsafe { global::<u32>(POSITIVE_POINT_Z_VA).read_unaligned() };
            let point_x = f32::from_bits(point_x_bits);
            let point_y = f32::from_bits(point_y_bits);
            let point_z = f32::from_bits(point_z_bits);

            let delta_y = sub32(point_y, saved_y);
            let delta_x = sub32(point_x, saved_x);
            let delta_z = sub32(point_z, saved_z);
            let distance_squared = add32(
                add32(mul32(delta_y, delta_y), mul32(delta_x, delta_x)),
                mul32(delta_z, delta_z),
            );
            let distance = distance_squared.sqrt();
            let written_x_bits = if wrong_positive_x {
                point_x_bits ^ 0x8000_0000
            } else {
                point_x_bits
            };

            unsafe {
                write_bits(object, 0x1B60 + index as usize * 4, distance.to_bits());
                write_bits(object, 0x1BA0 + index as usize * 4, point_y_bits);
                write_bits(object, 0x1BE0 + index as usize * 4, written_x_bits);
                write_bits(object, 0x1C20 + index as usize * 4, point_z_bits);
            }
        }
        return_index = index;
    }

    return_index
}

export!(thiscall, rw_00971f00(object: u32) -> u32 {
    unsafe { run_stage(object, false, false, false, false, false, false) }
});

// One deliberate wrong version: the first no-collision distance marker is
// changed while every other operation follows the same stage implementation.
export!(thiscall, mut_00971f00_wrong_first_distance(object: u32) -> u32 {
    unsafe { run_stage(object, true, false, false, false, false, false) }
});

// One deliberate wrong version: reverse the normalized X bit pattern in the
// newly covered positive ID1 branch while preserving every other operation.
export!(thiscall, mut_00971f00_wrong_positive_id1_x(object: u32) -> u32 {
    unsafe { run_stage(object, false, false, true, false, false, false) }
});

// One deliberate wrong version: reverse only the positive ID2 distance result.
export!(thiscall, mut_00971f00_wrong_positive_id2_distance(object: u32) -> u32 {
    unsafe { run_stage(object, false, false, false, true, false, false) }
});

// One deliberate wrong version: reverse only the radial distance in the
// newly covered positive ID3 branch.
export!(thiscall, mut_00971f00_wrong_positive_id3_distance(object: u32) -> u32 {
    unsafe { run_stage(object, false, false, false, false, true, false) }
});

// One deliberate wrong version: reverse the sign of the positive ID4 X sample.
export!(thiscall, mut_00971f00_wrong_positive_x(object: u32) -> u32 {
    unsafe { run_stage(object, false, true, false, false, false, false) }
});

// One deliberate wrong version: the positive ID1 and ID3 frame word is taken
// as zero, the model the earlier lane used, instead of the callee's write.
export!(thiscall, mut_00971f00_wrong_frame_word(object: u32) -> u32 {
    unsafe { run_stage(object, false, false, false, false, false, true) }
});

// One deliberate wrong version: on the closed gate, one distance slot is also
// written, otherwise identical.
export!(thiscall, mut_00971f00_wrong_closed_gate(object: u32) -> u32 {
    let gate = unsafe { global::<u8>(GATE_VA).read_unaligned() };
    if gate == 0 {
        unsafe { write_bits(object as *mut u8, 0x17E4, NEGATIVE_ONE) };
        0
    } else {
        unsafe { run_stage(object, false, false, false, false, false, false) }
    }
});

// One deliberate wrong version: the last edge distance is also copied one
// word higher in the same table.
export!(thiscall, mut_00971f00_wrong_neighbour_slot(object: u32) -> u32 {
    let ret = unsafe { run_stage(object, false, false, false, false, false, false) };
    let source = (object as usize + 0x1B60 + ret as usize * 4) as *const u32;
    let value = unsafe { core::ptr::read_unaligned(source) };
    let neighbour = (object as usize + 0x1B60 + (ret as usize + 1) * 4) as *mut u32;
    unsafe { core::ptr::write_unaligned(neighbour, value) };
    ret
});
