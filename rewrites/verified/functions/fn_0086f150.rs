//! Proof scope: complete straight-line wrapper under valid frame, text and integer fixtures.
//! The callback is a scripted recorder; native formatting, output and collaborator effects are untested.
//! Both forwarded words, text snapshots and callback EAX are compared; semantic text/integer labels are inferred.
//! Malformed pointers and broader engine states are excluded; one call shape is not branch instrumentation.
//! Current immutable run receipts override stale preparation metadata; the positive-only projection removes the mutant export.
#![allow(unsafe_code)]

// High-level wrapper: read the native argument pair from the handler frame,
// forward its text and integer through the relocated debug-print table slot,
// and return the callback's EAX value.
use core::{
    ffi::{c_char, c_void},
    mem,
    ptr,
};

const DEBUG_PRINT_TABLE_FILE_VA: u32 = 0x0110_B718;

#[repr(C)]
struct HandlerFrame {
    _reserved: [u32; 2],
    values: *const PrintInt2Values,
}

#[repr(C)]
struct PrintInt2Values {
    text: *const c_char,
    value: i32,
}

type DebugPrint = unsafe extern "C" fn(*const c_char, i32) -> u32;

unsafe fn invoke_debug_print(frame: *const HandlerFrame, increment: i32) -> u32 {
    let values_pointer = unsafe { ptr::read_unaligned(ptr::addr_of!((*frame).values)) };
    let values = unsafe { ptr::read_unaligned(values_pointer) };
    let slot = lf_checker_rt::global::<*const c_void>(DEBUG_PRINT_TABLE_FILE_VA);
    let callback_pointer = unsafe { ptr::read_unaligned(slot) };
    let callback: DebugPrint = unsafe { mem::transmute(callback_pointer) };
    unsafe { callback(values.text, values.value.wrapping_add(increment)) }
}

lf_checker_rt::export!(cdecl, rw_fn_0086f150(frame: *const HandlerFrame) -> u32 {
    unsafe { invoke_debug_print(frame, 0) }
});

