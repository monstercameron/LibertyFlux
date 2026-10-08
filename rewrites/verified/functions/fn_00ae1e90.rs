//! Proof scope: complete body under non-null allocation and valid pointer fixtures.
//! Allocator and dispatch effects are scripted; null allocation and other branch shapes are excluded.
//! Full-data checking is disabled; no stack writes were observed. Broader engine states are unproven.
//! Corrected cookie behavior passed all configured comparisons; the same-contract counter mutant fails.
//! The positive export always enables the counter update; its tested helper remains unchanged.
//! Only the mutant export was removed; this projection has not been separately built.
//! Earlier failed proofs and setup errors remain preserved; copied process timestamps are not fresh queries.
#![allow(non_snake_case)]

use core::ptr;

const COUNTER_VA: u32 = 0x0103_27A0;
const INITIAL_VTABLE_VA: u32 = 0x00E7_E048;
const DISPATCH_VTABLE_VA: u32 = 0x00EA_772C;
const OBJECT_BYTES: u32 = 0x1C;

#[inline(always)]
fn sign_extend_low_nibble(x: u32) -> u32 {
    let mut v = x & 0x8000_000F;
    if (v as i32) < 0 {
        v = v.wrapping_sub(1);
        v |= 0xFFFF_FFF0;
        v = v.wrapping_add(1);
    }
    v
}

unsafe fn model(
    scalar: u32,
    p1: u32,
    p2: u32,
    p3: u32,
    p4: u32,
    increment_counter: bool,
) -> u32 {
    unsafe {
        // The contract's allocator script always returns heap segment 0.
        // The original null branch faults at address zero, so null allocation
        // is deliberately outside this contract's input domain.
        let object = lf_checker_rt::callee_cdecl!(0, u32, OBJECT_BYTES, 0u32);
        let obj = object as *mut u32;

        // Match the original field setup and its fixed image vtable writes.
        ptr::write_volatile(obj, lf_checker_rt::relocated(INITIAL_VTABLE_VA));
        let old_cookie = ptr::read_volatile(obj.add(1));
        let counter = lf_checker_rt::global::<u32>(COUNTER_VA);
        let counter_value = ptr::read_volatile(counter);
        ptr::write_volatile(obj.add(1), old_cookie ^ ((old_cookie ^ counter_value) & 0x3FFF));
        if increment_counter {
            ptr::write_volatile(counter, counter_value.wrapping_add(1));
        }
        ptr::write_volatile(obj.add(2), scalar);
        ptr::write_volatile(obj, lf_checker_rt::relocated(DISPATCH_VTABLE_VA));
        ptr::write_volatile(obj.add(3), ptr::read_volatile(p1 as *const u32));
        ptr::write_volatile(obj.add(4), ptr::read_volatile(p2 as *const u32));
        ptr::write_volatile(obj.add(5), ptr::read_volatile(p3 as *const u32));
        ptr::write_volatile(obj.add(6), ptr::read_volatile(p4 as *const u32));

        // Keep both calls as genuine indirect [vtable+8] dispatches. v8
        // plants recorder stub 1 in the fixed .rdata slot for both sides.
        let table1 = ptr::read_volatile(obj);
        let slot1 = ptr::read_volatile(table1.wrapping_add(8) as *const u32);
        let call1: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot1 as usize);
        let first_raw = call1(object);

        let first = sign_extend_low_nibble(first_raw);
        let second_arg = sign_extend_low_nibble(0x10u32.wrapping_sub(first));

        let table2 = ptr::read_volatile(obj);
        let slot2 = ptr::read_volatile(table2.wrapping_add(8) as *const u32);
        let call2: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot2 as usize);
        let second_raw = call2(object);

        let sum = second_raw.wrapping_add(second_arg);
        let sign_fill = if (sum as i32) < 0 { u32::MAX } else { 0 };
        let adjusted = sum.wrapping_add(sign_fill & 0xF);
        let shifted = ((adjusted as i32) >> 4) as u32;
        let scaled = shifted.wrapping_shl(14);
        let before = ptr::read_volatile(obj.add(1));
        let result = (scaled ^ before) & 0x01FF_C000;
        ptr::write_volatile(obj.add(1), before ^ result);
        result
    }
}

lf_checker_rt::export!(cdecl, fn_00ae1e90(
    scalar: u32, p1: u32, p2: u32, p3: u32, p4: u32
) -> u32 {
    unsafe { model(scalar, p1, p2, p3, p4, true) }
});

