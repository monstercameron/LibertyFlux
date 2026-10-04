// original: 0x00b55860 dllist_unlink_8c_90
/// Unlink this node from a doubly linked list whose links live at +0x8c/+0x90.
///
/// Same shape as fn_00b55830 with different link offsets: the neighbours are
/// relinked around this node, then both links on this node are cleared. The
/// return value is whatever happened to be in EAX (never written).
export!(thiscall, rw_00b55860(this: u32) -> u32 {
    unsafe {
        const PREV: u32 = 0x8c;
        const NEXT: u32 = 0x90;
        let prev = ((this + PREV) as *const u32).read();
        if prev != 0 {
            ((prev + NEXT) as *mut u32).write(((this + NEXT) as *const u32).read());
        }
        let next = ((this + NEXT) as *const u32).read();
        if next != 0 {
            ((next + PREV) as *mut u32).write(((this + PREV) as *const u32).read());
        }
        ((this + NEXT) as *mut u32).write(0);
        ((this + PREV) as *mut u32).write(0);
        0
    }
});
