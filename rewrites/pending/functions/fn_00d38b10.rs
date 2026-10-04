// original: 0x00d38b10 gate_owner_target
/// Gate the (owner, target) pair through a chain of checks: probe the inner
/// object, ask the mode gate, then walk the handle chain. Returns 1 when every
/// stage accepts, 0 at the first rejection. The original's stack-cookie check
/// is intentionally not reproduced: it is behaviorally transparent when the
/// stack is intact (the worker runs the real check on the original side), and
/// intercepting it would overwrite the return register with the stub answer.
export!(thiscall, rw_00d38b10(this: u32, target: u32) -> u32 {
    unsafe fn byte_at(addr: u32) -> u8 {
        *(addr as *const u8)
    }
    unsafe fn word_at(addr: u32) -> u32 {
        *(addr as *const u32)
    }
    let inner = unsafe { word_at(this + 0x18) };
    let probe = callee_thiscall!(1, u32, inner, target);
    let gate = callee_thiscall!(2, u32, this, target, probe);
    if gate & 0xff == 0 {
        return 0;
    }
    if unsafe { byte_at(target + 0x211) } != 0 {
        return 0;
    }
    if unsafe { byte_at(inner + 0x118) } & 1 != 0 {
        return 0;
    }
    let mode = unsafe { word_at(this + 0x20) };
    if mode == 0xffff_ffff {
        return 1;
    }
    let h1 = callee_thiscall!(4, u32, inner, mode);
    let h2 = callee_thiscall!(5, u32, h1, inner);
    let h3 = callee_thiscall!(6, u32, inner, h2);
    if unsafe { word_at(this + 0x14) } & 0x3500 != 0 {
        return 0;
    }
    if unsafe { byte_at(this + 0x30) } != 0 {
        return 0;
    }
    if h3 == 0 {
        return 0;
    }
    let fin = callee_thiscall!(7, u32, h3, inner);
    if fin & 0xff == 0 {
        return 0;
    }
    // The original passes a frame scratch word as the out-block; the contract
    // skips the address (frame layouts differ) and the answer drives the gate.
    let mut scratch = [0u32; 4];
    let v = callee_thiscall!(8, u32, scratch.as_mut_ptr() as u32, inner);
    if v & 0xff != 0 {
        return 0;
    }
    if unsafe { byte_at(target + 0x2a0) } & 1 != 0 {
        return 0;
    }
    1
});
