// original: 0x00CD7860 CTaskComplexSeekEntityAnyMeans<CEntitySeekPosCalculatorXYOffset>::vf1

/// Use the shared intelligence context to find the ped, then ask the helper
/// for a destination position using the task's handle, a global distance word,
/// and the fixed scale value. Copy the task's four position words at offsets
/// `0x40` through `0x4C` to the returned destination and return that pointer.
/// Float fields are copied as raw bits. If either helper produces no
/// destination, the original's stores through the null result fault.
///
/// Calling convention: thiscall with no incoming stack arguments. Both
/// outgoing helpers are thiscall; the position helper takes three stack
/// words and returns the writable result pointer.
lf_checker_rt::export!(thiscall, rw_00cd7860(this: u32) -> u32 {
    const INTELLIGENCE_CONTEXT: u32 = 0x0167_E2A0;
    const DISTANCE_WORD: u32 = 0x0105_1AE8;
    const OBJECT_HANDLE: u32 = 0x14;
    const POSITION_X: u32 = 0x40;
    const POSITION_Y: u32 = 0x44;
    const POSITION_Z: u32 = 0x48;
    const POSITION_HEADING: u32 = 0x4C;
    const FIXED_SCALE_BITS: u32 = 0x4100_0000;
    const FIND_PED: u32 = 1;
    const GET_POSITION: u32 = 2;

    unsafe {
        let context = lf_checker_rt::global::<u32>(INTELLIGENCE_CONTEXT).read();
        let ped = lf_checker_rt::callee_thiscall!(FIND_PED, u32, context);
        let destination = if ped == 0 {
            0
        } else {
            let handle = ((this + OBJECT_HANDLE) as *const u32).read_unaligned();
            let distance = lf_checker_rt::global::<u32>(DISTANCE_WORD).read();
            lf_checker_rt::callee_thiscall!(GET_POSITION, u32, ped, handle, distance, FIXED_SCALE_BITS)
        };

        let x = ((this + POSITION_X) as *const u32).read_unaligned();
        let y = ((this + POSITION_Y) as *const u32).read_unaligned();
        let z = ((this + POSITION_Z) as *const u32).read_unaligned();
        let heading = ((this + POSITION_HEADING) as *const u32).read_unaligned();
        ((destination + POSITION_X) as *mut u32).write_volatile(x);
        ((destination + POSITION_Y) as *mut u32).write_volatile(y);
        ((destination + POSITION_Z) as *mut u32).write_volatile(z);
        ((destination + POSITION_HEADING) as *mut u32).write_volatile(heading);
        destination
    }
});
