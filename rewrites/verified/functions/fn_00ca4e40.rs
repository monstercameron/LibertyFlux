// original: 0x00ca4e40 CEventHandler::~CEventHandler__deleting
/// Scalar deleting destructor: runs the destructor above, then frees this
/// object when the low bit of the flags argument is set. Returns this.
lf_rs75_rt::export!(thiscall, rw_00ca4e40(this: u32, flags: u32) -> u32 {
    let _: u32 = lf_rs75_rt::callee_thiscall!(1, u32, this);
    if flags & 1 != 0 {
        let _: u32 = lf_rs75_rt::callee_cdecl!(2, u32, this);
    }
    this
});
