// original: 0x00C15C10 table_update_driver

use lf_checker_rt::{global, export};

const G_VECTOR_MASK: u32 = 0x00FE8FA0;
const G_VECTOR_SCALE: u32 = 0x00FE88E8;
const G_VECTOR_BIAS: u32 = 0x00FE8858;
const G_MAGNITUDE_LIMIT: u32 = 0x00FE8B08;
const G_CONTROL_MAX: u32 = 0x00EC43C8;
const G_CONTROL_MIN: u32 = 0x00EC43B0;
const G_VECTOR_LIMIT: u32 = 0x00FE8830;
const G_VECTOR_FLOOR: u32 = 0x00EC43C0;

const THIS_SELECTED_INDEX: usize = 0x2D4;
const THIS_VECTOR_BASE: usize = 0x2B0;
const THIS_SCALAR_PRIMARY: usize = 0x2C4;
const THIS_SCALAR_SECONDARY: usize = 0x2C8;
const THIS_SCALAR_VECTOR: usize = 0x2C0;
const THIS_FLAGS: usize = 0x39B;
const THIS_VECTOR_HELPER: usize = 0x10;
const THIS_TABLE_POINTER: usize = 0x60;
const THIS_VECTOR_X: usize = 0x40;
const THIS_VECTOR_Y: usize = 0x44;
const THIS_VECTOR_Z: usize = 0x48;
const THIS_VECTOR_W: usize = 0x4C;
const OBJECT_INACTIVE_MARKER: usize = 0x328C;

const CALLEE_OBJECT: u32 = 1;
const CALLEE_OBJECT_QUERY: u32 = 2;
const CALLEE_DOUBLE_SCALAR: u32 = 3;
const CALLEE_FLOAT_SCALAR: u32 = 4;
const CALLEE_VECTOR_SETTER: u32 = 5;

const INPUT_VECTOR_OFFSETS: [usize; 12] = [0x00, 0x04, 0x08, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28, 0x30, 0x34, 0x38];

#[inline(always)]
fn ordered_add(left: f32, right: f32) -> f32 {
    core::hint::black_box(left + right)
}

#[inline(always)]
fn ordered_sub(left: f32, right: f32) -> f32 {
    core::hint::black_box(left - right)
}

#[inline(always)]
fn ordered_mul(left: f32, right: f32) -> f32 {
    core::hint::black_box(left * right)
}

#[inline(always)]
fn ordered_sqrt(value: f32) -> f32 {
    core::hint::black_box(value.sqrt())
}

#[inline(always)]
unsafe fn read_f32(base: *const u8, offset: usize) -> f32 {
    unsafe { base.add(offset).cast::<f32>().read() }
}

#[inline(always)]
unsafe fn read_u32(base: *const u8, offset: usize) -> u32 {
    unsafe { base.add(offset).cast::<u32>().read() }
}

#[inline(always)]
unsafe fn write_f32(base: *mut u8, offset: usize, value: f32) {
    unsafe { base.add(offset).cast::<f32>().write(value) }
}

#[inline(always)]
unsafe fn write_u32(base: *mut u8, offset: usize, value: u32) {
    unsafe { base.add(offset).cast::<u32>().write(value) }
}

#[inline(always)]
fn global_f32(address: u32) -> f32 {
    unsafe { global::<f32>(address).read() }
}

#[inline(always)]
fn global_u32(address: u32) -> u32 {
    unsafe { global::<u32>(address).read() }
}

/// Performs the covered no-active-object path of the update routine.
///
/// Copies the caller's vector block, derives output coordinates against the
/// baseline block, obtains the scripted scalar results, clamps the secondary
/// control value, and publishes the vector and scalar fields on `this`.
/// The proof fixes the manager and override globals to null, the selected
/// index to -1, flag bytes to zero, and all vector inputs to zero. Other
/// manager, interpolation, status, and matrix-update paths remain uncovered.
#[inline(never)]
pub unsafe fn run_driver(
    this: *mut u8,
    input_vector: *const u8,
    baseline: *const u8,
    output_primary: *mut u8,
    output_secondary: *mut u8,
    _unused_mode: u32,
    _unused_factor: f32,
    _unused_record: *mut u8,
) {
    unsafe {
        let mut source = [0.0_f32; 12];
        for (slot, offset) in INPUT_VECTOR_OFFSETS.iter().copied().enumerate() {
            source[slot] = read_f32(input_vector, offset);
        }

        // The active-object selection is absent in this contract, so the
        // helper's optional vector override is also absent.
        let translated_z = ordered_add(source[11], global_f32(G_VECTOR_BIAS));
        let uninitialized_word = 0.0_f32;

        write_f32(output_primary, 0x00, source[9]);
        write_f32(output_primary, 0x04, source[10]);
        write_f32(output_primary, 0x08, translated_z);
        write_f32(output_primary, 0x0C, uninitialized_word);
        write_f32(output_secondary, 0x00, source[9]);
        write_f32(output_secondary, 0x04, source[10]);
        write_f32(output_secondary, 0x08, translated_z);
        write_f32(output_secondary, 0x0C, uninitialized_word);

        let baseline_x = read_f32(baseline, 0x10);
        let baseline_y = read_f32(baseline, 0x14);
        let baseline_z = read_f32(baseline, 0x18);
        let mut local_x = ordered_sub(read_f32(output_primary, 0x00), read_f32(baseline, 0x30));
        let mut local_y = ordered_sub(read_f32(output_primary, 0x04), read_f32(baseline, 0x34));
        let mut local_z = ordered_sub(read_f32(output_primary, 0x08), read_f32(baseline, 0x38));

        let squared_length = ordered_add(
            ordered_add(ordered_mul(baseline_x, baseline_x), ordered_mul(baseline_y, baseline_y)),
            ordered_mul(baseline_z, baseline_z),
        );
        let normalization = if squared_length != 0.0 {
            global_f32(G_VECTOR_SCALE) / ordered_sqrt(squared_length)
        } else {
            0.0
        };
        let normalized = [
            0.0_f32,
            ordered_mul(baseline_x, normalization),
            ordered_mul(baseline_y, normalization),
            ordered_mul(baseline_z, normalization),
        ];

        let existing_vector = read_u32(this, THIS_VECTOR_BASE);
        write_u32(this, THIS_TABLE_POINTER, existing_vector);

        // The queried object and its three status records are scripted by the
        // contract. Their status bytes route this state to the no-active path.
        let object = lf_checker_rt::callee_cdecl!(CALLEE_OBJECT, u32, 0_u32);
        let status_a = lf_checker_rt::callee_thiscall!(CALLEE_OBJECT_QUERY, u32, object);
        let status_b = lf_checker_rt::callee_thiscall!(CALLEE_OBJECT_QUERY, u32, object);
        let status_c = lf_checker_rt::callee_thiscall!(CALLEE_OBJECT_QUERY, u32, object);
        let a = status_a as *const u8;
        let b = status_b as *const u8;
        let c = status_c as *const u8;
        let _status_bits = (a.add(4).read() ^ a.add(6).read(), b.add(4).read() ^ b.add(6).read(), c.add(4).read() ^ c.add(6).read());
        let _inactive_marker = (object as *const u8).add(OBJECT_INACTIVE_MARKER).read();

        let mask = global_u32(G_VECTOR_MASK);
        let angle_input_x = f32::from_bits(baseline_x.to_bits() ^ mask) as f64;
        let angle_input_y = baseline_y as f64;
        let x_bits = angle_input_x.to_bits();
        let y_bits = angle_input_y.to_bits();
        let angle_answer = lf_checker_rt::callee_cdecl!(
            CALLEE_DOUBLE_SCALAR,
            u64,
            x_bits as u32,
            (x_bits >> 32) as u32,
            y_bits as u32,
            (y_bits >> 32) as u32,
        );
        let primary_scalar = f64::from_bits(angle_answer) as f32;
        write_f32(this, THIS_SCALAR_PRIMARY, primary_scalar);

        let secondary_scalar = lf_checker_rt::callee_cdecl!(
            CALLEE_FLOAT_SCALAR,
            f32,
            baseline_z.to_bits(),
        );
        write_f32(this, THIS_SCALAR_SECONDARY, secondary_scalar);

        // Match the original's ordered lower and upper comparisons.
        let mut clamped_secondary = read_f32(this, THIS_SCALAR_SECONDARY);
        let upper = global_f32(G_CONTROL_MAX);
        if !(upper > clamped_secondary) {
            let lower = global_f32(G_CONTROL_MIN);
            if clamped_secondary > lower {
                clamped_secondary = lower;
            }
        } else {
            clamped_secondary = upper;
        }
        write_f32(this, THIS_SCALAR_SECONDARY, clamped_secondary);

        let mut flags = unsafe { this.add(THIS_FLAGS).read() };
        let magnitude = ordered_sqrt(ordered_add(
            ordered_add(ordered_mul(local_x, local_x), ordered_mul(local_y, local_y)),
            ordered_mul(local_z, local_z),
        ));
        if flags & 0x02 != 0 {
            let limit = global_f32(G_MAGNITUDE_LIMIT);
            write_f32(this, THIS_SCALAR_VECTOR, if limit > magnitude { magnitude } else { limit });
        }

        flags &= 0xFD;
        unsafe { this.add(THIS_FLAGS).write(flags) };

        let mut vector_control = global_f32(G_VECTOR_LIMIT);
        let existing_control = read_f32(this, THIS_SCALAR_VECTOR);
        if !(vector_control > existing_control) {
            let floor = global_f32(G_VECTOR_FLOOR);
            vector_control = if existing_control > floor { floor } else { existing_control };
        }
        write_f32(this, THIS_SCALAR_VECTOR, vector_control);

        local_x = ordered_sub(read_f32(output_primary, 0x00), ordered_mul(vector_control, normalized[0]));
        local_y = ordered_sub(read_f32(output_primary, 0x04), ordered_mul(vector_control, normalized[1]));
        local_z = ordered_sub(read_f32(output_primary, 0x08), ordered_mul(vector_control, normalized[2]));

        let helper_vector = [normalized[0], normalized[1], normalized[2], 0.0_f32];
        let _ = lf_checker_rt::callee_thiscall!(
            CALLEE_VECTOR_SETTER,
            u32,
            this.add(THIS_VECTOR_HELPER) as u32,
            helper_vector.as_ptr() as u32,
        );

        write_f32(this, THIS_VECTOR_X, local_x);
        write_f32(this, THIS_VECTOR_Y, local_y);
        write_f32(this, THIS_VECTOR_Z, local_z);
        // The corresponding source slot is unwritten on this path and the
        // contract supplies a zero stack fill.
        write_f32(this, THIS_VECTOR_W, 0.0);
    }
}

export!(thiscall, rw_00c15c10(
    this: *mut u8,
    input_vector: *const u8,
    baseline: *const u8,
    output_primary: *mut u8,
    output_secondary: *mut u8,
    unused_mode: u32,
    unused_factor: f32,
    unused_record: *mut u8,
) -> () {
    unsafe {
        run_driver(this, input_vector, baseline, output_primary, output_secondary, unused_mode, unused_factor, unused_record)
    }
});
