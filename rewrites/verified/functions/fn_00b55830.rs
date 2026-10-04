// original: 0x00b55830 dllist_unlink_10_14
/// Unlink this node from a doubly linked list whose links live at +0x10/+0x14.
///
/// If the previous node exists, its next link (+0x14) is set to this node's
/// next; if the next node exists, its previous link (+0x10) is set to this
/// node's previous. Both links on this node are then cleared. The return
/// value is whatever happened to be in EAX (the original never writes it).
export!(thiscall, rw_00b55830(this: u32) -> u32 {
    unsafe {
        const PREV: u32 = 0x10;
        const NEXT: u32 = 0x14;
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
