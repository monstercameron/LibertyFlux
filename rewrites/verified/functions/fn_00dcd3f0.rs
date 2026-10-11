// original: 0x00DCD3F0 CTaskSimpleReloadGun::process_ped
//! Validates the owning ped and weapon state, then advances a reload task through up to five state transitions.
//! A null weapon-info result faults on its first field read, matching the original invalid-input path. The routine records animation flags, runs the state-specific animation and attachment helpers, and updates the task timer with wrapping subtraction before a signed comparison.
#![allow(non_snake_case)]

const C_VALIDATE_WEAPON: u32 = 1;
const C_WEAPON_INFO: u32 = 2;
const C_PICK_RELOAD_ANIM: u32 = 3;
const C_SET_STATE: u32 = 4;
const C_SET_ANIM_TIME: u32 = 5;
const C_ATTACH_PED: u32 = 6;
const C_SHARED_READY: u32 = 7;
const C_REFRESH_ANIM: u32 = 8;
const C_CLEANUP: u32 = 9;

const PED_WEAPON_MANAGER: u32 = 0x2b0;
const PED_STATE_B: u32 = 0x219;
const WEAPON_INFO_ID: u32 = 0x18;
const WEAPON_INFO_MODE: u32 = 0x08;
const WEAPON_INFO_VALID: u32 = 0x0c;
const WEAPON_INFO_FLAGS: u32 = 0x20;

const TASK_CURRENT_ANIM: u32 = 0x48;
const TASK_TIMER: u32 = 0x4c;
const TASK_LOOP_CONTINUES: u32 = 0x50;
const TASK_STATE: u32 = 0x54;
const TASK_PREVIOUS_ANIM_FLAGS: u32 = 0x5c;
const TASK_FLAGS: u32 = 0x60;
const TASK_LAST_EVENT: u32 = 0x64;

const ANIM_DURATION: u32 = 0x40;
const ANIM_CURRENT_TIME: u32 = 0x4c;
const ANIM_NEXT_TIME: u32 = 0x70;
const ANIM_FLAGS: u32 = 0x74;
const UPDATE_GATE: u32 = 1 << 11;
const ATTACH_GATE: u32 = 1 << 29;
const UPDATE_EVENT: u32 = 2;
const TIME_THRESHOLD_GLOBAL: u32 = 0x00fe_88dc;

#[inline(always)]
unsafe fn rd32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn rd8(address: u32) -> u8 {
    unsafe { (address as *const u8).read() }
}

#[inline(always)]
unsafe fn wr8(address: u32, value: u8) {
    unsafe { (address as *mut u8).write(value) }
}

#[inline(always)]
unsafe fn rd_f32(address: u32) -> f32 {
    f32::from_bits(unsafe { rd32(address) })
}

/// Advance one reload-gun task update for a task object and its owning ped.
///
/// The task stores its current animation pointer at `+0x48`, timer at `+0x4c`,
/// loop-continuation byte at `+0x50`, state at `+0x54`, previous animation
/// flags at `+0x5c`, gating flags at `+0x60`, and event code at `+0x64`. The
/// ped's weapon manager is at `+0x2b0`; its weapon-info id is at `+0x18`.
/// Validation failure, missing required animation, or an absent weapon-info
/// bit triggers cleanup and returns true. Otherwise the routine executes up
/// to five state transitions: state 0 picks a reload animation and records an
/// event, state 1 resets its time, state 2 handles attachment and ordered
/// animation-time gates before decrementing the signed timer, and state 3
/// cleans up. State values above 3 leave the loop. The final result is true
/// only when the state is 3. Timer decrement wraps as a 32-bit word before its
/// signed comparison; floating comparisons are ordered, so NaN is false.
unsafe fn run_dcd3f0(task: u32, ped: u32) -> u32 {
    let weapon_manager: u32 = lf_checker_rt::callee_thiscall!(
        C_VALIDATE_WEAPON,
        u32,
        ped.wrapping_add(PED_WEAPON_MANAGER)
    );
    if weapon_manager == 0 {
        lf_checker_rt::callee_thiscall!(C_CLEANUP, u32, task, ped);
        return 1;
    }

    let weapon_info_id = unsafe { rd32(weapon_manager.wrapping_add(WEAPON_INFO_ID)) };
    let weapon_info: u32 =
        lf_checker_rt::callee_cdecl!(C_WEAPON_INFO, u32, weapon_info_id);
    if unsafe { rd32(weapon_info.wrapping_add(WEAPON_INFO_FLAGS)) } & (1 << 5) == 0 {
        lf_checker_rt::callee_thiscall!(C_CLEANUP, u32, task, ped);
        return 1;
    }

    let mut current_state = unsafe { rd32(task.wrapping_add(TASK_STATE)) };
    let mut animation = unsafe { rd32(task.wrapping_add(TASK_CURRENT_ANIM)) };
    if current_state != 0 && animation == 0 {
        lf_checker_rt::callee_thiscall!(C_CLEANUP, u32, task, ped);
        return 1;
    }

    unsafe {
        wr32(task.wrapping_add(TASK_PREVIOUS_ANIM_FLAGS), 0);
        if animation != 0 {
            wr32(
                task.wrapping_add(TASK_PREVIOUS_ANIM_FLAGS),
                rd32(animation.wrapping_add(ANIM_FLAGS)),
            );
        }
        wr8(task.wrapping_add(TASK_LOOP_CONTINUES), 1);
    }

    let mut iteration = 0u32;
    while iteration < 5 {
        unsafe { wr8(task.wrapping_add(TASK_LOOP_CONTINUES), 0) };
        current_state = unsafe { rd32(task.wrapping_add(TASK_STATE)) };
        if current_state > 3 {
            break;
        }

        match current_state {
            0 => {
                lf_checker_rt::callee_thiscall!(
                    C_PICK_RELOAD_ANIM,
                    u32,
                    task,
                    ped
                );
                unsafe { wr32(task.wrapping_add(TASK_LAST_EVENT), 0) };
                unsafe { wr32(weapon_manager.wrapping_add(0x1c), UPDATE_EVENT) };
                lf_checker_rt::callee_thiscall!(C_SET_STATE, u32, task, 1);
            }
            1 => {
                animation = unsafe { rd32(task.wrapping_add(TASK_CURRENT_ANIM)) };
                lf_checker_rt::callee_thiscall!(C_SET_ANIM_TIME, u32, animation, 0);
                lf_checker_rt::callee_thiscall!(C_SET_STATE, u32, task, 2);
            }
            2 => {
                animation = unsafe { rd32(task.wrapping_add(TASK_CURRENT_ANIM)) };
                let animation_flags = unsafe { rd32(animation.wrapping_add(ANIM_FLAGS)) };
                if animation_flags & UPDATE_GATE != 0 {
                    let info_id = unsafe { rd32(weapon_manager.wrapping_add(WEAPON_INFO_ID)) };
                    let info: u32 =
                        lf_checker_rt::callee_cdecl!(C_WEAPON_INFO, u32, info_id);
                    if unsafe { rd32(info.wrapping_add(WEAPON_INFO_MODE)) } == 3
                        && unsafe { rd32(info.wrapping_add(WEAPON_INFO_VALID)) } != 0
                    {
                        lf_checker_rt::callee_thiscall!(
                            C_ATTACH_PED,
                            u32,
                            ped.wrapping_add(PED_WEAPON_MANAGER),
                            ped
                        );
                    }
                }

                let mut carry_state_bit = 0u32;
                if lf_checker_rt::callee_cdecl!(C_SHARED_READY, u32,) & 0xff != 0
                    && unsafe { rd8(ped.wrapping_add(PED_STATE_B)) } != 0
                {
                    carry_state_bit = (animation_flags >> 1) & 1;
                }

                if unsafe { rd32(task.wrapping_add(TASK_FLAGS)) } & ATTACH_GATE == 0 {
                    let saved_time = unsafe { rd_f32(animation.wrapping_add(ANIM_CURRENT_TIME)) };
                    let duration_word = unsafe { rd32(animation.wrapping_add(ANIM_DURATION)) };
                    lf_checker_rt::callee_thiscall!(
                        C_REFRESH_ANIM,
                        u32,
                        animation,
                        duration_word
                    );
                    let next_time = unsafe { rd_f32(animation.wrapping_add(ANIM_NEXT_TIME)) };
                    if saved_time > next_time {
                        carry_state_bit = 1;
                    }
                }

                let time_threshold = f32::from_bits(unsafe {
                    lf_checker_rt::global::<u32>(TIME_THRESHOLD_GLOBAL).read_unaligned()
                });
                let current_time = unsafe { rd_f32(animation.wrapping_add(ANIM_CURRENT_TIME)) };
                if current_time >= time_threshold || carry_state_bit != 0 {
                    let remaining = unsafe { rd32(task.wrapping_add(TASK_TIMER)) }.wrapping_sub(1);
                    unsafe { wr32(task.wrapping_add(TASK_TIMER), remaining) };
                    if (remaining as i32) <= 0 {
                        unsafe { wr32(task.wrapping_add(TASK_LAST_EVENT), UPDATE_EVENT) };
                        lf_checker_rt::callee_thiscall!(C_SET_STATE, u32, task, 3);
                    } else {
                        lf_checker_rt::callee_thiscall!(C_SET_STATE, u32, task, 1);
                    }
                }
            }
            3 => {
                lf_checker_rt::callee_thiscall!(C_CLEANUP, u32, task, ped);
            }
            _ => unreachable!(),
        }

        iteration += 1;
        if unsafe { rd8(task.wrapping_add(TASK_LOOP_CONTINUES)) } == 0 {
            break;
        }
    }

    current_state = unsafe { rd32(task.wrapping_add(TASK_STATE)) };
    let completed = u32::from(current_state == 3);
    completed
}

lf_checker_rt::export!(thiscall, rw_00dcd3f0(task: u32, ped: u32) -> u32 {
    unsafe { run_dcd3f0(task, ped) }
});
