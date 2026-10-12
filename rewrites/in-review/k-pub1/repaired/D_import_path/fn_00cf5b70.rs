// original: 0x00cf5b70 ped_task_climb
use lf_checker_rt::{add_f32, mul_f32, neg_f32, read_f32, read_u32, sub_f32, write_u32};

/// Advance a climb task for one frame. The pedestrian flag is set first; a
/// positive predicate result delegates to the early helper and returns its
/// word. Otherwise a missing or blocked state sets task status two, and a
/// false solver result returns immediately. The successful solver path eases
/// its angle, may attach an optional pointer, and transforms the solver's
/// vector when a valid matrix is available. It then normalizes the angle,
/// rotates the task's three axes, stores the resulting vector and solver
/// slots, and returns the optional-pointer slot. Matrix and state early paths
/// are distinct from the successful transform path.
unsafe fn run(this: u32, ped: u32, target: u32, step: u32, mutant: bool) -> u32 {
    const PREDICATE: u32 = 1;
    const EARLY: u32 = 2;
    const SOLVE: u32 = 3;
    const EASE: u32 = 4;
    const ATTACH: u32 = 5;
    const HAS_MATRIX: u32 = 6;
    const MATRIX: u32 = 7;
    const ATAN2: u32 = 8;
    const NORMALIZE_A: u32 = 9;
    const NORMALIZE_B: u32 = 10;
    const TRANSFORM: u32 = 11;

    const STATE_PTR: u32 = 0x10;
    const STATUS: u32 = 0x14;
    const VECTOR: u32 = 0x20;
    const ANGLE: u32 = 0x30;
    const MATRIX_PTR: u32 = 0x64;
    const CLEAR_SLOT: u32 = 0x6c;
    const AXIS_X: u32 = 0x40;
    const AXIS_Y: u32 = 0x44;
    const AXIS_Z: u32 = 0x48;
    const EASE_SCALE: u32 = 0x4c;
    const FLAGS: u32 = 0xbe0;
    const GLOBAL_RATE: u32 = 0x0117_35bc;
    const GLOBAL_SCALE: u32 = 0x0105_394c;

    let mut output_vector = [0u32; 4];
    let mut output_angle = 0u32;
    let mut unused_output = 0u32;
    let mut output_return = 0u32;
    let mut transformed = [0u32; 10];

    unsafe { write_u32(ped.wrapping_add(FLAGS), read_u32(ped.wrapping_add(FLAGS)) | 0x20) };
    let predicate = lf_checker_rt::callee_stdcall!(PREDICATE, u32, ped, 0);
    if (predicate as u8) != 0 {
        return lf_checker_rt::callee_thiscall!(EARLY, u32, this, ped, target, step);
    }

    let state = unsafe { read_u32(this.wrapping_add(STATE_PTR)) };
    if state == 0 {
        unsafe { write_u32(this.wrapping_add(STATUS), 2) };
        return 0;
    }
    let state_flags = unsafe { read_u32(state.wrapping_add(0x74)) } >> 14;
    if (state_flags & 1) != 0 {
        unsafe { write_u32(this.wrapping_add(STATUS), 2) };
        return state_flags;
    }

    let vector_ptr = output_vector.as_mut_ptr() as u32;
    let solve_result: u32 = lf_checker_rt::callee_thiscall!(
        SOLVE,
        u32,
        this,
        ped,
        target,
        step,
        unsafe { read_u32(state.wrapping_add(0x10)) },
        unsafe { read_u32(state.wrapping_add(0x0c)) },
        unsafe { read_u32(state.wrapping_add(0x4c)) },
        this.wrapping_add(CLEAR_SLOT),
        vector_ptr,
        &mut output_angle as *mut u32 as u32,
        &mut unused_output as *mut u32 as u32,
        &mut output_return as *mut u32 as u32,
    );
    if (solve_result as u8) == 0 {
        return solve_result;
    }

    let rate = unsafe { read_f32(lf_checker_rt::relocated(GLOBAL_RATE)) };
    let scale = unsafe { read_f32(lf_checker_rt::relocated(GLOBAL_SCALE)) };
    let ease_scale = mul_f32(rate, scale);
    let eased: f32 = lf_checker_rt::callee_cdecl!(
        EASE,
        f32,
        step,
        output_angle,
        ease_scale.to_bits(),
    );
    output_angle = eased.to_bits();

    let optional_pointer = unused_output;
    let matrix_slot = unsafe { read_u32(this.wrapping_add(MATRIX_PTR)) };
    if optional_pointer != 0 && optional_pointer != matrix_slot {
        let _: u32 = lf_checker_rt::callee_thiscall!(ATTACH, u32, this, optional_pointer);
    }

    let matrix_owner = unsafe { read_u32(this.wrapping_add(MATRIX_PTR)) };
    if matrix_owner != 0 {
        let has_matrix = lf_checker_rt::callee_thiscall!(HAS_MATRIX, u32, matrix_owner);
        if (has_matrix as u8) != 0 {
            let matrix = lf_checker_rt::callee_thiscall!(MATRIX, u32, matrix_owner);
            let bearing_y = unsafe { read_f32(matrix.wrapping_add(4)) };
            let bearing_x = unsafe { read_f32(matrix) };
            let bearing_y = if mutant { neg_f32(bearing_y) } else { bearing_y };
            let y_bits = (core::hint::black_box(bearing_y) as f64).to_bits();
            let x_bits = (core::hint::black_box(bearing_x) as f64).to_bits();
            let bearing_bits: u64 = lf_checker_rt::callee_cdecl!(
                ATAN2,
                u64,
                y_bits as u32,
                (y_bits >> 32) as u32,
                x_bits as u32,
                (x_bits >> 32) as u32,
            );
            let bearing = core::hint::black_box(f64::from_bits(bearing_bits)) as f32;
            let angle_delta = sub_f32(f32::from_bits(output_angle), bearing);
            output_angle = lf_checker_rt::callee_cdecl!(
                NORMALIZE_A,
                f32,
                angle_delta.to_bits(),
            )
            .to_bits();

            let dx = sub_f32(
                f32::from_bits(output_vector[0]),
                unsafe { read_f32(matrix.wrapping_add(0x30)) },
            );
            let dy = sub_f32(
                f32::from_bits(output_vector[1]),
                unsafe { read_f32(matrix.wrapping_add(0x34)) },
            );
            let dz = sub_f32(
                f32::from_bits(output_vector[2]),
                unsafe { read_f32(matrix.wrapping_add(0x38)) },
            );

            let mut x = mul_f32(dx, unsafe { read_f32(matrix) });
            x = add_f32(x, mul_f32(dy, unsafe { read_f32(matrix.wrapping_add(4)) }));
            x = add_f32(x, mul_f32(dz, unsafe { read_f32(matrix.wrapping_add(8)) }));
            let mut y = mul_f32(dx, unsafe { read_f32(matrix.wrapping_add(0x10)) });
            y = add_f32(y, mul_f32(dy, unsafe { read_f32(matrix.wrapping_add(0x14)) }));
            y = add_f32(y, mul_f32(dz, unsafe { read_f32(matrix.wrapping_add(0x18)) }));
            let mut z = mul_f32(dx, unsafe { read_f32(matrix.wrapping_add(0x20)) });
            z = add_f32(z, mul_f32(dy, unsafe { read_f32(matrix.wrapping_add(0x24)) }));
            z = add_f32(z, mul_f32(dz, unsafe { read_f32(matrix.wrapping_add(0x28)) }));
            output_vector[0] = x.to_bits();
            output_vector[1] = y.to_bits();
            output_vector[2] = z.to_bits();
        }
    }

    let _: f32 = lf_checker_rt::callee_cdecl!(
        NORMALIZE_B,
        f32,
        sub_f32(
            f32::from_bits(output_angle),
            unsafe { read_f32(this.wrapping_add(ANGLE)) },
        )
        .to_bits(),
    );
    let _: u32 = lf_checker_rt::callee_thiscall!(
        TRANSFORM,
        u32,
        transformed.as_mut_ptr() as u32,
    );

    let axis_x = unsafe { read_f32(this.wrapping_add(AXIS_X)) };
    let axis_y = unsafe { read_f32(this.wrapping_add(AXIS_Y)) };
    let axis_z = unsafe { read_f32(this.wrapping_add(AXIS_Z)) };
    let mut out_x = mul_f32(f32::from_bits(output_vector[3]), axis_x);
    out_x = add_f32(out_x, mul_f32(f32::from_bits(transformed[3]), axis_y));
    out_x = add_f32(out_x, mul_f32(f32::from_bits(transformed[7]), axis_z));
    let mut out_y = mul_f32(f32::from_bits(transformed[0]), axis_x);
    out_y = add_f32(out_y, mul_f32(f32::from_bits(transformed[4]), axis_y));
    out_y = add_f32(out_y, mul_f32(f32::from_bits(transformed[8]), axis_z));
    let mut out_z = mul_f32(f32::from_bits(transformed[1]), axis_x);
    out_z = add_f32(out_z, mul_f32(f32::from_bits(transformed[5]), axis_y));
    out_z = add_f32(out_z, mul_f32(f32::from_bits(transformed[9]), axis_z));

    unsafe {
        write_u32(this.wrapping_add(AXIS_X), out_x.to_bits());
        write_u32(this.wrapping_add(AXIS_Y), out_y.to_bits());
        write_u32(this.wrapping_add(AXIS_Z), out_z.to_bits());
        write_u32(this.wrapping_add(VECTOR), output_return);
        write_u32(this.wrapping_add(VECTOR + 4), output_vector[0]);
        write_u32(this.wrapping_add(VECTOR + 8), output_vector[1]);
        write_u32(this.wrapping_add(VECTOR + 12), output_vector[2]);
        write_u32(this.wrapping_add(ANGLE), 0);
        write_u32(this.wrapping_add(EASE_SCALE), output_vector[2]);
        write_u32(this.wrapping_add(0x68), unused_output);
    }
    unused_output
}

lf_checker_rt::export!(thiscall, rw_00cf5b70(this: u32, ped: u32, target: u32, step: u32) -> u32 {
    unsafe { run(this, ped, target, step, false) }
});

/// Deliberately wrong: reverse the bearing helper's first double argument.
lf_checker_rt::export!(thiscall, mut_00cf5b70(this: u32, ped: u32, target: u32, step: u32) -> u32 {
    unsafe { run(this, ped, target, step, true) }
});

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn read_f32(address: u32) -> f32 {
    f32::from_bits(unsafe { read_u32(address) })
}

#[inline(always)]
fn mul_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(core::hint::black_box(left) * core::hint::black_box(right))
}

#[inline(always)]
fn neg_f32(value: f32) -> f32 {
    f32::from_bits(value.to_bits() ^ SIGN_BIT)
}

#[inline(always)]
fn sub_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(core::hint::black_box(left) - core::hint::black_box(right))
}

#[inline(always)]
fn add_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(core::hint::black_box(left) + core::hint::black_box(right))
}

const SIGN_BIT: u32 = 0x8000_0000;
