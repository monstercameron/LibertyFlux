// original: 0x00CEA2B0 peds_task_update
#![allow(unsafe_code)]
use lf_checker_rt::{callee_cdecl, callee_thiscall};

const MESSAGE_BYTES: usize = 0xCC4;

// Callee table ids used by the checker.
const CALLEE_MSG_INIT: u32 = 1;
const CALLEE_ADD_INT: u32 = 2;
const CALLEE_PED_IN_WINDOW: u32 = 3;
const CALLEE_RAND: u32 = 4;
const CALLEE_ADD_FLOAT: u32 = 5;
const CALLEE_ADD_VECTOR: u32 = 6;
const CALLEE_ADD_SCALAR: u32 = 7;
const CALLEE_SEND: u32 = 8;
const CALLEE_MSG_RESET: u32 = 9;
const CALLEE_NORMALISE_FIRST: u32 = 10;
const CALLEE_TRAJECTORY: u32 = 11;
const CALLEE_SCALED_VECTOR: u32 = 12;
const CALLEE_WEAPON_INFO: u32 = 13;
const CALLEE_TIMER_OFFSET: u32 = 14;
const CALLEE_NOTIFY: u32 = 15;
const CALLEE_NORMALISE_SECOND: u32 = 16;

// Task fields.
const TASK_THRESHOLD_LOW: u32 = 0x14;
const TASK_TIMER_BASE: u32 = 0x10;
const TASK_NEXT_TIME: u32 = 0x18;
const TASK_STAGE: u32 = 0x24;
const TASK_READY: u32 = 0x29;

// Ped fields.
const PED_FLAGS: u32 = 0x270;
const PED_BIT18: u32 = 1 << 18;
const PED_SUB_OBJECT: u32 = 0x2C4;
const PED_BLOCK_BYTE: u32 = 0x211;
const PED_MESSAGE_TARGET: u32 = 0x7B4;

// Owner-info object fields.
const SUB_ARG: u32 = 0x20;
const SUB_INFO: u32 = 0x25C;

// Info object fields.
const INFO_BLOCK: u32 = 0x1C;
const INFO_WEAPON_KEY: u32 = 0x18;

// Weapon record field.
const WEAPON_OFFSET: u32 = 0x8C;

// Image addresses (file VA) the function loads from.
const RUNNING_TIME_VA: u32 = 0x0117_35B4;
const MSG_OPEN_VA: u32 = 0x0105_1CC8;
const MSG_ID_VA: u32 = 0x0105_216C;
const MSG_VECTOR_NAME_VA: u32 = 0x0105_2174;
const MSG_FIRST_FLOAT_NAME_VA: u32 = 0x0105_2178;
const MSG_SECOND_FLOAT_NAME_VA: u32 = 0x0105_217C;
const MSG_SCALAR_NAME_VA: u32 = 0x0105_218C;
const MSG_ONE_NAME_VA: u32 = 0x0105_2190;
const MSG_COUNT_NAME_VA: u32 = 0x0105_2194;
const RAND_SCALE_VA: u32 = 0x00FE_8684;
const FIRST_SCALE_IN_WINDOW_VA: u32 = 0x00FE_8858;
const FIRST_BASE_IN_WINDOW_VA: u32 = 0x00FE_881C;
const FIRST_SCALE_OUT_WINDOW_VA: u32 = 0x00ED_D73C;
const FIRST_BASE_OUT_WINDOW_VA: u32 = 0x00ED_D738;
const SECOND_SCALE_VA: u32 = 0x00EA_4D2C;
const SECOND_BASE_VA: u32 = 0x00FE_888C;
const DOT_LIMIT_VA: u32 = 0x00FE_8898;
const ONE_BITS: u32 = 0x3F80_0000;
const MINUS_ONE_BITS: u32 = 0xBF80_0000;

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_u8(address: u32) -> u8 {
    unsafe { (address as *const u8).read() }
}

#[inline(always)]
unsafe fn write_u8(address: u32, value: u8) {
    unsafe { (address as *mut u8).write(value) }
}

#[inline(always)]
unsafe fn read_f32(address: u32) -> f32 {
    f32::from_bits(unsafe { read_u32(address) })
}

#[inline(always)]
fn multiply(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) * core::hint::black_box(right)
}

#[inline(always)]
fn add(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

#[inline(always)]
fn subtract(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) - core::hint::black_box(right)
}

/// Builds and sends the main message: a first float from a scaled random
/// value (its scale and base depend on the ped's window test), a second
/// float from another scaled random value, then the vector, the scalar, a
/// one and a count. The send sets the task's ready byte.
unsafe fn send_main_message(task: u32, ped: u32, vector: u32, scalar: u32) {
    let mut message = [0u8; MESSAGE_BYTES];
    let message_ptr = message.as_mut_ptr() as u32;
    unsafe {
        let _: u32 = callee_thiscall!(CALLEE_MSG_INIT, u32, message_ptr);
        let _: u32 = callee_thiscall!(
            CALLEE_ADD_INT,
            u32,
            message_ptr,
            read_u32(lf_checker_rt::relocated(MSG_OPEN_VA)),
            1
        );

        let in_window: u8 = callee_thiscall!(CALLEE_PED_IN_WINDOW, u8, ped);
        let random_first = callee_cdecl!(CALLEE_RAND, u32,) as i32 as f32;
        let first_value = if in_window != 0 {
            subtract(
                multiply(
                    multiply(random_first, read_f32(lf_checker_rt::relocated(RAND_SCALE_VA))),
                    read_f32(lf_checker_rt::relocated(FIRST_SCALE_IN_WINDOW_VA)),
                ),
                read_f32(lf_checker_rt::relocated(FIRST_BASE_IN_WINDOW_VA)),
            )
        } else {
            subtract(
                multiply(
                    multiply(random_first, read_f32(lf_checker_rt::relocated(RAND_SCALE_VA))),
                    read_f32(lf_checker_rt::relocated(FIRST_SCALE_OUT_WINDOW_VA)),
                ),
                read_f32(lf_checker_rt::relocated(FIRST_BASE_OUT_WINDOW_VA)),
            )
        };
        let _: u32 = callee_thiscall!(
            CALLEE_ADD_FLOAT,
            u32,
            message_ptr,
            read_u32(lf_checker_rt::relocated(MSG_FIRST_FLOAT_NAME_VA)),
            first_value.to_bits()
        );

        let random_second = callee_cdecl!(CALLEE_RAND, u32,) as i32 as f32;
        let second_value = add(
            multiply(
                multiply(random_second, read_f32(lf_checker_rt::relocated(RAND_SCALE_VA))),
                read_f32(lf_checker_rt::relocated(SECOND_SCALE_VA)),
            ),
            read_f32(lf_checker_rt::relocated(SECOND_BASE_VA)),
        );
        let _: u32 = callee_thiscall!(
            CALLEE_ADD_FLOAT,
            u32,
            message_ptr,
            read_u32(lf_checker_rt::relocated(MSG_SECOND_FLOAT_NAME_VA)),
            second_value.to_bits()
        );

        let _: u32 = callee_thiscall!(
            CALLEE_ADD_VECTOR,
            u32,
            message_ptr,
            read_u32(lf_checker_rt::relocated(MSG_VECTOR_NAME_VA)),
            read_u32(vector),
            read_u32(vector.wrapping_add(4)),
            read_u32(vector.wrapping_add(8))
        );
        let _: u32 = callee_thiscall!(
            CALLEE_ADD_SCALAR,
            u32,
            message_ptr,
            read_u32(lf_checker_rt::relocated(MSG_SCALAR_NAME_VA)),
            scalar
        );
        let _: u32 = callee_thiscall!(
            CALLEE_ADD_FLOAT,
            u32,
            message_ptr,
            read_u32(lf_checker_rt::relocated(MSG_ONE_NAME_VA)),
            ONE_BITS
        );
        let _: u32 = callee_thiscall!(
            CALLEE_ADD_INT,
            u32,
            message_ptr,
            read_u32(lf_checker_rt::relocated(MSG_COUNT_NAME_VA)),
            1
        );
        let target = read_u32(ped.wrapping_add(PED_MESSAGE_TARGET));
        let _: u32 = callee_thiscall!(
            CALLEE_SEND,
            u32,
            target,
            read_u32(lf_checker_rt::relocated(MSG_ID_VA)),
            message_ptr
        );
        write_u8(task.wrapping_add(TASK_READY), 1);
        let _: u32 = callee_thiscall!(CALLEE_MSG_RESET, u32, message_ptr);
    }
}

/// Builds and sends the vector fallback message: one vector, no ready change.
unsafe fn send_vector_message(ped: u32, vector: u32) {
    let mut message = [0u8; MESSAGE_BYTES];
    let message_ptr = message.as_mut_ptr() as u32;
    unsafe {
        let _: u32 = callee_thiscall!(CALLEE_MSG_INIT, u32, message_ptr);
        let _: u32 = callee_thiscall!(
            CALLEE_ADD_VECTOR,
            u32,
            message_ptr,
            read_u32(lf_checker_rt::relocated(MSG_VECTOR_NAME_VA)),
            read_u32(vector),
            read_u32(vector.wrapping_add(4)),
            read_u32(vector.wrapping_add(8))
        );
        let target = read_u32(ped.wrapping_add(PED_MESSAGE_TARGET));
        let _: u32 = callee_thiscall!(
            CALLEE_SEND,
            u32,
            target,
            read_u32(lf_checker_rt::relocated(MSG_ID_VA)),
            message_ptr
        );
        let _: u32 = callee_thiscall!(CALLEE_MSG_RESET, u32, message_ptr);
    }
}

/// Stage-two part: the scaled vector routine fills two vectors, each is
/// differenced and normalised, the dot product of the directions is compared
/// with an image limit, and the trajectory call fires only above it. The
/// weapon record's offset and the running time are then added to a timer
/// base, and the ped is notified.
unsafe fn update_stage_two(task: u32, ped: u32, sub_object: u32, second_vector: u32) {
    unsafe {
        let mut out_second = [0u32; 3];
        let mut out_first = [0u32; 3];
        let out_second_ptr = out_second.as_mut_ptr() as u32;
        let out_first_ptr = out_first.as_mut_ptr() as u32;
        let sub_arg = read_u32(sub_object.wrapping_add(SUB_ARG));
        let info = read_u32(sub_object.wrapping_add(SUB_INFO));

        // The scaled-vector routine writes the second output, then the first.
        let _: u32 = callee_thiscall!(
            CALLEE_SCALED_VECTOR,
            u32,
            info,
            ped,
            sub_arg,
            out_second_ptr,
            out_first_ptr
        );

        // First output minus second output, normalised.
        let mut difference = [0u32; 3];
        for i in 0..3usize {
            let first = f32::from_bits(out_first[i]);
            let second = f32::from_bits(out_second[i]);
            difference[i] = subtract(first, second).to_bits();
        }
        let mut direction = [0u32; 3];
        let _: u32 = callee_thiscall!(
            CALLEE_NORMALISE_FIRST,
            u32,
            direction.as_mut_ptr() as u32,
            difference.as_ptr() as u32
        );

        // Second vector minus the second output, normalised.
        let mut offset = [0u32; 3];
        for i in 0..3u32 {
            let input = f32::from_bits(read_u32(second_vector.wrapping_add(4 * i)));
            let second = f32::from_bits(out_second[i as usize]);
            offset[i as usize] = subtract(input, second).to_bits();
        }
        let mut second_direction = [0u32; 3];
        let _: u32 = callee_thiscall!(
            CALLEE_NORMALISE_SECOND,
            u32,
            second_direction.as_mut_ptr() as u32,
            offset.as_ptr() as u32
        );

        // Dot product, summed left to right with the operand order pinned.
        let ax = f32::from_bits(direction[0]);
        let ay = f32::from_bits(direction[1]);
        let az = f32::from_bits(direction[2]);
        let bx = f32::from_bits(second_direction[0]);
        let by = f32::from_bits(second_direction[1]);
        let bz = f32::from_bits(second_direction[2]);
        let dot = add(add(multiply(ax, bx), multiply(ay, by)), multiply(az, bz));
        let dot_limit = read_f32(lf_checker_rt::relocated(DOT_LIMIT_VA));
        if dot > dot_limit {
            let _: u32 = callee_thiscall!(
                CALLEE_TRAJECTORY,
                u32,
                info,
                ped,
                sub_arg,
                out_second_ptr,
                out_first_ptr,
                0,
                0,
                0,
                0,
                MINUS_ONE_BITS
            );
        }

        // Running time is read before the weapon lookup, as the original does.
        let running = read_u32(lf_checker_rt::relocated(RUNNING_TIME_VA));
        let weapon_key = read_u32(info.wrapping_add(INFO_WEAPON_KEY));
        let weapon = callee_cdecl!(CALLEE_WEAPON_INFO, u32, weapon_key);
        let weapon_value = read_u32(weapon.wrapping_add(WEAPON_OFFSET));
        let timer_base = read_u32(task.wrapping_add(TASK_TIMER_BASE));
        let timer_offset = callee_cdecl!(CALLEE_TIMER_OFFSET, u32, 0, timer_base);
        write_u32(
            task.wrapping_add(TASK_NEXT_TIME),
            timer_offset.wrapping_add(weapon_value).wrapping_add(running),
        );

        let _: u32 = callee_thiscall!(CALLEE_NOTIFY, u32, ped, 0xFFFF_FFFF);
    }
}

/// Advances a ped's natural-motion task (peds subsystem) by one update.
///
/// `task` is the task, `ped` the ped, `vector` and `second_vector` pointers to
/// three-float vectors, and `scalar` a float passed as its bit pattern.
///
/// Part one runs for any task stage other than zero. When the ped's bit 18
/// flag is clear it sends one message to the ped's message target: the full
/// main message when the task is not yet ready and the running time has
/// reached the task's low threshold (the send then sets the ready byte), and
/// otherwise a vector fallback message that leaves the ready byte alone.
/// When the bit is set, part one is skipped.
///
/// Part two runs only for stage two. It is skipped when the running time is
/// below the task's next-time word (unsigned), when the ped's owner-info
/// pointer is null, when the ped's block byte is set, or when the info
/// object's block word is non-zero. Otherwise it runs the stage-two update.
/// The owner-info pointer is read before any call.
///
/// The callees are the game's own message, random, vector, trajectory and
/// timer routines, reached through the checker's interception table. The
/// stack cookie check at exit belongs to the original's own code.
unsafe fn update_task(task: u32, ped: u32, vector: u32, second_vector: u32, scalar: u32) {
    unsafe {
        let sub_object = read_u32(ped.wrapping_add(PED_SUB_OBJECT));
        let stage = read_u32(task.wrapping_add(TASK_STAGE));
        if stage == 0 {
            return;
        }

        if read_u32(ped.wrapping_add(PED_FLAGS)) & PED_BIT18 == 0 {
            let ready = read_u8(task.wrapping_add(TASK_READY));
            let running = read_u32(lf_checker_rt::relocated(RUNNING_TIME_VA));
            if ready == 0 && running >= read_u32(task.wrapping_add(TASK_THRESHOLD_LOW)) {
                send_main_message(task, ped, vector, scalar);
            } else {
                send_vector_message(ped, vector);
            }
        }

        if stage != 2 {
            return;
        }
        let running = read_u32(lf_checker_rt::relocated(RUNNING_TIME_VA));
        if running < read_u32(task.wrapping_add(TASK_NEXT_TIME)) {
            return;
        }
        if sub_object == 0 {
            return;
        }
        if read_u8(ped.wrapping_add(PED_BLOCK_BYTE)) != 0 {
            return;
        }
        let info = read_u32(sub_object.wrapping_add(SUB_INFO));
        if read_u32(info.wrapping_add(INFO_BLOCK)) != 0 {
            return;
        }
        update_stage_two(task, ped, sub_object, second_vector);
    }
}

lf_checker_rt::export!(thiscall, rw_00cea2b0(this: u32, owner: u32, vector: u32, second_vector: u32, scalar: u32) -> u32 {
    unsafe { update_task(this, owner, vector, second_vector, scalar) }
    0
});
