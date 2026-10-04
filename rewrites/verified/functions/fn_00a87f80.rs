// original: 0x00A87F80 activate_phase_outputs
/// Bring the phase's outputs up and bind its sinks.
///
/// Stores the four blend constants, and when the readiness check fails
/// selects the fallback level pair from the mode word. Then runs the member
/// initializer, raises the ready flags, applies the pending tag reset when
/// the tag word or the guarded pair requests it, binds the two global sinks
/// and the member sink, and returns the finalizer's answer.
export!(thiscall, rw_a87f80(this: u32) -> u32 {
    const BLEND0: u32 = 0x3F2A_A64C;
    const BLEND1: u32 = 0x3F80_0000;
    const MODE: u32 = 0x011D_6FD0;
    const MODE2: u32 = 0x011D_6FD4;
    const LEVEL_A: u32 = 0x0103_B924;
    const LEVEL_B: u32 = 0x0103_B925;
    const LEVEL2_A: u32 = 0x0103_BFF8;
    const LEVEL2_B: u32 = 0x0103_BFF9;
    const TAG: u32 = 0x0103_9298;
    const SINK_A: u32 = 0x0128_E310;
    const SINK_B: u32 = 0x0128_E400;
    unsafe {
        ((this + 0x2C) as *mut u32).write(BLEND0);
        ((this + 0x30) as *mut u32).write(BLEND1);
        ((this + 0x34) as *mut u32).write(BLEND1);
        ((this + 0x38) as *mut u32).write(BLEND1);
    }
    if callee_thiscall!(1, u32, this) == 0 {
        let mode = unsafe { global::<u32>(MODE).read() };
        let level = if mode == 1 || mode == 2 { 1u8 } else { 2u8 };
        unsafe {
            global::<u8>(LEVEL_A).write(level);
            global::<u8>(LEVEL_B).write(level);
            global::<u8>(LEVEL2_A).write(2);
            global::<u8>(LEVEL2_B).write(2);
        }
    }
    callee_thiscall!(2, u32, this);
    unsafe { ((this + 0x20) as *mut u8).write(1) };
    let tag = unsafe { global::<u32>(TAG).read() };
    unsafe {
        global::<u8>(0x0104_90A8).write(1);
        global::<u8>(0x012B_D1BF).write(0);
        global::<u8>(0x012B_D193).write(0);
        global::<u8>(0x0103_C111).write(1);
    }
    let guarded = unsafe { global::<u32>(MODE2).read() == 1 && global::<u8>(LEVEL_A).read() == 5 };
    if tag == 5 || guarded {
        unsafe {
            global::<u8>(LEVEL_A).write(2);
            global::<u32>(TAG).write(u32::MAX);
        }
    }
    callee_thiscall!(3, u32, relocated(SINK_A));
    callee_thiscall!(4, u32, relocated(SINK_B));
    // The final call inherits whatever the member-sink stub leaves in ecx,
    // so it is made as cdecl: ecx is not an input there.
    callee_thiscall!(5, u32, this.wrapping_add(4));
    let answer = callee_cdecl!(6, u32,);
    unsafe { ((this + 0x21) as *mut u8).write(1) };
    answer
});
