// original: 0x00BC3220 CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorRadiusAngleOffset>::vf18

/// Handle one event for a move-seek task using x86 thiscall (`this` in ECX,
/// event code and request pointer on the stack). The signed event-code switch
/// handles `0x386`, `0x387`, `0x11a`, `0x3ae`, and the idle case `0x516`.
/// For active targets it refreshes the target state, optionally copies the
/// entity position to task offset `0x30`, otherwise takes the returned
/// position from the position helper, and compares planar squared distance
/// with squared radius (`+0x48`) before comparing against the threshold (`+0x50`).
/// Ordered float operations follow the original SSE operand order; unordered
/// distance comparisons take the original `jbe` path. Event and vtable calls
/// are forwarded with their original arguments. On a far `0x3ae` response,
/// it sets response offsets `+0x60`, `+0x64`, and flag word `+0xd8`, then
/// invokes the two response configuration methods. It returns the selected
/// callback result or response object in EAX.unsafe fn handle_seek_task_event(this: u32, request: u32, mutate_near: bool) -> u32 {
    const TASK_ACTIVE_TARGET: u32 = 0x20;
    const TASK_TARGET_ENTITY: u32 = 0x08;
    const TASK_POSITION: u32 = 0x30;
    const TASK_RADIUS: u32 = 0x48;
    const TASK_THRESHOLD: u32 = 0x50;
    const TASK_FLAGS: u32 = 0x80;
    const TARGET_ENTITY_FLAGS: u32 = 0xD8;
    const TARGET_ENTITY_POSITION: u32 = 0x40;
    const REQUEST_TARGET: u32 = 0x20;
    const REQUEST_KIND: u32 = 0x1E2;
    const EVENT_IDLE: u32 = 0x516;
    const EVENT_READY: u32 = 0x11A;
    const EVENT_NEAR: u32 = 0x386;
    const EVENT_APPROACH: u32 = 0x387;
    const EVENT_COMPLETE: u32 = 0x3AE;
    const TARGET_READY_BIT: u32 = 1;
    const TASK_CAN_COPY_TARGET: u8 = 0x08;
    const TASK_HAS_NEAR_ACTION: u8 = 0x01;
    const TASK_NEAR_ACTION_SET: u8 = 0x02;
    const RESPONSE_DURATION: f32 = f32::from_bits(0x4080_0000);
    const RESPONSE_DISTANCE: f32 = f32::from_bits(0xC47A_0000);
    const ZERO: f32 = f32::from_bits(0);

    unsafe {
        if read_u32(this, TASK_ACTIVE_TARGET) == 0 {
            return lf_checker_rt::callee_thiscall!(14, u32, this, EVENT_IDLE, request);
        }

        let target_entity = read_u32(this, TASK_TARGET_ENTITY);
        let target_state = lf_checker_rt::callee_thiscall!(1, u32, target_entity);
        if target_state == EVENT_NEAR {
            return lf_checker_rt::callee_thiscall!(14, u32, this, EVENT_IDLE, request);
        }

        if target_state == EVENT_READY {
            let request_kind = read_u8(request, REQUEST_KIND) & 0x0F;
            if request_kind >= 2 {
                return lf_checker_rt::callee_thiscall!(8, u32, this, EVENT_READY, request);
            }
            return lf_checker_rt::callee_thiscall!(2, u32, this);
        }

        if target_state != EVENT_APPROACH && target_state != EVENT_COMPLETE {
            return 0;
        }

        let refreshed_state = lf_checker_rt::callee_thiscall!(1, u32, target_entity);
        if refreshed_state == EVENT_COMPLETE
            && (read_u32(target_entity, TARGET_ENTITY_FLAGS) >> 14) & TARGET_READY_BIT != 0
            && read_u8(this, TASK_FLAGS) & TASK_CAN_COPY_TARGET != 0
        {
            for word in 0..3u32 {
                write_u32(
                    this,
                    TASK_POSITION + word * 4,
                    read_u32(target_entity, TARGET_ENTITY_POSITION + word * 4),
                );
            }
        } else {
            // The native position helper receives a local out buffer but
            // returns the position object consumed by the caller.
            let mut unused_out_words = [0u32; 4];
            let position = lf_checker_rt::callee_thiscall!(
                3,
                u32,
                this,
                unused_out_words.as_mut_ptr() as u32,
                request
            );
            for word in 0..4u32 {
                write_u32(this, TASK_POSITION + word * 4, read_u32(position, word * 4));
            }
        }

        let target_position = read_u32(request, REQUEST_TARGET);
        let delta_x = sub_f32(
            read_f32(target_position, 0x30),
            read_f32(this, TASK_POSITION),
        );
        let delta_y = sub_f32(
            read_f32(target_position, 0x34),
            read_f32(this, TASK_POSITION + 4),
        );
        let distance_squared = add_f32(
            mul_f32(delta_y, delta_y),
            mul_f32(delta_x, delta_x),
        );
        let radius = read_f32(this, TASK_RADIUS);
        let radius_squared = mul_f32(radius, radius);

        if !(distance_squared > radius_squared) {
            let flags = read_u8(this, TASK_FLAGS);
            write_u8(this, TASK_FLAGS, flags | TASK_NEAR_ACTION_SET);
            if flags & TASK_HAS_NEAR_ACTION != 0 {
                if mutate_near {
                    return lf_checker_rt::callee_thiscall!(10, u32, this, EVENT_APPROACH, request);
                }
                return lf_checker_rt::callee_thiscall!(13, u32, this, EVENT_NEAR, request);
            }
            return lf_checker_rt::callee_thiscall!(14, u32, this, EVENT_IDLE, request);
        }

        let threshold = read_f32(this, TASK_THRESHOLD);
        if threshold == ZERO {
            return lf_checker_rt::callee_thiscall!(11, u32, this, EVENT_COMPLETE, request);
        }

        let threshold_squared = mul_f32(threshold, threshold);
        if !(distance_squared > threshold_squared) {
            return lf_checker_rt::callee_thiscall!(10, u32, this, EVENT_APPROACH, request);
        }

        let response = lf_checker_rt::callee_thiscall!(9, u32, this, EVENT_COMPLETE, request);
        finish_seek_response(this, request, response, RESPONSE_DURATION, RESPONSE_DISTANCE)
    }
}

unsafe fn finish_seek_response(
    this: u32,
    request: u32,
    response: u32,
    duration: f32,
    distance: f32,
) -> u32 {
    const RESPONSE_VTABLE_SUBOBJECT: u32 = 0x14;
    const RESPONSE_DURATION_FIELD: u32 = 0x60;
    const RESPONSE_DISTANCE_FIELD: u32 = 0x64;
    const RESPONSE_FLAGS: u32 = 0xD8;
    const RESPONSE_CONFIGURED: u32 = 0x0010_0000;
    const EVENT_COMPLETE: u32 = 0x3AE;

    unsafe {
        if response == 0 {
            return 0;
        }
        let response_state = lf_checker_rt::callee_thiscall!(4, u32, response);
        if response_state != EVENT_COMPLETE {
            return response;
        }

        write_u32(response, RESPONSE_DURATION_FIELD, duration.to_bits());
        write_u32(response, RESPONSE_DISTANCE_FIELD, distance.to_bits());
        write_u32(
            response,
            RESPONSE_FLAGS,
            read_u32(response, RESPONSE_FLAGS) | RESPONSE_CONFIGURED,
        );
        let response_subobject = response + RESPONSE_VTABLE_SUBOBJECT;
        let _ = lf_checker_rt::callee_thiscall!(
            5,
            u32,
            response_subobject,
            request,
            this + 0x30,
            0
        );
        let radius = read_f32(this, 0x48);
        let _ = lf_checker_rt::callee_thiscall!(
            6,
            u32,
            response_subobject,
            radius.to_bits()
        );
        response
    }
}

lf_checker_rt::export!(thiscall, rw_00bc3220(this: u32, request: u32) -> u32 {
    unsafe { handle_seek_task_event(this, request, false) }
});


