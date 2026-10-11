// original: 0x00DE4800 UIGTAScrollingRow::~UIGTAScrollingRow__deleting
/// Run the row's base destructor and, when bit zero of the deletion flags is
/// set, pass the row to the allocator's release routine. The method uses
/// thiscall with one stack argument, returns the original object pointer, and
/// lets the base and release routines own their side effects.
lf_checker_rt::export!(thiscall, rw_de4800(this: u32, deletion_flags: u32) -> u32 {
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
    if deletion_flags & 1 != 0 {
        let _ = lf_checker_rt::callee_cdecl!(2, u32, this);
    }
    this
});

