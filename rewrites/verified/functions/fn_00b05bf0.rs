// original: 0x00b05bf0 create_and_attach_child
/// Create a child record and attach it to the parent.
///
/// thiscall `(this, v, b1, b2)`: makes a child (helper 1, thiscall, no
/// stack args), stores the full word `v` at child `+4`, the low byte of
/// `b1` at `+0xd`, zeroes `+0` and `+8`, stores the low byte of `b2` at
/// `+0xc`, then attaches it (helper 2, thiscall `(this, child)`).
/// Returns helper 2's answer.
export!(thiscall, rw_00b05bf0(this: u32, v: u32, b1: u32, b2: u32) -> u32 {
    let child: u32 = callee_thiscall!(1, u32, this);
    unsafe {
        ((child + 4) as *mut u32).write_unaligned(v);
        ((child + 0xD) as *mut u8).write(b1 as u8);
        (child as *mut u32).write_unaligned(0);
        ((child + 8) as *mut u32).write_unaligned(0);
        ((child + 0xC) as *mut u8).write(b2 as u8);
    }
    let r: u32 = callee_thiscall!(2, u32, this, child);
    r
});
