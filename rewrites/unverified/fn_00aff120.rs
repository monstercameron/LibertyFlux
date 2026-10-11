// original: 0x00AFF120 input_master_pass
/// Runs the vehicle master pass after the session and mode gates permit it.
/// It obtains the active position data, selects the mode's range, invokes the
/// pool and placement passes, runs optional feature hooks, and clears the
/// completion flag. The supplied global index selects a four-word data row;
/// a negative index leaves the local position values at their initialized
/// zero values.
use lf_checker_rt::{callee_cdecl, export, global};

const SESSION_GATE: u32 = 0x011F_7060;
const SESSION_VALUE: u32 = 0x0120_88B4;
const GAME_MODE: u32 = 0x0103_7720;
const DATA_INDEX: u32 = 0x0160_031C;
const DATA_TABLE: u32 = 0x0160_0FD0;
const DEFAULT_RANGE: u32 = 0x00FE_88E8;
const RANGE_FACTOR: u32 = 0x0103_FF50;
const RANGE_ONE: u32 = 0x0103_FF54;
const RANGE_TWO: u32 = 0x0103_FF58;
const DISTANCE_LIMIT: u32 = 0x00FE_8B34;
const FEATURE_WORD: u32 = 0x0103_FF74;
const COMPLETION_FLAG: u32 = 0x012E_21F0;

export!(cdecl, rw_aff120() -> u32 {
    unsafe { input_master_pass(false) }
});

// Wrong version for the proof contract: leaves the completion flag unchanged.
export!(cdecl, mut_aff120() -> u32 {
    unsafe { input_master_pass(true) }
});

unsafe fn input_master_pass(mutant: bool) -> u32 {
    if global::<u32>(SESSION_GATE).read() == 1 {
        return 0;
    }

    let session_value = global::<u32>(SESSION_VALUE).read();
    if session_value != u32::MAX || global::<u32>(GAME_MODE).read() == 0x12 {
        return session_value;
    }

    let first_gate: u32 = callee_cdecl!(1, u32,);
    if first_gate as u8 != 0 {
        return first_gate;
    }
    let second_gate: u32 = callee_cdecl!(2, u32,);
    if second_gate as u8 != 0 {
        return second_gate;
    }

    let _: u32 = callee_cdecl!(3, u32,);
    let _setup: u32 = callee_cdecl!(4, u32,);

    let mut entity_values = [0u32; 6];
    let mut position = [0.0f32; 2];
    let mut auxiliary_values = [0u32; 3];
    let mut position_valid = [0u8; 4];
    let _query: u32 = callee_cdecl!(
        5,
        u32,
        entity_values.as_mut_ptr().add(2) as u32,
        entity_values.as_mut_ptr().add(1) as u32,
        entity_values.as_mut_ptr() as u32,
        position.as_mut_ptr() as u32,
        auxiliary_values.as_mut_ptr() as u32,
        position_valid.as_mut_ptr() as u32,
    );

    let data_index = global::<i32>(DATA_INDEX).read();
    if data_index >= 0 {
        let row = DATA_TABLE.wrapping_add((data_index as u32).wrapping_mul(0x30));
        for word in 0..4usize {
            entity_values[2 + word] =
                global::<u32>(row.wrapping_add((word as u32).wrapping_mul(4))).read();
        }
    }

    let mut base_range = global::<f32>(DEFAULT_RANGE).read();
    if entity_values[0] != 0 {
        if entity_values[0] == 1 {
            base_range = global::<f32>(RANGE_ONE).read();
        } else if entity_values[0] == 2 {
            base_range = global::<f32>(RANGE_TWO).read();
        }
    }
    let range_factor = global::<f32>(RANGE_FACTOR).read();
    let range = core::hint::black_box(range_factor) * core::hint::black_box(base_range);

    let distant = if position_valid[0] == 0 {
        0
    } else {
        let x = core::hint::black_box(position[0]);
        let y = core::hint::black_box(position[1]);
        let x_squared = core::hint::black_box(x * x);
        let y_squared = core::hint::black_box(y * y);
        let squared_distance = core::hint::black_box(y_squared + x_squared);
        let distance = core::hint::black_box(squared_distance).sqrt();
        (distance > global::<f32>(DISTANCE_LIMIT).read()) as u32
    };

    let _range_pass: u32 = callee_cdecl!(
        6,
        u32,
        entity_values.as_mut_ptr().add(2) as u32,
        range.to_bits(),
        distant,
    );
    let _pool_pass: u32 = callee_cdecl!(7, u32,);
    let _cull_pass: u32 = callee_cdecl!(8, u32,);
    let _sort_pass: u32 = callee_cdecl!(9, u32,);

    let feature_flags = global::<u32>(FEATURE_WORD).read();
    if ((feature_flags >> 16) & 0xFF) != 0 {
        let _optional_range: u32 = callee_cdecl!(10, u32, range.to_bits());
    }
    if ((feature_flags >> 24) & 0xFF) != 0 {
        let _optional_data: u32 = callee_cdecl!(
            11,
            u32,
            auxiliary_values.as_mut_ptr() as u32,
            0,
            range.to_bits(),
        );
    }

    let final_gate: u32 = callee_cdecl!(1, u32,);
    let result = if final_gate as u8 != 0 {
        final_gate
    } else {
        let _placement: u32 = callee_cdecl!(
            12,
            u32,
            entity_values.as_mut_ptr().add(2) as u32,
            position.as_mut_ptr() as u32,
            range.to_bits(),
        );
        callee_cdecl!(
            13,
            u32,
            entity_values.as_mut_ptr().add(2) as u32,
            auxiliary_values.as_mut_ptr() as u32,
            range.to_bits(),
        )
    };

    if !mutant {
        global::<u32>(COMPLETION_FLAG).write(0);
    }
    result
}
