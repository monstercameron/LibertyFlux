// original: 0x0088F700 
// 0088F700 audSound reset: stamp the vtable, first time pick the voice kind
// through the kind table, otherwise re-resolve a live voice, then drop the
// aux byte and buffer.
export!(thiscall, rw_0088f700(this: *mut u8) -> () {
    unsafe {
        let mode = *this.add(0x3A);
        *(this as *mut u32) = relocated(0xE7854C);
        if mode & 0x10 == 0 {
            *this.add(0x3A) = mode | 0x10;
            let sel = *this.add(0x3B);
            let target = *global::<u32>(0x115D774 + (sel as u32) * 4);
            let pick: extern "cdecl" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            pick(this as u32);
        } else if mode & 2 != 0 {
            let sel = *this.add(4);
            if sel != 0xFF {
                let row = (*this.add(0x40) as u32).wrapping_mul(0x6F40);
                let stride = *global::<u32>(0x115D968);
                let base = *global::<u32>(0x115D988);
                let entry = *((row.wrapping_add(base).wrapping_add(0x6F14)) as *const u32);
                let pool = stride.wrapping_mul(sel as u32).wrapping_add(entry);
                callee_thiscall!(2, u32, pool);
                callee_thiscall!(
                    3,
                    u32,
                    relocated(0x115D8A0),
                    *this.add(0x40) as u32,
                    sel as u32
                );
            }
        }
        let aux = *(this.add(0xA8) as *const u32);
        *(this.add(0xA4) as *mut u32) = 0xFFFF_FFFF;
        if aux != 0 {
            *(aux as *mut u8) = 0;
            *(this.add(0xA8) as *mut u32) = 0;
        }
        let buf = *(this.add(0xAC) as *const u32);
        if buf != 0 {
            *(buf as *mut u32) = 0;
            *(this.add(0xAC) as *mut u32) = 0;
        }
    }
});
