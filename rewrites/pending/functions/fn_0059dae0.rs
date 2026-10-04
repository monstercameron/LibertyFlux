// original: 0x0059dae0 poll_dual_flag
// True when flag A is set and flag B is clear (both global bytes).
//
// Returns the boolean in AL only; the upper bytes keep whatever the caller
// had in EAX, so the contract compares the `al` channel.
export!(cdecl, rw_0059DAE0() -> u32 {
    let a = unsafe { *global::<u8>(0x10376E9) };
    if a == 0 {
        return 0;
    }
    let b = unsafe { *global::<u8>(0x11F7076) };
    u32::from(b == 0)
});
