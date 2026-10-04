// original: 0x00cb7170 collect_route_rows
/// Collects route rows into `out` as described above; returns the row count.
export!(thiscall, rw_cb7170(this_ptr: u32, out: u32) -> u32 {
    unsafe {
        (out as *mut u32).write(0);
        ((out.wrapping_add(4)) as *mut u32).write(0);
        let sub = ((this_ptr.wrapping_add(8)) as *const u32).read();
        let vptr = (sub as *const u32).read();
        let slot = ((vptr.wrapping_add(0xC)) as *const u32).read();
        let kind_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        if kind_of(sub) != 0x389 {
            return 0;
        }
        if ((this_ptr.wrapping_add(0xB0)) as *const u32).read() & 0x100 != 0 {
            return 0;
        }
        let sub2 = ((this_ptr.wrapping_add(8)) as *const u32).read();
        let table = ((sub2.wrapping_add(0x34)) as *const u32).read();
        let mut pos = ((sub2.wrapping_add(0x38)) as *const u32).read() as i32;
        if pos >= (table as *const u32).read() as i32 {
            return 0;
        }
        loop {
            let mark_addr =
                (this_ptr.wrapping_add(0x68)).wrapping_add((pos as u32).wrapping_mul(4));
            if (mark_addr as *const u16).read() != 0 {
                break;
            }
            let n = (out as *const u32).read();
            let src = (table as u32)
                .wrapping_add(0x18)
                .wrapping_add((pos as u32).wrapping_mul(16));
            let dst = (out as u32).wrapping_add(n.wrapping_add(1).wrapping_mul(16));
            (dst as *mut u32).write((src.wrapping_sub(8) as *const u32).read());
            ((dst.wrapping_add(4)) as *mut u32)
                .write((src.wrapping_sub(4) as *const u32).read());
            ((dst.wrapping_add(8)) as *mut u32).write((src as *const u32).read());
            ((dst.wrapping_add(12)) as *mut u32)
                .write((src.wrapping_add(4) as *const u32).read());
            (out as *mut u32).write(n.wrapping_add(1));
            pos = pos.wrapping_add(1);
            if pos >= (table as *const u32).read() as i32 {
                break;
            }
        }
        (out as *const u32).read()
    }
});
