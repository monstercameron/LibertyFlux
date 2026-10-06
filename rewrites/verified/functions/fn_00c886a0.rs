// original: 0x00c886a0 audPlaceableTracker::vf1
///
/// `this+8` points at a record whose word at +0x20 is an optional child
/// pointer. Returns `child+0x30` when the child is non-null, else
/// `record+0x10`. Thiscall, one ignored stack word (callee pops 4).

lf_checker_rt::export!(thiscall, rw_00c886a0(this: u32, _unused: u32) -> u32 {
    unsafe {

        let rec = ((this.wrapping_add(8)) as *const u32).read_unaligned();
        let child = ((rec.wrapping_add(0x20)) as *const u32).read_unaligned();
        if child != 0 {
            child.wrapping_add(0x30)
        } else {
            rec.wrapping_add(0x10)
        }
    }
});
