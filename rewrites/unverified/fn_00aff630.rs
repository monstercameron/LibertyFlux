// original: 0x00AFF630 pool_cull_and_sort
/// Selects eligible pool rows, orders them by distance and priority, and
/// applies the per-row action to the farthest rows beyond the selected limit.
/// The candidate scan walks slots downward; equal-priority rows are ordered
/// from farther to nearer, while priority class 3 follows other classes.
/// Recent timestamps suppress an action, and the security-cookie check
/// preserves the caller-visible register state.
use lf_checker_rt::{callee_cdecl, callee_fastcall, callee_thiscall, export, global};

const MODE_GLOBAL: u32 = 0x0117_3604;
const POOL_GLOBAL: u32 = 0x012E_22A4;
const CLOCK_GLOBAL: u32 = 0x0117_35B4;
const DISTANCE_LIMIT: u32 = 0x00FE_8B38;
const SECURITY_COOKIE: u32 = 0x0105_7FB4;
const ROW_STRIDE: usize = 0x1310;
const ROW_CAPACITY: usize = 100;

#[inline(always)]
fn vcall_row_check(stub: u32, row: u32) -> u32 {
    let call: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(stub as usize) };
    call(row)
}

export!(cdecl, rw_aff630() -> u32 {
    unsafe { pool_cull_and_sort(false); 0 }
});

/// Wrong version for the proof contract: reverses the distance ordering.
export!(cdecl, mut_aff630() -> u32 {
    unsafe { pool_cull_and_sort(true); 0 }
});

unsafe fn pool_cull_and_sort(mutant: bool) {
    let mode = global::<u32>(MODE_GLOBAL).read() & 0x3F;
    if mode == 0x28 {
        let pool = global::<u32>(POOL_GLOBAL).read();
        let row_base = (pool as *const u32).read_unaligned();
        let used_slots = (pool.wrapping_add(4) as *const u32).read_unaligned();
        let mut slot_count = (pool.wrapping_add(8) as *const i32).read_unaligned();
        let stride = (pool.wrapping_add(12) as *const i32).read_unaligned();
        let mut candidate_rows = [0u32; ROW_CAPACITY];
        let mut candidate_distances = [0.0f32; ROW_CAPACITY];
        let mut eligible_count = 0i32;
        let mut candidate_count = 0usize;

        while slot_count != 0 {
            slot_count = slot_count.wrapping_sub(1);
            let slot_live = ((used_slots.wrapping_add(slot_count as u32)) as *const u8).read() & 0x80 == 0;
            if !slot_live {
                continue;
            }
            let row = row_base.wrapping_add(stride.wrapping_mul(slot_count) as u32);
            if row == 0 {
                continue;
            }

            let state = (row.wrapping_add(0x10B8) as *const u8).read();
            let type_flags = (row.wrapping_add(0x28) as *const u32).read_unaligned();
            let row_flags = (row.wrapping_add(0xF1F) as *const u8).read();
            let secondary_flags = (row.wrapping_add(0xF17) as *const u8).read();
            let has_supported_type = row_flags & 0x40 != 0
                || (type_flags & 0x7C00) == 0x0C00
                || secondary_flags & 0x08 != 0;
            if state == 2 || !has_supported_type {
                continue;
            }

            let first_reject = callee_cdecl!(1, u32,);
            if first_reject as u8 != 0 {
                continue;
            }
            let second_reject = callee_cdecl!(2, u32,);
            if second_reject as u8 != 0 {
                continue;
            }

            let vtable = (row as *const u32).read_unaligned();
            let row_check = vcall_row_check((vtable.wrapping_add(0x34) as *const u32).read_unaligned(), row);
            if row_check as u8 != 0 && candidate_count < ROW_CAPACITY {
                candidate_rows[candidate_count] = row;
                candidate_count += 1;
            }
            eligible_count = eligible_count.wrapping_add(1);
        }

        let mut keep_count = 10i32;
        if callee_cdecl!(4, u32,) as u8 != 0 {
            keep_count = 8;
        }
        if keep_count < eligible_count {
            let mut actions_left = eligible_count.wrapping_sub(keep_count);
            let mut player = [0.0f32; 3];
            callee_cdecl!(5, u32, player.as_mut_ptr());

            for i in 0..candidate_count {
                let row = candidate_rows[i];
                let matrix = (row.wrapping_add(0x20) as *const u32).read_unaligned();
                let row_x = ((matrix.wrapping_add(0x30)) as *const f32).read_unaligned();
                let row_y = ((matrix.wrapping_add(0x34)) as *const f32).read_unaligned();
                let row_z = ((matrix.wrapping_add(0x38)) as *const f32).read_unaligned();
                let dy = core::hint::black_box(player[1] - row_y);
                let dx = core::hint::black_box(player[0] - row_x);
                let dz = core::hint::black_box(player[2] - row_z);
                let xy = core::hint::black_box(core::hint::black_box(dy * dy) + core::hint::black_box(dx * dx));
                let xyz = core::hint::black_box(xy + core::hint::black_box(dz * dz));
                candidate_distances[i] = core::hint::black_box(xyz.sqrt());
            }

            loop {
                let mut swapped = false;
                if candidate_count < 2 {
                    break;
                }
                for i in 0..(candidate_count - 1) {
                    let left_row = candidate_rows[i];
                    let right_row = candidate_rows[i + 1];
                    let left_type = ((left_row.wrapping_add(0x28) as *const u32).read_unaligned() >> 10) & 0x1F;
                    let right_type = ((right_row.wrapping_add(0x28) as *const u32).read_unaligned() >> 10) & 0x1F;
                    let left_distance = candidate_distances[i];
                    let right_distance = candidate_distances[i + 1];
                    let distance_order = if mutant { left_distance > right_distance } else { left_distance < right_distance };
                    if (distance_order && left_type == right_type) || (left_type == 3 && right_type != 3) {
                        candidate_rows.swap(i, i + 1);
                        candidate_distances.swap(i, i + 1);
                        swapped = true;
                    }
                }
                if !swapped {
                    break;
                }
            }

            let current_time = global::<u32>(CLOCK_GLOBAL).read();
            let action_distance = global::<f32>(DISTANCE_LIMIT).read();
            for i in 0..candidate_count {
                if actions_left <= 0 {
                    break;
                }
                let row = candidate_rows[i];
                if action_distance < candidate_distances[i] && (callee_cdecl!(6, u32, 0) as u8) == 0 {
                    let has_component = (row.wrapping_add(0x6C) as *const u32).read_unaligned() != 0;
                    let range_rejected = if has_component { (callee_cdecl!(7, u32,) as u8) != 0 } else { false };
                    let type_flags = (row.wrapping_add(0x28) as *const u32).read_unaligned();
                    let recent = if (type_flags & 0x7C00) == 0x0C00 {
                        current_time.wrapping_sub((row.wrapping_add(0x12B8) as *const u32).read_unaligned()) < 10_000
                    } else {
                        let recently_seen = current_time.wrapping_sub((row.wrapping_add(0x12B0) as *const u32).read_unaligned()) < 10_000;
                        let child = (row.wrapping_add(0xF50) as *const u32).read_unaligned();
                        recently_seen && (child == 0 || (child.wrapping_add(0x211) as *const u8).read() == 0)
                    };
                    if !range_rejected && !recent {
                        callee_cdecl!(8, u32, row);
                        actions_left = actions_left.wrapping_sub(1);
                    }
                }
            }
        }
    }

    let cookie = global::<u32>(SECURITY_COOKIE).read();
    let _ = callee_fastcall!(9, u32, cookie);
}
