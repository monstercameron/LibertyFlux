// original: 0x00971F00 aud_occlusion_group_update
//! Stage proof for the gate-open, zero-collision-result path of the audio occlusion update.
//!
//! The export increments the object's three-bit cursor, runs all three update
//! loops, issues the four intercepted cdecl call-site families, and writes the
//! default no-collision markers and bounds. This stage scripts every callee to
//! return zero and fixes the initial object bytes to zero; it does not establish
//! the positive collision-result paths or the native callee's behavior.

use lf_checker_rt::{export, global};

const GATE_VA: u32 = 0x0103_79D0;
const RADIUS_VA: u32 = 0x00FE_8C2C;
const NEGATIVE_ONE: u32 = 0xBF80_0000;
const ONE: u32 = 0x3F80_0000;

#[inline(always)]
unsafe fn read_f32(object: *const u8, offset: usize) -> f32 {
    f32::from_bits(unsafe { core::ptr::read_unaligned(object.add(offset).cast::<u32>()) })
}

#[inline(always)]
unsafe fn write_bits(object: *mut u8, offset: usize, bits: u32) {
    unsafe { core::ptr::write_unaligned(object.add(offset).cast::<u32>(), bits) };
}

#[inline(always)]
fn add32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

#[inline(always)]
fn sub32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) - core::hint::black_box(right)
}

unsafe fn run_stage(object_arg: u32, wrong_first_distance: bool) -> u32 {
    let object = object_arg as *mut u8;
    let gate = unsafe { global::<u8>(GATE_VA).read_unaligned() };
    if gate == 0 {
        // This stage pins the gate open; the original's closed-gate EAX is its
        // incoming register and cannot be observed by a safe thiscall export.
        return 0;
    }

    let radius = unsafe { global::<f32>(RADIUS_VA).read_unaligned() };
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
        let _ = lf_checker_rt::callee_cdecl!(1, u32, 0u32, 0u32, 0u32, 0u32, 6u32, 1u32, 3u32);
        let distance_bits = if wrong_first_distance && step == 0 {
            0xC000_0000
        } else {
            NEGATIVE_ONE
        };
        unsafe { write_bits(object, 0x17E0 + index as usize * 4, distance_bits) };
        let vector = (index as usize + 0x139) * 16;
        unsafe {
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

        if index % 3 == 0 {
            let _ = lf_checker_rt::callee_cdecl!(2, u32, 0u32, 0u32, 0u32, 0u32, 6u32, 1u32, 3u32);
            unsafe {
                write_bits(object, 0x1690 + (index / 3) as usize * 4, NEGATIVE_ONE);
            }
        }
    }

    // Four radial samples are updated from the zero-result branch.
    for lane in 0u32..4 {
        let index = cursor + lane * 8;
        let _ = lf_checker_rt::callee_cdecl!(3, u32, 0u32, 0u32, 0u32, 0u32, 6u32, 1u32, 3u32);
        unsafe { write_bits(object, 0x1960 + index as usize * 4, NEGATIVE_ONE) };

        let y = if (2..=6).contains(&index) {
            if index <= 5 {
                sub32(saved_y, radius)
            } else {
                saved_y
            }
        } else {
            add32(saved_y, radius)
        };
        unsafe { write_bits(object, 0x19E0 + index as usize * 4, y.to_bits()) };

        let x = if (1..=3).contains(&index) {
            add32(saved_x, radius)
        } else if index <= 4 {
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
    }

    // The final two calls update the selected edge samples. The stage's
    // fixed initial cursor selects the low-selector-one branch.
    let high = cursor >> 2;
    let low = cursor & 3;
    let mut return_index = 0u32;
    for step in 0u32..2 {
        let lane = step + high * 2;
        let index = low + lane * 4;
        let _ = lf_checker_rt::callee_cdecl!(4, u32, 0u32, 0u32, 0u32, 0u32, 6u32, 1u32, 3u32);
        unsafe { write_bits(object, 0x1B60 + index as usize * 4, NEGATIVE_ONE) };

        if low == 1 {
            unsafe { write_bits(object, 0x1BA0 + index as usize * 4, saved_y.to_bits()) };
            let x = if lane == 0 || lane == 3 {
                saved_x
            } else {
                add32(saved_x, radius)
            };
            unsafe { write_bits(object, 0x1BE0 + index as usize * 4, x.to_bits()) };
            let z = if lane < 2 {
                add32(saved_z, radius)
            } else {
                sub32(saved_z, radius)
            };
            unsafe { write_bits(object, 0x1C20 + index as usize * 4, z.to_bits()) };
        }
        return_index = index;
    }

    return_index
}

export!(thiscall, rw_00971f00(object: u32) -> u32 {
    unsafe { run_stage(object, false) }
});

// One deliberate wrong version: the first no-collision distance marker is
// changed while every other operation follows the same stage implementation.
export!(thiscall, mut_00971f00_wrong_first_distance(object: u32) -> u32 {
    unsafe { run_stage(object, true) }
});
