// original: 0x00c6e170 anim_release_buffer
/// Clear the two state words, then release the buffer at `this+0x10` if set:
/// its inner block, its data block, then the header itself, nulling the slot.
export!(thiscall, rw_00c6e170(this: u32) -> () {
    unsafe {
        *(this as *mut u32).add(2) = 0;
        *(this as *mut u32).add(3) = 0;
        let p = *((this + 0x10) as *const u32);
        if p != 0 {
            let inner = *(p as *const u32);
            let buf = *((p + 0x0c) as *const u32);
            if buf != 0 {
                callee_cdecl!(1, u32, buf);
            }
            if inner != 0 {
                callee_cdecl!(1, u32, inner);
            }
            callee_cdecl!(1, u32, p);
            *((this + 0x10) as *mut u32) = 0;
        }
    }
});
