// original: 0x00b790b0 CTaskMoveInterface::~CTaskMoveInterface
/// Deleting destructor for `CTaskMoveInterface`.
///
/// Installs the base virtual table, then frees the object when bit 0 of
/// the flags argument is set. Returns the object pointer.
export!(thiscall, rw_00b790b0(this: u32, flags: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0x00EB2B98);
    }
    if flags & 1 != 0 {
        let _: u32 = callee_cdecl!(1, u32, this);
    }
    this
});
