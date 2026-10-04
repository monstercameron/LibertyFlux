// original: 0x0094f8b0 CAtdNodeFrameAddress::~CAtdNodeFrameAddress__deleting
//
// (batch list: CAtdNodeFrameAddress::vf0)
/// Deleting destructor through the shared allocator: restores the base
/// vtable, runs the base destructor, and when the low flag bit is set asks
/// the global allocator to release the object. Returns the object.
export!(thiscall, rw_0094f8b0(this_obj: u32, flags: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00E8_AE8C;
        const ALLOCATOR: u32 = 0x011F_6FEC;
        *(this_obj as *mut u32) = relocated(VTABLE);
        callee_thiscall!(1, u32, this_obj);
        if flags & 1 != 0 {
            let mgr = *global::<u32>(ALLOCATOR);
            callee_thiscall!(2, u32, mgr, this_obj);
        }
        this_obj
    }
});
