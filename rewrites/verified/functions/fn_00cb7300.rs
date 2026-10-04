// original: 0x00cb7300 route_index_machine_32
/// Route-index state machine described above; returns the disposition code.
export!(thiscall, rw_cb7300(this_ptr: u32) -> u32 {
    unsafe {
        loop {
            let table = ((this_ptr.wrapping_add(0x34)) as *const u32).read();
            let count = (table as *const u32).read();
            if count == 0 {
                return 0x516;
            }
            let idx = ((this_ptr.wrapping_add(0x38)) as *const u32).read() as i32;
            let next = idx.wrapping_add(1);
            let count_s = count as i32;
            if next < count_s {
                return 0x3AF;
            }
            if next != count_s {
                if idx != count_s {
                    return 0x11C;
                }
                let key = ((this_ptr.wrapping_add(0x20)) as *const u32).read();
                let ctrp = (this_ptr.wrapping_add(0x24)) as *mut u32;
                ctrp.write(ctrp.read().wrapping_add(1));
                if key > 3 {
                    return 0x11C;
                }
                match key {
                    0 => return 0x516,
                    1 => {
                        if ctrp.read() != 1 {
                            return 0x516;
                        }
                        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, table);
                        ((this_ptr.wrapping_add(0x38)) as *mut u32).write(0);
                    }
                    2 => {
                        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, table);
                        ((this_ptr.wrapping_add(0x38)) as *mut u32).write(0);
                    }
                    _ => {
                        ((this_ptr.wrapping_add(0x38)) as *mut u32).write(0);
                    }
                }
            } else {
                let flag = ((this_ptr.wrapping_add(0x3C)) as *const u8).read();
                if flag & 4 != 0 {
                    return 0x387;
                } else {
                    return 0x384;
                }
            }
        }
    }
});
