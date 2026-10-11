// original: 0x00CCFAB0 ped_task_choose_nearest_candidate (proposed)

/// Select a nearby eligible task candidate from the owner's fixed sixteen-slot
/// table. `task` is the task object in ECX; `owner` is a pointer to an object
/// whose table pointer is at byte offset `OWNER_CANDIDATES`; `out_candidate`
/// points to a dword that receives the selected candidate when the task has
/// at least one matching entry. The task flag at `TASK_FLAGS` has bit 7
/// cleared on completion and on the no-match path.
///
/// Candidate pointers occupy sixteen dwords starting at `CANDIDATE_SLOTS` in
/// the table. Null entries are ignored. A candidate with a nonzero filter byte
/// at `CANDIDATE_FILTER` is ignored when its kind at `CANDIDATE_KIND` is 1 or
/// 2. The candidate must have a nonnull context at `CANDIDATE_CONTEXT`; the
/// context lookup and both owner lookups must resolve to the same owner.
/// A matched candidate is counted even when the status helper reports it as
/// inactive. The nearest candidate is selected by ordered single precision
/// squared distance, in y, x, z accumulation order, against the task's
/// reference position and the relocated global range limit. Unordered float
/// comparisons follow COMISS/JBE behavior: they do not replace the best or
/// farthest candidate. A null initial owner lookup returns zero. If no
/// candidate matches, the task flag is cleared and the task pointer is
/// returned without writing the output slot. Otherwise the nearest pointer
/// (possibly null when no distance is below the range limit) is written and
/// the output pointer is returned. The routine uses thiscall with two stack
/// arguments.
lf_checker_rt::export!(thiscall, rw_00ccfab0(task: u32, owner: u32, out_candidate: u32) -> u32 {
    const TASK_FLAGS: u32 = 0xF0;
    const OWNER_CANDIDATES: u32 = 0x224;
    const CANDIDATE_SLOTS: u32 = 0x168;
    const SLOT_COUNT: u32 = 16;
    const CANDIDATE_FILTER: u32 = 0x210;
    const CANDIDATE_KIND: u32 = 0xA74;
    const CANDIDATE_CONTEXT: u32 = 0x224;
    const ENTITY_POSITION: u32 = 0x20;
    const POSITION_X: u32 = 0x30;
    const TASK_RANGE_GLOBAL: u32 = 0x00FE8D18;
    const CLEAR_READY: u8 = 0x7F;
    const OWNER_CALLEE: u32 = 1;
    const CONTEXT_CALLEE: u32 = 2;
    const ACTIVE_CALLEE: u32 = 4;
    const TASK_STATE_CALLEE: u32 = 3;
    const FINISH_CALLEE: u32 = 5;

    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as usize as *const u8).read() }
    }
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as usize as *const u32).read_unaligned() }
    }
    unsafe fn write_u8(address: u32, value: u8) {
        unsafe { (address as usize as *mut u8).write(value) }
    }
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as usize as *mut u32).write_unaligned(value) }
    }
    unsafe fn read_f32(address: u32) -> f32 {
        f32::from_bits(unsafe { read_u32(address) })
    }
    unsafe fn owner_of(object: u32) -> u32 {
        unsafe { lf_checker_rt::callee_thiscall!(OWNER_CALLEE, u32, object) }
    }
    unsafe fn context_for(pointer: u32) -> u32 {
        unsafe { lf_checker_rt::callee_thiscall!(CONTEXT_CALLEE, u32, pointer) }
    }
    unsafe fn is_active(object: u32) -> u8 {
        unsafe { lf_checker_rt::callee_thiscall!(ACTIVE_CALLEE, u8, object) }
    }
    unsafe fn state_ok(object: u32) -> u8 {
        unsafe { lf_checker_rt::callee_thiscall!(TASK_STATE_CALLEE, u8, object) }
    }
    unsafe fn finish_task(object: u32) {
        let _: u32 = unsafe { lf_checker_rt::callee_thiscall!(FINISH_CALLEE, u32, object) };
    }
    #[inline(always)]
    fn sub_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    }
    #[inline(always)]
    fn mul_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }
    #[inline(always)]
    fn add_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }
    unsafe fn ordered_distance_sq(
        origin_x: f32,
        origin_y: f32,
        origin_z: f32,
        position: u32,
        origin_first: bool,
    ) -> f32 {
        let px = unsafe { read_f32(position + POSITION_X) };
        let py = unsafe { read_f32(position + POSITION_X + 4) };
        let pz = unsafe { read_f32(position + POSITION_X + 8) };
        let (dy, dx, dz) = if origin_first {
            (
                sub_f32(origin_y, py),
                sub_f32(origin_x, px),
                sub_f32(origin_z, pz),
            )
        } else {
            (
                sub_f32(py, origin_y),
                sub_f32(px, origin_x),
                sub_f32(pz, origin_z),
            )
        };
        let dy_sq = mul_f32(dy, dy);
        let dx_sq = mul_f32(dx, dx);
        let dz_sq = mul_f32(dz, dz);
        add_f32(add_f32(dy_sq, dx_sq), dz_sq)
    }
    unsafe fn clear_ready(task: u32) {
        let flags = unsafe { read_u8(task + TASK_FLAGS) } & CLEAR_READY;
        unsafe { write_u8(task + TASK_FLAGS, flags) };
    }

    unsafe {
        let initial_owner = owner_of(task);
        if initial_owner == 0 {
            clear_ready(task);
            return 0;
        }
        let task_owner = owner_of(task);
        let task_position = read_u32(task_owner + ENTITY_POSITION);
        let reference = if task_position != 0 {
            task_position.wrapping_add(POSITION_X)
        } else {
            task_owner.wrapping_add(0x10)
        };
        let origin_x = read_f32(reference);
        let origin_y = read_f32(reference + 4);
        let origin_z = read_f32(reference + 8);
        let range_limit = lf_checker_rt::global::<f32>(TASK_RANGE_GLOBAL).read_unaligned();

        let table = read_u32(owner + OWNER_CANDIDATES);
        let mut matched_count = 0u32;
        let mut inactive_count = 0u32;
        let mut nearest_distance = range_limit;
        let mut farthest_distance = 0.0f32;
        let mut nearest_candidate = 0u32;
        let mut farthest_context = 0u32;
        let mut last_context = 0u32;

        for slot in 0..SLOT_COUNT {
            let candidate = read_u32(table + CANDIDATE_SLOTS + slot * 4);
            if candidate == 0 {
                continue;
            }
            if read_u8(candidate + CANDIDATE_FILTER) != 0 {
                let kind = read_u32(candidate + CANDIDATE_KIND);
                if kind == 1 || kind == 2 {
                    continue;
                }
            }
            let candidate_context = read_u32(candidate + CANDIDATE_CONTEXT);
            if candidate_context == 0 {
                continue;
            }
            let context = context_for(candidate_context);
            if context == 0 {
                continue;
            }
            let candidate_owner = owner_of(context);
            let current_owner = owner_of(task);
            if candidate_owner != current_owner {
                continue;
            }

            last_context = context;
            matched_count = matched_count.wrapping_add(1);
            if state_ok(context) == 0 {
                inactive_count = inactive_count.wrapping_add(1);
            }
            let position = read_u32(candidate + ENTITY_POSITION);
            let distance = ordered_distance_sq(
                origin_x,
                origin_y,
                origin_z,
                position,
                false,
            );
            if nearest_distance > distance {
                nearest_distance = distance;
                nearest_candidate = candidate;
            }
            if distance > farthest_distance {
                farthest_distance = distance;
                farthest_context = context;
            }
        }

        let owner_is_active = is_active(owner) != 0;
        let minimum_active_count = 1 + u32::from(owner_is_active);
        if matched_count == 0 {
            clear_ready(task);
            return task;
        }
        if inactive_count < minimum_active_count {
            clear_ready(task);
            write_u32(out_candidate, nearest_candidate);
            return out_candidate;
        }

        let last_position = read_u32(last_context + ENTITY_POSITION);
        let late_distance = ordered_distance_sq(
            origin_x,
            origin_y,
            origin_z,
            last_position,
            true,
        );
        if !(nearest_distance > late_distance) {
            clear_ready(task);
            write_u32(out_candidate, nearest_candidate);
            return out_candidate;
        }
        if state_ok(last_context) == 0 && state_ok(nearest_candidate) == 0 {
            finish_task(farthest_context);
        }
        clear_ready(task);
        write_u32(out_candidate, nearest_candidate);
        out_candidate
    }
});
