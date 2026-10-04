// original: 0x00be44a0 CTaskComplexWaitForDryWeather::vf18 (symbols)

/// Delegate to the next task operation once the sky is dry enough, else null.
///
/// Compares the global moisture reading against the fixed threshold
/// `DRY_AT` (0.2): while the reading is below it (or unordered) this returns
/// zero. Once dry, it tail-jumps to the operation in this object's vtable
/// slot `NEXT_SLOT` (0x4c) with this object and the incoming argument. The
/// rewrite expresses the tail jump as a plain forwarding call; the calls and
/// result match, while the stack pointer legitimately differs by the pushed
/// return address (see the contract), because a jump is not a call.
///
/// Original: 0x00be44a0 (thiscall, one stack word: the incoming argument).
lf_checker_rt::export!(thiscall, rw_00be44a0(this: u32, arg: u32) -> u32 {
    unsafe {
        const MOISTURE: u32 = 0x012ddeac;
        const DRY_AT_BITS: u32 = 0x3e4ccccd; // 0.2f
        const NEXT_SLOT: u32 = 0x4c;
        let reading = f32::from_bits(
            (lf_checker_rt::global::<u32>(MOISTURE) as *const u32).read_unaligned(),
        );
        if reading >= f32::from_bits(DRY_AT_BITS) {
            let vtable = (this as *const u32).read_unaligned();
            let target = (vtable.wrapping_add(NEXT_SLOT) as *const u32).read_unaligned();
            let next: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            return next(this, arg);
        }
        0
    }
});
