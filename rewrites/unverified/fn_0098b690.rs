// original: 0x0098b690 audVehicleAudioEntity::maybe_start_engine_loops
/// Starts the engine loop when the engine and audio state permit it.
///
/// A nonzero receiver field at `0x9e0` guards a pointer read at `0x820`. The
/// pointed-to byte at `0xf14` must have bit three set. The routine then calls
/// its thiscall helper when the loop-start byte at `0xb24` is zero, or when
/// that byte is nonzero and the receiver word at `0x9e4` equals three. A call
/// sets `0xb24` to one. Every path clears the byte at `0xb35`; the result is
/// unspecified and is not part of the contract.
const HAS_ENGINE: u32 = 0x9E0;
const ENGINE_POINTER: u32 = 0x820;
const ENGINE_FLAGS: u32 = 0xF14;
const ENGINE_LOOP_MASK: u8 = 8;
const LOOP_STARTED: u32 = 0xB24;
const LOOP_STATE: u32 = 0x9E4;
const LOOP_STATE_READY: u32 = 3;
const LOOP_RESET: u32 = 0xB35;
const START_LOOP_CALLEE: u32 = 1;

export!(thiscall, rw_0098b690(this_: u32) -> () {
    let has_engine = unsafe { (this_.wrapping_add(HAS_ENGINE) as *const u32).read() };
    if has_engine != 0 {
        let engine = unsafe { (this_.wrapping_add(ENGINE_POINTER) as *const u32).read() };
        let engine_flags = unsafe { (engine.wrapping_add(ENGINE_FLAGS) as *const u8).read() };
        if engine_flags & ENGINE_LOOP_MASK != 0 {
            let loop_started = unsafe { (this_.wrapping_add(LOOP_STARTED) as *const u8).read() };
            let may_start = loop_started == 0
                || unsafe { (this_.wrapping_add(LOOP_STATE) as *const u32).read() } == LOOP_STATE_READY;
            if may_start {
                let _helper_result = lf_checker_rt::callee_thiscall!(START_LOOP_CALLEE, u32, this_, 1u32);
                unsafe { (this_.wrapping_add(LOOP_STARTED) as *mut u8).write(1); }
            }
        }
    }
    unsafe { (this_.wrapping_add(LOOP_RESET) as *mut u8).write(0); }
});
