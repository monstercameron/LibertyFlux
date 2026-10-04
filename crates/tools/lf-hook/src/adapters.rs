//! `thiscall` support on stable Rust: tiny generated stubs.
//!
//! Game classes use `thiscall` (`this` in ECX, args on the stack, callee
//! pops). Stable Rust has no `thiscall` ABI, so each thiscall hook gets two
//! generated 32-bit stubs in executable memory:
//!
//! * detour stub (the hook target): rebuilds a cdecl `(this, args...)`
//!   frame for a plain `extern "C" fn(this: *mut u8, args...) -> Ret`,
//!   stashing the return address in EBX (callee-saved, so the Rust
//!   function preserves it) and unwinding everything afterwards.
//! * caller stub (to invoke the trampoline as thiscall): entered with
//!   `(this, args...)` on the stack, moves `this` into ECX, drops it and
//!   jumps to the trampoline. The stub is callee-pops (like the thiscall
//!   target itself), so callers must use an `extern "system"` prototype,
//!   never `extern "C"`.
//!
//! Both preserve every register except the return registers; ECX is
//! caller-saved under thiscall so clobbering it is legal. All stub state
//! lives on the stack, so stubs are reentrant and thread-safe.

// The two stub builders write machine code into fresh executable
// allocations of matching size; the `unsafe` blocks are those writes.
#![allow(unsafe_code)]

use crate::detour::HookError;
use crate::mem;

/// Build a detour stub for a thiscall function with `nargs` stack arguments.
/// Returns the stub address to use as the hook's detour.
///
/// Entered as thiscall (`[ret][a1..aN]`, `this` in ECX). Builds a cdecl
/// frame `[ret2][this][a1..aN]` for `rust_fn`, then unwinds to the
/// original frame and `ret N*4`, exactly like the hooked function would.
pub fn make_thiscall_detour_stub(rust_fn: usize, nargs: u32) -> Result<*mut u8, HookError> {
    assert!(nargs <= 30, "thiscall stub supports at most 30 stack args");
    // 53                  push ebx
    // 8B 5C 24 04         mov ebx,[esp+4]      ; stash return address
    // FF 74 24 DD (x N)   push dword [esp+DD]  ; aN..a1, DD = 4+N*4
    // 51                  push ecx             ; this
    // E8 rel32            call rust_fn
    // 83 C4 (4+N*4)       add esp,4+N*4
    // 5B                  pop ebx
    // C2 N*4              ret N*4
    let n = nargs as usize;
    let dd = (4 + n * 4) as u8;
    let frame = (4 + n * 4) as u8;
    // 5 (push/mov) + 4 per arg + 1 (push ecx) + 5 (call) + 3 (add esp)
    // + 1 (pop ebx) + 3 (ret imm16).
    let len = 5 + 4 * n + 1 + 5 + 3 + 1 + 3;
    let stub = mem::alloc_exec(len).map_err(HookError::AllocFailed)?;
    let mut b = vec![0u8; len];
    b[0] = 0x53;
    b[1] = 0x8B;
    b[2] = 0x5C;
    b[3] = 0x24;
    b[4] = 0x04;
    for i in 0..n {
        let o = 5 + i * 4;
        b[o] = 0xFF;
        b[o + 1] = 0x74;
        b[o + 2] = 0x24;
        b[o + 3] = dd;
    }
    let mut o = 5 + 4 * n;
    b[o] = 0x51; // push ecx
    o += 1;
    b[o] = 0xE8; // call rust_fn
    let rel = (rust_fn as i64).wrapping_sub(stub as usize as i64 + o as i64 + 5) as i32;
    b[o + 1..o + 5].copy_from_slice(&rel.to_le_bytes());
    o += 5;
    b[o] = 0x83; // add esp,frame
    b[o + 1] = 0xC4;
    b[o + 2] = frame;
    o += 3;
    b[o] = 0x5B; // pop ebx
    o += 1;
    b[o] = 0xC2; // ret N*4
    b[o + 1..o + 3].copy_from_slice(&((nargs * 4) as u16).to_le_bytes());
    debug_assert_eq!(o + 3, len);
    // SAFETY: `stub` is a fresh `len`-byte allocation and `b` is `len` bytes.
    unsafe { mem::write_exec(stub, &b) };
    Ok(stub)
}

/// Build a caller stub that invokes `trampoline` as thiscall.
///
/// The stub takes `(this, args...)` on the stack: it pops the return
/// address, swaps `this` out of the stack, moves it to ECX and jumps on.
/// Callee-pops — call it through an `extern "system"` function pointer.
pub fn make_thiscall_caller_stub(trampoline: usize) -> Result<*mut u8, HookError> {
    // 58               pop eax            ; return address
    // 87 04 24        xchg eax,[esp]      ; eax=this, [esp]=ret
    // 8B C8           mov ecx,eax
    // 68 imm32        push trampoline
    // C3              ret
    let stub = mem::alloc_exec(12).map_err(HookError::AllocFailed)?;
    let mut b = [0u8; 12];
    b[0] = 0x58;
    b[1] = 0x87;
    b[2] = 0x04;
    b[3] = 0x24;
    b[4] = 0x8B;
    b[5] = 0xC8;
    b[6] = 0x68;
    b[7..11].copy_from_slice(&(trampoline as u32).to_le_bytes());
    b[11] = 0xC3;
    // SAFETY: `stub` is a fresh 12-byte allocation and `b` is 12 bytes.
    unsafe { mem::write_exec(stub, &b) };
    Ok(stub)
}

/// Free a stub built by `make_thiscall_detour_stub`/`make_caller_stub`.
pub fn free_stub(stub: *mut u8) {
    mem::free_exec(stub);
}
