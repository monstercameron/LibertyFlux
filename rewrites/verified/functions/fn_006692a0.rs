// original: 0x006692A0 rage::ptxEventEffect::vf14

/// Read the target pointer at offset `0x110` in the second stack argument.
/// If it is nonzero, invoke the intercepted parameterless `thiscall` helper
/// with that pointer in ECX and return its EAX. A zero field returns the
/// owner pointer already loaded into EAX.
/// The receiver and the first and third stack words are not read.
const VF14_TARGET_POINTER_OFFSET: u32 = 0x110;
const VF14_TARGET_CALL_ID: u32 = 1;

lf_checker_rt::export!(thiscall, rw_006692a0(_receiver: u32, _arg0: u32, owner: u32, _arg2: u32) -> u32 {
    let target = unsafe {
        (owner.wrapping_add(VF14_TARGET_POINTER_OFFSET) as *const u32).read_unaligned()
    };
    if target == 0 {
        return owner;
    }
    unsafe { lf_checker_rt::callee_thiscall!(VF14_TARGET_CALL_ID, u32, target) }
});
