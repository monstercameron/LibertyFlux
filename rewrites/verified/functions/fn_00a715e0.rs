// original: 0x00A715E0 CTaskComplexPlayerPlaceCarBomb::vf18
/// Dispatches a player place-car-bomb task event. It gates completion while an
/// event is active; otherwise it handles the exact event type by constructing
/// and attaching the appropriate task, updating the task state, and running
/// cleanup when the resulting active/flag state requires it. Distance checks
/// use single-precision arithmetic in source order and preserve the original
/// signed count comparison and event-specific call arguments.
// Primitive memory accessors used by the event rewrite.
#[inline(always)]
unsafe fn read_u8(address: u32) -> u8 { unsafe { (address as *const u8).read() } }
#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
#[inline(always)]
unsafe fn write_u8(address: u32, value: u8) { unsafe { (address as *mut u8).write(value) } }
#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }
#[inline(always)]
unsafe fn read_f32(address: u32) -> f32 { f32::from_bits(unsafe { read_u32(address) }) }

const BOMB_ACTIVE: u32 = 0x19;
const BOMB_INTERFACE: u32 = 0x08;
const BOMB_PLAYER: u32 = 0x14;
const BOMB_FLAGS: u32 = 0x26;
const BOMB_POSITION: u32 = 0x30;
const BOMB_COUNT: u32 = 0x40;
const BOMB_CONTEXT_ENTITY: u32 = 0x20;
const BOMB_ENTITY_POSITION: u32 = 0x30;
const BOMB_PI: u32 = 0x40490fdb;
const BOMB_DISTANCE_LIMIT: u32 = 0x00fe87e8;
const BOMB_SCALE_A: u32 = 0x00ed7e68;
const BOMB_SCALE_B: u32 = 0x00fe8a24;
const BOMB_POOL: u32 = 0x0167e2a0;

const P_ALLOCATE_FIRST: u32 = 1;
const P_ALLOCATE_SECOND: u32 = 2;
const P_RANGE_VALUE: u32 = 3;
const P_BUILD_RANGE: u32 = 4;
const P_ATTACH_TASK: u32 = 5;
const P_ALLOCATE_FAR: u32 = 6;
const P_ALLOCATE_CHILD: u32 = 7;
const P_CHILD_SCALE: u32 = 8;
const P_BUILD_CHILD: u32 = 9;
const P_ATTACH_CHILD: u32 = 10;
const P_DIRECTION: u32 = 11;
const P_ANGLE: u32 = 12;
const P_ALLOCATE_NEAR: u32 = 13;
const P_BUILD_NEAR: u32 = 14;
const P_ALLOCATE_SPECIAL: u32 = 15;
const P_BUILD_SPECIAL: u32 = 16;
const P_ALLOCATE_MEDIUM: u32 = 17;
const P_BUILD_MEDIUM: u32 = 18;
const P_ALLOCATE_TIMER: u32 = 19;
const P_BUILD_TIMER: u32 = 20;
const P_CLEANUP: u32 = 21;
const P_EVENT_TYPE: u32 = 22;
const P_PHASE_COMPLETE: u32 = 23;

#[inline(always)]
fn add_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(core::hint::black_box(left) + core::hint::black_box(right))
}

#[inline(always)]
fn sub_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(core::hint::black_box(left) - core::hint::black_box(right))
}

#[inline(always)]
fn mul_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(core::hint::black_box(left) * core::hint::black_box(right))
}

#[target_feature(enable = "sse")]
unsafe fn sqrt_f32(value: f32) -> f32 {
    unsafe {
        let input = core::arch::x86::_mm_set_ss(value);
        core::arch::x86::_mm_cvtss_f32(core::arch::x86::_mm_sqrt_ss(input))
    }
}

/// Handle the player place-car-bomb task's event dispatch. Event 0x190 marks
/// the task active; 0x13f constructs a two-part task; 0x11d chooses a nearby
/// or distant entity task using a single-precision distance and a signed
/// count limit; 0x3a6 creates a direction task; and 0xca, 0x398 and 0x3c1
/// select either a special task or a one-second timer. The top-level event
/// gate is signed, the stored count comparison is signed, and all event
/// selections after the gate are exact equality tests. Distance is evaluated
/// as the y and x squares followed by the z square; an unordered comparison
/// result follows the near path. This is a thiscall with one context pointer
/// argument. Calls returning float use the x87-compatible cdecl ABI.
unsafe fn place_car_bomb(this: u32, event_record: u32) -> u32 {
    unsafe {
        let mut selected = 0u32;
        let phase_active = read_u8(this + BOMB_ACTIVE) != 0;
        let interface = read_u32(this + BOMB_INTERFACE);

        if phase_active {
            let _ = lf_checker_rt::callee_thiscall!(P_EVENT_TYPE, u32, interface);
            let task_flags = read_u32(interface + 0x0c);
            if task_flags & 1 == 0 {
                let completed = lf_checker_rt::callee_thiscall!(
                    P_PHASE_COMPLETE,
                    u8,
                    interface,
                    event_record,
                    1,
                    0
                );
                if completed != 0 {
                    write_u32(interface + 0x0c, task_flags | 2);
                }
            }
        } else {
            let event_type = lf_checker_rt::callee_thiscall!(P_EVENT_TYPE, u32, interface);
            if event_type == 0x190 {
                write_u8(this + BOMB_ACTIVE, 1);
            } else if event_type == 0x13f {
                let parent = lf_checker_rt::callee_thiscall!(P_ALLOCATE_FIRST, u32, pool_pointer());
                if parent != 0 {
                    let child = lf_checker_rt::callee_thiscall!(P_ALLOCATE_SECOND, u32, pool_pointer());
                    let child_task = if child != 0 {
                        let scale = lf_checker_rt::callee_cdecl!(
                            P_RANGE_VALUE,
                            f32,
                            2u32,
                            this + BOMB_POSITION,
                            0.2f32.to_bits(),
                            1.0f32.to_bits(),
                            u32::MAX,
                            1u32,
                            0u32,
                            0u32,
                            0u32,
                            1u32
                        );
                        lf_checker_rt::callee_thiscall!(
                            P_BUILD_RANGE,
                            u32,
                            child,
                            scale.to_bits(),
                            this + BOMB_POSITION,
                            0.2f32.to_bits(),
                            1.0f32.to_bits(),
                            u32::MAX,
                            1u32,
                            0u32,
                            0u32,
                            0u32,
                            1u32
                        )
                    } else {
                        0
                    };
                    selected = lf_checker_rt::callee_thiscall!(
                        P_ATTACH_TASK,
                        u32,
                        parent,
                        child_task,
                        0,
                        0,
                        0
                    );
                }
            } else if event_type == 0x11d {
                let entity = read_u32(event_record + BOMB_CONTEXT_ENTITY);
                let dx = sub_f32(read_f32(entity + BOMB_ENTITY_POSITION), read_f32(this + BOMB_POSITION));
                let dy = sub_f32(read_f32(entity + BOMB_ENTITY_POSITION + 4), read_f32(this + BOMB_POSITION + 4));
                let dz = sub_f32(read_f32(entity + BOMB_ENTITY_POSITION + 8), read_f32(this + BOMB_POSITION + 8));
                let dy_squared = mul_f32(dy, dy);
                let dx_squared = mul_f32(dx, dx);
                let dz_squared = mul_f32(dz, dz);
                let planar_squared = add_f32(dy_squared, dx_squared);
                let distance_squared = add_f32(planar_squared, dz_squared);
                let distance = unsafe { sqrt_f32(distance_squared) };
                let limit = read_f32(lf_checker_rt::relocated(BOMB_DISTANCE_LIMIT));
                let is_near = distance <= limit || distance.is_nan();
                let count = read_u32(this + BOMB_COUNT);
                if !is_near && (count as i32) < 4 {
                    let parent = lf_checker_rt::callee_thiscall!(P_ALLOCATE_FAR, u32, pool_pointer());
                    if parent == 0 {
                        selected = 0;
                    } else {
                        let child = lf_checker_rt::callee_thiscall!(P_ALLOCATE_CHILD, u32, pool_pointer());
                        let child_task = if child != 0 {
                            let scale = lf_checker_rt::callee_cdecl!(
                                P_CHILD_SCALE,
                                f32,
                                2u32,
                                this + BOMB_POSITION,
                                0.2f32.to_bits(),
                                1u32,
                                0u32
                            );
                            lf_checker_rt::callee_thiscall!(
                                P_BUILD_CHILD,
                                u32,
                                child,
                                scale.to_bits(),
                                this + BOMB_POSITION,
                                0.2f32.to_bits(),
                                1u32,
                                0u32
                            )
                        } else {
                            0
                        };
                        selected = lf_checker_rt::callee_thiscall!(
                            P_ATTACH_CHILD,
                            u32,
                            parent,
                            child_task,
                            0,
                            0,
                            0
                        );
                    }
                    let increment = if mutant { 2 } else { 1 };
                    write_u32(this + BOMB_COUNT, count.wrapping_add(increment));
                } else {
                    let player = read_u32(this + BOMB_PLAYER);
                    let target = read_u32(player + 0x20);
                    let direction = lf_checker_rt::callee_cdecl!(
                        P_DIRECTION,
                        f32,
                        read_f32(target + BOMB_ENTITY_POSITION).to_bits(),
                        read_f32(target + BOMB_ENTITY_POSITION + 4).to_bits(),
                        read_f32(entity + BOMB_ENTITY_POSITION).to_bits(),
                        read_f32(entity + BOMB_ENTITY_POSITION + 4).to_bits()
                    );
                    let angle = lf_checker_rt::callee_cdecl!(P_ANGLE, f32, direction.to_bits());
                    let task = lf_checker_rt::callee_thiscall!(P_ALLOCATE_NEAR, u32, pool_pointer());
                    selected = lf_checker_rt::callee_thiscall!(
                        P_BUILD_NEAR,
                        u32,
                        task,
                        this + BOMB_POSITION,
                        angle.to_bits(),
                        8.0f32.to_bits()
                    );
                    write_u32(selected + 0xdc, BOMB_PI);
                }
            } else if event_type == 0x398 || event_type == 0x3c1 || event_type == 0xca {
                if read_u8(this + BOMB_FLAGS) & 1 != 0 {
                    let task = lf_checker_rt::callee_thiscall!(P_ALLOCATE_SPECIAL, u32, pool_pointer());
                    if task != 0 {
                        selected = lf_checker_rt::callee_thiscall!(
                            P_BUILD_SPECIAL,
                            u32,
                            task,
                            0x18u32,
                            0x12cu32,
                            1000.0f32.to_bits(),
                            0u32,
                            1.0f32.to_bits(),
                            0u32
                        );
                    }
                } else {
                    let task = lf_checker_rt::callee_thiscall!(P_ALLOCATE_TIMER, u32, pool_pointer());
                    if task != 0 {
                        selected = lf_checker_rt::callee_thiscall!(P_BUILD_TIMER, u32, task, 0x3e8);
                    }
                }
            } else if event_type == 0x3a6 {
                let task = lf_checker_rt::callee_thiscall!(P_ALLOCATE_MEDIUM, u32, pool_pointer());
                if task != 0 {
                    let player = read_u32(this + BOMB_PLAYER);
                    let target = read_u32(player + 0x20);
                    let factor = mul_f32(read_f32(lf_checker_rt::relocated(BOMB_SCALE_A)), read_f32(lf_checker_rt::relocated(BOMB_SCALE_B)));
                    selected = lf_checker_rt::callee_thiscall!(
                        P_BUILD_MEDIUM,
                        u32,
                        task,
                        target + BOMB_ENTITY_POSITION,
                        factor.to_bits(),
                        0.2f32.to_bits()
                    );
                }
            }
        }

        if read_u8(this + BOMB_ACTIVE) != 0 && read_u8(this + BOMB_FLAGS) & 1 != 0 {
            lf_checker_rt::callee_thiscall!(P_CLEANUP, u32, this + 0x1c, u32::MAX);
        }
        selected
    }
}

#[inline(always)]
fn pool_pointer() -> u32 {
    unsafe { lf_checker_rt::global::<u32>(BOMB_POOL).read_unaligned() }
}

#[inline(always)]
unsafe fn read_f32(address: u32) -> f32 {
    f32::from_bits(unsafe { read_u32(address) })
}

lf_checker_rt::export!(thiscall, rw_00a715e0(this: u32, event_record: u32) -> u32 {
    unsafe { place_car_bomb(this, event_record) }
});

