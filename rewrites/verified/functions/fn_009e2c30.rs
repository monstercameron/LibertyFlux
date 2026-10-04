// original: 0x009e2c30 audPlaceableTracker::~audPlaceableTracker__deleting
/// 0x009E2C30 (audPlaceableTracker deleting destructor): stamp the vtable,
/// run the base destructor, and free the object when flags bit 0 is set.
/// Returns the object pointer. (thiscall/1)
export!(thiscall, rw_009e2c30(this: *mut u8, flags: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0xE985D8);
        callee_thiscall!(1, u32, this as u32);
        if flags & 1 != 0 {
            callee_cdecl!(2, u32, this as u32);
        }
        this as u32
    }
});
