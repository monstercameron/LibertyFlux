// original: 0x00DAD640 CTaskComplexTrackEntity::vf18

/// Select and queue a tracking task for an entity.
///
/// `this` is the task object. The method requires both a target reference at
/// `+0x14` and an entity pointer at `+0x08`; the entity's virtual kind query
/// must return `0xCB`. It then compares the square of the float at `+0x38`
/// with the float at `+0x70`. An unordered comparison follows the lower
/// branch, matching the original `comiss`/`jbe` behavior.
///
/// The upper branch requests a near tracking task using the weight at `+0x50`
/// and the payload at `+0x60`. The lower branch builds a position reference
/// from the target's `+0x20` slot and requests the other tracking task. A
/// non-null scene context queues the selected task; a failed eligibility
/// check uses the fallback task request. All helper calls preserve their
/// original thiscall argument order, including the ten-word task request.
///
/// The incoming request word is unused. Pointer fields and task records use
/// the original 32-bit layout.
lf_checker_rt::export!(thiscall, rw_00DAD640(this: u32, _request: u32) -> u32 {
    unsafe {
        const ENTITY_REFERENCE: u32 = 0x08;
        const TARGET_REFERENCE: u32 = 0x14;
        const RANGE_VALUE: u32 = 0x38;
        const TRACK_WEIGHT: u32 = 0x50;
        const TRACK_PAYLOAD: u32 = 0x60;
        const RANGE_LIMIT: u32 = 0x70;
        const TARGET_POSITION: u32 = 0x20;
        const TARGET_POSITION_FALLBACK: u32 = 0x10;
        const TARGET_POSITION_OFFSET: u32 = 0x30;
        const TARGET_ENTITY_KIND: u32 = 0xCB;
        const SCENE_SINGLETON_VA: u32 = 0x0167_E2A0;
        const NEAR_TASK: u32 = 1;
        const SCENE_CONTEXT: u32 = 2;
        const CREATE_NEAR_TASK: u32 = 3;
        const CREATE_POSITION_TASK: u32 = 4;
        const ENQUEUE_TASK: u32 = 5;
        const CREATE_FALLBACK_TASK: u32 = 6;
        const QUARTER: u32 = 0x3E80_0000;
        const HALF: u32 = 0x3F00_0000;
        const FIFTH: u32 = 0x3E4C_CCCD;
        const EIGHT: u32 = 0x4100_0000;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
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
        unsafe fn scene_context() -> u32 {
            let singleton = unsafe {
                lf_checker_rt::global::<u32>(SCENE_SINGLETON_VA).read_unaligned()
            };
            lf_checker_rt::callee_thiscall!(SCENE_CONTEXT, u32, singleton)
        }

        let target = read_u32(this.wrapping_add(TARGET_REFERENCE));
        if target == 0 {
            return 0;
        }
        let entity = read_u32(this.wrapping_add(ENTITY_REFERENCE));
        let eligible = entity != 0
            && lf_checker_rt::callee_thiscall!(NEAR_TASK, u32, entity)
                == TARGET_ENTITY_KIND;

        if eligible {
            let range = read_f32(this.wrapping_add(RANGE_VALUE));
            let range_squared = multiply(range, range);
            let range_limit = read_f32(this.wrapping_add(RANGE_LIMIT));
            let manager = scene_context();

            let task = if manager == 0 {
                0
            } else if range_squared > range_limit {
                let weight = read_f32(this.wrapping_add(TRACK_WEIGHT)).to_bits();
                lf_checker_rt::callee_thiscall!(
                    CREATE_NEAR_TASK,
                    u32,
                    manager,
                    weight,
                    this.wrapping_add(TRACK_PAYLOAD),
                    QUARTER,
                    0,
                    0
                )
            } else {
                let target_position = read_u32(target.wrapping_add(TARGET_POSITION));
                let position = if target_position == 0 {
                    target.wrapping_add(TARGET_POSITION_FALLBACK)
                } else {
                    target_position.wrapping_add(TARGET_POSITION_OFFSET)
                };
                lf_checker_rt::callee_thiscall!(
                    CREATE_POSITION_TASK,
                    u32,
                    manager,
                    3,
                    position,
                    HALF,
                    FIFTH,
                    u32::MAX,
                    1,
                    0,
                    0,
                    0,
                    1
                )
            };

            let queue = scene_context();
            if queue == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(
                    ENQUEUE_TASK, u32, queue, task, 0, 0, 0
                )
            }
        } else {
            let context = scene_context();
            if context == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(
                    CREATE_FALLBACK_TASK, u32, context, 0x32, 0, 0, EIGHT
                )
            }
        }
    }
});
