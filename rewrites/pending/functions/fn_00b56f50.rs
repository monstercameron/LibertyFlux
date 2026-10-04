// original: 0x00b56f50 release_child_and_refresh
// rs05f2: release a child object and refresh the owner.
//
// When `child` is null the function does nothing. Otherwise it releases the
// child (thiscall/0) and then refreshes the owner through a helper reached
// by a jump thunk (thiscall/0).
export!(thiscall, rw_b56f50(this: *const u8, child: *const u8) -> () {
    unsafe {
        if child.is_null() {
            return;
        }
        let release: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        release(child as u32);
        let refresh: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        refresh(this as u32);
    }
});
