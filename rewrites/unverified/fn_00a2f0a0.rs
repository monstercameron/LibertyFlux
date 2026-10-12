// original: 0x00A2F0A0 rage::sysEmergencyAllocator::vf0

/// Runs the allocator's base cleanup with the object in ECX.
///
/// The thiscall entry takes a pointer-like receiver in ECX and one 32-bit
/// flags word on the stack. It always invokes the no-argument base cleanup,
/// invokes the optional release helper with the receiver when bit zero of
/// flags is set, returns the receiver unchanged in EAX, and pops the one
/// stack argument. The helper calls are intercepted by the proof contract.
lf_checker_rt::export!(thiscall, rw_00a2f0a0(receiver: u32, flags: u32) -> u32 {
    const BASE_CLEANUP: u32 = 1;
    const OPTIONAL_RELEASE: u32 = 2;

    let _base_result = lf_checker_rt::callee_thiscall!(BASE_CLEANUP, u32, receiver);
    if flags & 1 != 0 {
        let _release_result = lf_checker_rt::callee_cdecl!(OPTIONAL_RELEASE, u32, receiver);
    }
    receiver
});
