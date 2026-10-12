// original: 0x00aff980 VehicleTeleport
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

const POOL_HANDLE: u32 = 0x012e22a4;
const FIXED_MANAGER: u32 = 0x011e67c0;
const LOWER_LIMIT: f32 = -100.0;
const HEIGHT_SCALE: f32 = 0.7;
const DEFAULT_HEIGHT: f32 = -10.0;
const CHILD_DISTANCE_SQUARED: f32 = 25.0;

/// Resolve the entity handle, select a vertical placement, invoke the
/// placement hooks, then copy each present child matrix into its cached
/// transform. The horizontal coordinates and two byte flags come from the
/// caller's stack arguments. The native routine also stores its selected
/// vertical coordinate back into the caller's third float slot.
unsafe fn vehicle_teleport_impl(
    object: u32,
    x_bits: u32,
    z_bits: u32,
    height_bits: u32,
    use_zero_height: u32,
    query_height: u32,
) -> u32 {
    let requested_height = f32::from_bits(height_bits);
    let original_matrix = (object + 0x20) as *const u32;
    let original_matrix = original_matrix.read_unaligned();
    let original_x = ((original_matrix + 0x30) as *const f32).read_unaligned();
    let original_y = ((original_matrix + 0x34) as *const f32).read_unaligned();
    let original_z = ((original_matrix + 0x38) as *const f32).read_unaligned();

    let pool = global::<u32>(POOL_HANDLE).read();
    let handle: u32 = callee_thiscall!(1, u32, pool, object);

    let mut selected_height = requested_height;
    let mut scale = 1.0_f32;
    if !(requested_height > LOWER_LIMIT || requested_height.is_nan()) {
        let sampled: f64 = callee_cdecl!(2, f64, x_bits, z_bits, 4);
        let placed_height = sampled as f32;
        selected_height = placed_height;
        if ((object + 0x1304) as *const u32).read_unaligned() == 2 {
            // The native helper receives a transient stack record. Keep its
            // observed words explicit so the contract can snapshot the data
            // without comparing the build-dependent pointer itself.
            let mut probe_record = [height_bits, 1.0_f32.to_bits(), 0, original_x.to_bits()];
            let audible: u8 = callee_cdecl!(
                3,
                u8,
                x_bits,
                z_bits,
                0,
                probe_record.as_mut_ptr() as u32,
                0,
                0,
                0,
                0x40c00000,
                0x41a00000,
                0
            );
            if audible != 0 && requested_height > placed_height {
                selected_height = requested_height;
                scale = HEIGHT_SCALE;
            }
        }
    }

    if query_height as u8 != 0 {
        let model_height: f32 = callee_thiscall!(4, f32, object);
        let product = core::hint::black_box(model_height * scale);
        selected_height = product + selected_height;
    }

    let _cleared: u32 = callee_thiscall!(
        5,
        u32,
        relocated(FIXED_MANAGER),
        handle
    );

    let mut placement_record = [x_bits, z_bits, selected_height.to_bits(), 0];
    let base_height_bits = if use_zero_height as u8 != 0 {
        0
    } else {
        DEFAULT_HEIGHT.to_bits()
    };
    let state = ((object + 0x1300) as *const u32).read_unaligned();
    let _placed: u32 = callee_thiscall!(
        11,
        u32,
        object,
        placement_record.as_mut_ptr() as u32,
        base_height_bits,
        0
    );
    if state != 2 {
        ((object + 0xbe) as *mut u8).write(1);
        if query_height as u8 != 0 {
            let local_mask = *((object + 0x10b8) as *const u8) & 7;
            let set = u32::from(local_mask != 0);
            if state == 0 {
                let _grounded: u32 = callee_thiscall!(
                    7,
                    u32,
                    object,
                    set,
                    0x40a00000,
                    0x40a00000
                );
            } else if state == 1 {
                let _grounded: u32 = callee_thiscall!(
                    6,
                    u32,
                    object,
                    set,
                    0x40a00000,
                    0x40a00000
                );
            }
        }
    }

    let _object_coordinates: u32 = callee_cdecl!(8, u32, object);
    let _reset_handles: u32 = callee_thiscall!(9, u32, object + 0xdd4);
    let _object_vtable = (object as *const u32).read_unaligned();
    let _updated: u32 = callee_thiscall!(12, u32, object);
    let matrix_pointer = ((object + 0x20) as *const u32).read_unaligned();

    let new_x = ((matrix_pointer + 0x30) as *const f32).read_unaligned();
    let new_y = ((matrix_pointer + 0x34) as *const f32).read_unaligned();
    let new_z = ((matrix_pointer + 0x38) as *const f32).read_unaligned();
    let dy = core::hint::black_box(new_y - original_y);
    let dx = core::hint::black_box(new_x - original_x);
    let dz = core::hint::black_box(new_z - original_z);
    let dy_squared = core::hint::black_box(dy * dy);
    let dx_squared = core::hint::black_box(dx * dx);
    let dz_squared = core::hint::black_box(dz * dz);
    let horizontal = core::hint::black_box(dy_squared + dx_squared);
    let distance_squared = core::hint::black_box(horizontal + dz_squared);
    let distant = distance_squared > CHILD_DISTANCE_SQUARED;

    let first_child = ((object + 0xf50) as *const u32).read_unaligned();
    if first_child != 0 {
        if distant {
            let _child_started: u32 = callee_thiscall!(13, u32, first_child, matrix_pointer, 0, 1);
            let _child_updated: u32 = callee_thiscall!(14, u32, first_child, 0);
        }
        let child_matrix = ((first_child + 0x20) as *const u32).read_unaligned();
        for word in 0..4_u32 {
            let value = ((child_matrix + 0x30 + word * 4) as *const u32).read_unaligned();
            ((first_child + 0xa0 + word * 4) as *mut u32).write_unaligned(value);
            ((first_child + 0x90 + word * 4) as *mut u32).write_unaligned(value);
        }
        ((first_child + 0xc7) as *mut u8).write(1);
    }

    for index in 0..8_u32 {
        let child = ((object + 0xf54 + index * 4) as *const u32).read_unaligned();
        if child != 0 {
            if distant {
                let _child_started: u32 = callee_thiscall!(13, u32, child, matrix_pointer, 0, 1);
                let _child_updated: u32 = callee_thiscall!(14, u32, child, 0);
            }
            let child_matrix = ((child + 0x20) as *const u32).read_unaligned();
            let destination = child + 0x80;
            for word in 0..4_u32 {
                let value = ((child_matrix + 0x30 + word * 4) as *const u32).read_unaligned();
                ((destination + 0x20 + word * 4) as *mut u32).write_unaligned(value);
                ((destination + 0x10 + word * 4) as *mut u32).write_unaligned(value);
            }
            ((child + 0xc7) as *mut u8).write(1);
        }
    }

    let link = ((object + 0x6c) as *const u32).read_unaligned();
    if link != 0 {
        let _result: u32 = callee_cdecl!(10, u32, 1, 1);
        return _result;
    }
    object.wrapping_add(0xf74)
}

export!(cdecl, rw_aff980(object: u32, x_bits: u32, z_bits: u32, height_bits: u32, use_zero_height: u32, query_height: u32) -> u32 {
    unsafe {
        vehicle_teleport_impl(object, x_bits, z_bits, height_bits, use_zero_height, query_height)
    }
});
