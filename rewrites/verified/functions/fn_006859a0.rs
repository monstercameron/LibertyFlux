// original: 0x006859A0 forward_to_00686f30

/// Forwards the value and owner pointer through helper 1 and returns the
/// helper answer. ECX points to the second word of a two-word local context.
/// The contract omits that frame address with an empty call-register set and
/// snapshots one 32-bit pointee word at offset 0. The stack arguments remain
/// compared in order: the input value, then the owner pointer.
///
/// Scope: one straight-line call; helper 1 is an edges stub, so its native
/// implementation is outside this proof. The strict stock-v8 checks include
/// calls, return, stack, heap, globals, termination and undeclared calls.
lf_checker_rt::export!(thiscall, rw_006859a0(this: u32, value: u32) -> u32 {
    let mut context = [this, this];
    let context_ptr = (&mut context[1] as *mut u32) as u32;
    lf_checker_rt::callee_thiscall!(1, u32, context_ptr, value, this)
});
