// original: 0x00CD78F0 CTaskComplexFollowLeaderInFormation::vf1

/// Resolve the ped from the shared intelligence context and, when present,
/// request a formation-follow task using the object's two handles and the
/// stored blend factor. The helper's return value is returned unchanged; a
/// missing ped yields zero. The blend factor is copied as its original f32
/// bit pattern without arithmetic.
///
/// Calling convention: thiscall with no incoming stack arguments. The
/// delegated helper receives the first handle, second handle, then the blend
/// factor as three 32-bit stack words.
lf_checker_rt::export!(thiscall, rw_00cd78f0(this: u32) -> u32 {
    const INTELLIGENCE_CONTEXT: u32 = 0x0167_E2A0;
    const FIRST_HANDLE: u32 = 0x14;
    const SECOND_HANDLE: u32 = 0x18;
    const BLEND_FACTOR: u32 = 0x1C;
    const FIND_PED: u32 = 1;
    const CREATE_TASK: u32 = 2;

    unsafe {
        let context = lf_checker_rt::global::<u32>(INTELLIGENCE_CONTEXT).read();
        let ped = lf_checker_rt::callee_thiscall!(FIND_PED, u32, context);
        if ped == 0 {
            0
        } else {
            let first = ((this + FIRST_HANDLE) as *const u32).read_unaligned();
            let second = ((this + SECOND_HANDLE) as *const u32).read_unaligned();
            let blend_bits = ((this + BLEND_FACTOR) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(CREATE_TASK, u32, ped, first, second, blend_bits)
        }
    }
});
