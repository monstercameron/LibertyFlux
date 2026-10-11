// original: 0x006692A0 rage::ptxEventEffect::vf14

/// Read the target pointer at offset `0x110` in the second stack argument.
/// If it is nonzero, invoke the intercepted parameterless `thiscall` helper
/// with that pointer in ECX and return its EAX. A zero field returns zero.
/// The receiver and the first and third stack words are not read.
const TARGET_POINTER_OFFSET: u32 = 0x110;
const TARGET_CALL_ID: u32 = 1;

lf_checker_rt::export!(thiscall, rw_006692a0(_receiver: u32, _arg0: u32, owner: u32, _arg2: u32) -> u32 {
    let target = unsafe {
        (owner.wrapping_add(TARGET_POINTER_OFFSET) as *const u32).read_unaligned()
    };
    if target == 0 {
        return 0;
    }
    unsafe { lf_checker_rt::callee_thiscall!(TARGET_CALL_ID, u32, target) }
});
