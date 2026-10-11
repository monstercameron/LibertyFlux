// original: 0x00B086E0 net_peer_link_update
//! Refresh the network peer and state objects, apply the current peer
//! parameters, compare their reported positions, and either schedule an
//! action or release temporary objects. The action path also checks a
//! returned vector and updates the selected peer's status words.
//!
//! This proof covers the non-null peer/state path with the early global
//! gate set. It does not reach fallback selection, the alternate gate
//! values, the action-vector threshold, or the action commit. The contract
//! also skips transient stack words passed to three helper calls.

#![allow(clippy::too_many_arguments)]

const ORIGINAL_GLOBAL: u32 = 0x0103_9298;
const GATE_GLOBAL: u32 = 0x0128_E3F9;
const FLAG_GLOBAL: u32 = 0x0104_0098;
const LIMIT_GLOBAL: u32 = 0x0104_009C;

const CALL_INITIALIZE: u32 = 1;
const CALL_GET_PEER: u32 = 2;
const CALL_GET_STATE: u32 = 3;
const CALL_GET_FALLBACK: u32 = 4;
const CALL_UPDATE_FLOATS: u32 = 5;
const CALL_UPDATE_PEER: u32 = 6;
const CALL_READ_RESULT: u32 = 7;
const CALL_APPLY_RESULT: u32 = 8;
const CALL_QUERY_STATE: u32 = 9;
const CALL_GET_ACTION: u32 = 10;
const CALL_COMMIT_ACTION: u32 = 11;
const CALL_RELEASE_PEER: u32 = 12;
const CALL_RELEASE_FALLBACK: u32 = 13;
const CALL_RELEASE_STATE: u32 = 14;
const CALL_GET_PEER_AGAIN: u32 = 15;
const CALL_RELEASE_ACTION: u32 = 16;
const SLOT_PEER: u32 = 17;
const SLOT_STATE: u32 = 18;
const SLOT_FALLBACK: u32 = 19;
const SLOT_ACTION_VECTOR: u32 = 20;

const OFF_SLOT: u32 = 0x0C;
const OFF_VECTOR_SLOT: u32 = 0xEC;
const OFF_STATE: u32 = 0x13C;
const OFF_ACTION_VECTOR: u32 = 0x20;
const OFF_ACTION_FLAGS: u32 = 0x28;
const OFF_ACTION_KIND: u32 = 0x1304;
const DISTANCE_LIMIT: f32 = f32::from_bits(0x41A0_0000);
const ACTION_LIMIT: f32 = f32::from_bits(0x4040_0000);
const DEFAULT_LIMIT: u32 = 0x0BB8;

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
    unsafe { f32::from_bits(read_u32(address)) }
}

#[inline(always)]
unsafe fn call_slot_zero(object: u32) -> u32 {
    unsafe {
        let table = read_u32(object);
        let target = read_u32(table.wrapping_add(OFF_SLOT));
        let function: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        function(object)
    }
}

#[inline(always)]
unsafe fn call_vector_slot(object: u32, out: u32) -> u32 {
    unsafe {
        let table = read_u32(object);
        let target = read_u32(table.wrapping_add(OFF_VECTOR_SLOT));
        let function: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        function(object, out)
    }
}

#[inline(always)]
unsafe fn point_from_answer(answer: u32) -> u32 {
    unsafe {
        let nested = read_u32(answer.wrapping_add(OFF_ACTION_VECTOR));
        if nested != 0 {
            nested.wrapping_add(0x30)
        } else {
            answer.wrapping_add(0x10)
        }
    }
}

#[inline(always)]
fn add_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

#[inline(always)]
fn sub_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) - core::hint::black_box(right)
}

#[inline(always)]
fn mul_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) * core::hint::black_box(right)
}

/// Match the original's vector distance: it subtracts each coordinate,
/// squares y before x before z, adds in that order, and then takes a
/// single-precision square root.
#[inline(always)]
unsafe fn distance_between(first: u32, second: u32) -> f32 {
    unsafe {
        let dx = sub_f32(read_f32(second), read_f32(first));
        let dy = sub_f32(read_f32(second.wrapping_add(4)), read_f32(first.wrapping_add(4)));
        let dz = sub_f32(read_f32(second.wrapping_add(8)), read_f32(first.wrapping_add(8)));
        let y_squared = mul_f32(dy, dy);
        let x_squared = mul_f32(dx, dx);
        let z_squared = mul_f32(dz, dz);
        add_f32(add_f32(y_squared, x_squared), z_squared).sqrt()
    }
}

/// Match the length calculation used by the later action-vector test.
#[inline(always)]
unsafe fn vector_length(vector: u32) -> f32 {
    unsafe {
        let x = read_f32(vector);
        let y = read_f32(vector.wrapping_add(4));
        let z = read_f32(vector.wrapping_add(8));
        let y_squared = mul_f32(y, y);
        let x_squared = mul_f32(x, x);
        let z_squared = mul_f32(z, z);
        add_f32(add_f32(y_squared, x_squared), z_squared).sqrt()
    }
}

unsafe fn run(this: u32, wrong_global_value: bool) -> u32 {
    unsafe {
        let original_this = this;
        let _initialized = lf_checker_rt::callee_thiscall!(CALL_INITIALIZE, u32, this);
        let mut eax = lf_checker_rt::callee_thiscall!(CALL_GET_PEER, u32, this, 2, 0);
        let mut ebx = eax;
        lf_checker_rt::global::<u32>(ORIGINAL_GLOBAL).write(if wrong_global_value {
            0
        } else {
            u32::MAX
        });
        let edi = lf_checker_rt::callee_thiscall!(CALL_GET_STATE, u32, this, 1, 0);
        let mut esi = 0;
        let mut fallback = 0;
        if ebx == 0 {
            eax = lf_checker_rt::callee_thiscall!(CALL_GET_FALLBACK, u32, original_this, 9, 0);
            ebx = eax;
            fallback = eax;
        } else {
            esi = ebx;
            eax = 0;
        }

        if edi != 0 {
            write_u32(edi.wrapping_add(0x2CC), 0x3E4C_CCCD);
            if esi != 0 {
                let first = read_u32(esi.wrapping_add(0x190));
                let second = read_u32(esi.wrapping_add(0x194));
                eax = lf_checker_rt::callee_thiscall!(CALL_UPDATE_FLOATS, u32, edi, first, second);
            } else if eax != 0 {
                let first = read_u32(eax.wrapping_add(0x218));
                let second = read_u32(eax.wrapping_add(0x21C));
                eax = lf_checker_rt::callee_thiscall!(CALL_UPDATE_FLOATS, u32, edi, first, second);
            }
            eax = lf_checker_rt::callee_thiscall!(CALL_UPDATE_PEER, u32, edi);
            if esi != 0 {
                // The original gives this virtual call a frame scratch
                // pointer. Its result is the value consumed by the next
                // direct helper; the checker contract records the call.
                let peer_result = call_slot_zero(edi);
                eax = lf_checker_rt::callee_cdecl!(CALL_READ_RESULT, u32, peer_result);
                if original_this != 0 {
                    eax = lf_checker_rt::callee_thiscall!(CALL_APPLY_RESULT, u32, edi, 0);
                }
                if read_u8(esi.wrapping_add(0x22C)) == 4 {
                    write_u8(
                        edi.wrapping_add(0x38C),
                        read_u8(edi.wrapping_add(0x38C)) | 1,
                    );
                } else {
                    let owner = read_u32(edi.wrapping_add(0x12C));
                    write_u32(owner.wrapping_add(0x1474), 0x4B);
                    write_u32(owner.wrapping_add(0x1478), 0);
                }
            }
        }

        if ebx != 0 {
            write_u8(ebx.wrapping_add(OFF_STATE), read_u8(ebx.wrapping_add(OFF_STATE)) & !8);
        }
        if edi != 0 {
            write_u8(edi.wrapping_add(OFF_STATE), read_u8(edi.wrapping_add(OFF_STATE)) | 0x0C);
        }

        let mut separation = 0.0_f32;
        if esi != 0 && edi != 0 {
            let peer_ready = call_slot_zero(esi);
            eax = peer_ready;
            if peer_ready != 0 {
                let state_ready = call_slot_zero(edi);
                eax = state_ready;
                if state_ready != 0 {
                    let first_answer = call_slot_zero(edi);
                    eax = first_answer;
                    let first_point = point_from_answer(first_answer);
                    let second_answer = call_slot_zero(esi);
                    eax = second_answer;
                    let second_point = point_from_answer(second_answer);
                    separation = distance_between(first_point, second_point);
                }
            }
        } else if fallback != 0 && edi != 0 {
            let fallback_ready = call_slot_zero(fallback);
            eax = fallback_ready;
            if fallback_ready != 0 {
                let state_ready = call_slot_zero(edi);
                eax = state_ready;
                if state_ready != 0 {
                    let first_answer = call_slot_zero(edi);
                    eax = first_answer;
                    let first_point = point_from_answer(first_answer);
                    let second_answer = call_slot_zero(fallback);
                    eax = second_answer;
                    let second_point = point_from_answer(second_answer);
                    separation = distance_between(first_point, second_point);
                }
            }
        }

        if read_u8(lf_checker_rt::global::<u8>(GATE_GLOBAL) as u32) != 0
            || separation > DISTANCE_LIMIT
        {
            return cleanup(eax, fallback, esi, original_this);
        }

        if esi != 0 {
            let has_peer_data = call_slot_zero(esi);
            eax = has_peer_data;
            if has_peer_data != 0 {
                let has_peer_flags = call_slot_zero(esi);
                eax = has_peer_flags;
                if (read_u32(has_peer_flags.wrapping_add(OFF_ACTION_FLAGS)) & 0x3C0) == 0x80 {
                    let peer_kind = call_slot_zero(esi);
                    eax = peer_kind;
                    if read_u32(peer_kind.wrapping_add(OFF_ACTION_KIND)) == 3 {
                        return release_peer(ebx);
                    }
                }
            }
        }

        let mut action_limit = read_u32(lf_checker_rt::global::<u32>(FLAG_GLOBAL) as u32);
        if action_limit == 0 {
            return release_peer(ebx);
        }
        if ebx == 0 || edi == 0 {
            return cleanup(eax, fallback, esi, original_this);
        }

        let action = lf_checker_rt::callee_thiscall!(CALL_GET_ACTION, u32, original_this, 3, 0);
        fallback = action;
        eax = action;
        if action == 0 {
            return cleanup(eax, fallback, esi, original_this);
        }
        action_limit = read_u32(lf_checker_rt::global::<u32>(LIMIT_GLOBAL) as u32);

        if esi != 0 {
            let action_ready = call_slot_zero(esi);
            eax = action_ready;
            if action_ready != 0 {
                let vector_owner = call_slot_zero(esi);
                eax = vector_owner;
                if vector_owner != 0 {
                    let mut scratch = [0u32; 3];
                    let vector = call_vector_slot(vector_owner, scratch.as_mut_ptr() as u32);
                    eax = vector;
                    let length = vector_length(vector);
                    if length > ACTION_LIMIT {
                        let kind_owner = call_slot_zero(esi);
                        eax = kind_owner;
                        if read_u32(kind_owner.wrapping_add(OFF_ACTION_KIND)) != 1 {
                            action_limit = DEFAULT_LIMIT;
                        }
                    }
                }
            }
        }

        eax = lf_checker_rt::callee_thiscall!(
            CALL_COMMIT_ACTION,
            u32,
            ebx,
            ebx,
            edi,
            action_limit,
            0,
            1
        );
        write_u8(ebx.wrapping_add(OFF_STATE), read_u8(ebx.wrapping_add(OFF_STATE)) | 0x0C);
        write_u32(ebx.wrapping_add(0x164), 1);
        write_u32(ebx.wrapping_add(0x16C), 1);
        eax
    }
}

unsafe fn cleanup(mut eax: u32, fallback: u32, state: u32, this: u32) -> u32 {
    unsafe {
        if fallback != 0 {
            eax = lf_checker_rt::callee_thiscall!(CALL_RELEASE_FALLBACK, u32, fallback, 0);
        }
        if state != 0 {
            eax = lf_checker_rt::callee_thiscall!(CALL_RELEASE_STATE, u32, state, 0);
        }
        eax = lf_checker_rt::callee_thiscall!(CALL_GET_PEER_AGAIN, u32, this, 2, 0);
        if eax != 0 {
            eax = lf_checker_rt::callee_thiscall!(CALL_RELEASE_ACTION, u32, eax, 0);
        }
        eax
    }
}

unsafe fn release_peer(peer: u32) -> u32 {
    unsafe {
        if peer != 0 {
            lf_checker_rt::callee_thiscall!(CALL_RELEASE_PEER, u32, peer, 0)
        } else {
            0
        }
    }
}

lf_checker_rt::export!(thiscall, rw_00B086E0(this: u32) -> u32 {
    unsafe { run(this, false) }
});

lf_checker_rt::export!(thiscall, rw_00B086E0_mut(this: u32) -> u32 {
    unsafe { run(this, true) }
});
