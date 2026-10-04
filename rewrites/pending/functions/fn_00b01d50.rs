// original: 0x00b01d50 release_ui_slot
/// Reset the sub-object at this+0x418 through its own routine, then
/// invalidate its tag word. Returns the reset routine's result.
export!(thiscall, rw_00b01d50(this_: *mut u8) -> u32 {
    let slot = unsafe { this_.add(0x418) };
    let r: u32 = callee_thiscall!(1, u32, slot as u32);
    unsafe {
        *(slot as *mut u32) = 0xFFFF_FFFF;
    }
    r
});
