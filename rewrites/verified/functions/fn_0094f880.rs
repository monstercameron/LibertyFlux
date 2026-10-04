// original: 0x0094f880 rage::atDNode<CReplayMgr::CInterpInfo*,rage::datBase>::~datBase>
//
// (batch list: rage::atDNode<CReplayMgr::CInterpInfo*, rage::datBase>::vf0)
/// Deleting destructor: restores the base vtable, runs the base destructor,
/// and frees the object when the low flag bit is set. Returns the object.
export!(thiscall, rw_0094f880(this_obj: u32, flags: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00E8_AE8C;
        *(this_obj as *mut u32) = relocated(VTABLE);
        callee_thiscall!(1, u32, this_obj);
        if flags & 1 != 0 {
            callee_cdecl!(2, u32, this_obj);
        }
        this_obj
    }
});
