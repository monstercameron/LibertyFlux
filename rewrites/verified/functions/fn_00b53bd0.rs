// original: 0x00b53bd0 manager_array_ctor
/// Constructor: builds the head object, constructs a 32-element array
/// of embedded entries and stamps their vtable, constructs two trailing
/// sub-objects, links the second trailing object through its slot-1
/// virtual, then links every array element through its slot-1 virtual
/// while recording element and table pointers. Returns `this`.
export!(thiscall, rw_00b53bd0(this: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        *(this as *mut u32) = relocated(0xEAF328);
        let mut elem = (this as u32).wrapping_add(0x0C);
        let mut i = 0x1Fu32;
        loop {
            callee_thiscall!(2, u32, elem);
            *(elem as *mut u32) = relocated(0xEAF314);
            elem = elem.wrapping_add(0x18);
            if i == 0 {
                break;
            }
            i -= 1;
        }
        callee_thiscall!(3, u32, (this as u32).wrapping_add(0x38C));
        let tail = (this as u32).wrapping_add(0x80C);
        callee_thiscall!(2, u32, tail);
        *this.add(0x824) = 0;
        let tail_vt = *(tail as *const u32);
        let tail_tgt = *((tail_vt as *const u8).add(4) as *const u32);
        let link_tail: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(tail_tgt as usize);
        link_tail(tail);
        let mut e = (this as u32).wrapping_add(0x0C);
        let mut tab = (this as u32).wrapping_add(0x38C);
        let mut dst = (this as u32).wrapping_add(0x78C);
        let mut n = 0x20u32;
        while n != 0 {
            let vt = *(e as *const u32);
            let tgt = *((vt as *const u8).add(4) as *const u32);
            let link: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(tgt as usize);
            link(e);
            *((dst as *mut u8).sub(0x480) as *mut u32) = e;
            *(dst as *mut u32) = tab;
            e = e.wrapping_add(0x18);
            tab = tab.wrapping_add(0x20);
            dst = dst.wrapping_add(4);
            n -= 1;
        }
        this as u32
    }
});
