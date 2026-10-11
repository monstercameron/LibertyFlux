// original: 0x00D74560 FRONTEND_MENU_MONTAGE_NAVIGATE_MT
/// Dispatches a montage navigation selector, updates its associated overlay
/// state, and forwards service calls with the original register and stack
/// arguments. A reserved selector has no effect; out-of-range selectors
/// return the constrained entry result unchanged.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

const C_LOG: u32 = 1;
const C_MODE_A: u32 = 2;
const C_MODE_B: u32 = 3;
const C_GATE_A: u32 = 4;
const C_GATE_B: u32 = 5;
const C_ACTION: u32 = 6;
const C_APPLY: u32 = 7;
const C_TAIL: u32 = 8;

const G_MODE_A: u32 = 0x01176D2C;
const G_MODE_B: u32 = 0x01176D28;
const G_TOGGLE: u32 = 0x01176D38;
const G_TAIL_ECX: u32 = 0x011F6954;
const G_LOG_ECX: u32 = 0x01176888;

#[inline(always)]
unsafe fn read_u8(address: u32) -> u8 {
    unsafe { (address as *const u8).read_unaligned() }
}

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u8(address: u32, value: u8) {
    unsafe { (address as *mut u8).write_unaligned(value) }
}

#[inline(always)]
unsafe fn pointer_at(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn log_action(message: u32) -> u32 {
    unsafe {
        callee_thiscall!(
            C_LOG,
            u32,
            relocated(G_LOG_ECX),
            relocated(message)
        )
    }
}

#[inline(always)]
unsafe fn clamped_mode(base_va: u32, amount: u32) -> u32 {
    let sum = unsafe { global::<u32>(base_va).read() }.wrapping_add(amount);
    let signed = sum as i32;
    if signed < 0 {
        4
    } else if signed > 4 {
        0
    } else {
        sum
    }
}

unsafe fn navigate(this_ptr: u32, amount: u32) -> u32 {
    let item = unsafe { pointer_at(this_ptr.wrapping_add(4)) };
    let selector = unsafe { read_u32(this_ptr.wrapping_add(0x3C)) };
    let index = selector.wrapping_sub(1);
    if index > 9 {
        return index;
    }

    match index {
        0 => {
            let message = unsafe { log_action(0x00EEBBF0) };
            let flags = unsafe { read_u8(item.wrapping_add(0x1B)) };
            let result = !flags;
            if result & 1 != 0 {
                unsafe { write_u8(item.wrapping_add(0x1B), flags | 1) };
            }
            (message & 0xFFFF_FF00) | u32::from(result)
        }
        1 => {
            let message = unsafe { log_action(0x00EEBC14) };
            let current = unsafe { read_u8(this_ptr.wrapping_add(0xFC)) };
            let next = u8::from(current == 0);
            unsafe {
                write_u8(
                    this_ptr.wrapping_add(0xFC),
                    next,
                )
            };
            let returned = next;
            (message & 0xFFFF_FF00) | u32::from(returned)
        }
        2 => {
            unsafe { log_action(0x00EEBCEC) };
            let child = unsafe { pointer_at(item.wrapping_add(4)) };
            let value = u8::from(unsafe { read_u8(child.wrapping_add(0x3C)) } == 0);
            unsafe { write_u8(child.wrapping_add(0x3C), value) };
            (item & 0xFFFF_FF00) | u32::from(value)
        }
        3 => {
            let first = unsafe { callee_cdecl!(C_GATE_A, u32,) };
            if first & 0xFF != 0 {
                return first;
            }
            let second = unsafe { callee_cdecl!(C_GATE_B, u32,) };
            if second & 0xFF != 0 {
                return second;
            }
            unsafe { log_action(0x00EEBCA4) };
            unsafe { write_u8(item.wrapping_add(0x18), 1) };
            unsafe { callee_cdecl!(C_ACTION, u32, 0) }
        }
        4 => {
            unsafe { log_action(0x00EEBCC8) };
            unsafe { callee_thiscall!(C_APPLY, u32, item) }
        }
        5 => {
            unsafe { log_action(0x00EEBD10) };
            unsafe {
                write_u8(item.wrapping_add(0x18), 0);
                write_u8(item.wrapping_add(0x1A), 1);
            }
            let target_ecx = unsafe { global::<u32>(G_TAIL_ECX).read() };
            unsafe { callee_thiscall!(C_TAIL, u32, target_ecx, 0) }
        }
        6 => index,
        7 => {
            unsafe { log_action(0x00EEBC38) };
            let value = unsafe { clamped_mode(G_MODE_A, amount) };
            unsafe {
                callee_thiscall!(
                    C_MODE_A,
                    u32,
                    relocated(G_LOG_ECX),
                    value
                )
            };
            unsafe { write_u8(item.wrapping_add(0x18), 1) };
            item
        }
        8 => {
            unsafe { log_action(0x00EEBC5C) };
            let value = unsafe { clamped_mode(G_MODE_B, amount) };
            unsafe {
                callee_thiscall!(
                    C_MODE_B,
                    u32,
                    relocated(G_LOG_ECX),
                    value
                )
            };
            unsafe { write_u8(item.wrapping_add(0x18), 1) };
            item
        }
        9 => {
            unsafe { log_action(0x00EEBC80) };
            let current = unsafe { global::<u8>(G_TOGGLE).read() };
            unsafe { global::<u8>(G_TOGGLE).write(u8::from(current == 0)) };
            unsafe { write_u8(item.wrapping_add(0x18), 1) };
            item
        }
        _ => index,
    }
}

export!(thiscall, rw_d74560(this_ptr: u32, amount: u32) -> u32 {
    unsafe { navigate(this_ptr, amount) }
});

