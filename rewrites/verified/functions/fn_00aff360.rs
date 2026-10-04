// original: 0x00aff360 switch_selection
/// Switch the current selection: release the old one unless it is null,
/// store the new one and acquire it. Equal values are a no-op.
export!(cdecl, rw_00aff360(arg: u32) -> u32 {
    unsafe {
        let slot = global::<u32>(0x1600184);
        let old = *slot;
        if old != arg {
            if old != 0 {
                callee_thiscall!(1, u32, old, slot as u32);
            }
            *slot = arg;
            callee_thiscall!(2, u32, arg, slot as u32);
        }
        0
    }
});
