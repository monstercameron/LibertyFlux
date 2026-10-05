// original: 0x008e0450 find_slot_and_emit_items

/// Find `target` in the pool chain starting at the entry's signed 16-bit
/// word `+0x48`, following each row's word `+0xc` until the target or -1,
/// then emit the entry's items through the item handler and report whether
/// the entry's secondary object is live.
///
/// A start of -1, or a chain that reaches -1, misses and returns the last
/// row address with its low byte cleared (zero when nothing was walked: the
/// original returns whatever the entry `eax` held, so the contract pins it).
/// A dead flag on the walk faults on a null row read, like the original. On
/// a hit the item list is resolved through word `+8` (`+0xb4` of what it
/// points at) or, when that is null, through word `+0xc`, and each item is
/// emitted with `extra` forwarded; the return is 1 when word `+0xc` points
/// at an object whose word `+4` is non-zero, else 0.
///
/// Original: cdecl (entry, target, extra).
lf_checker_rt::export!(cdecl, rw_008e0450(entry: *mut u8, target: u32, extra: u32) -> u32 {
    unsafe {
        let e = entry as *mut u32;
        let mut ecx = *(entry.add(0x48) as *const i16) as i32 as u32;
        if ecx == 0xFFFFFFFF {
            return 0;
        }
        let g = *lf_checker_rt::global::<u32>(0x11764C0) as *mut u32;
        let flags = *g.add(1) as *const u8;
        let stride = *g.add(3);
        let base = *g as *mut u8;
        let mut eax: u32 = 0;
        loop {
            if ecx == target {
                break;
            }
            if *flags.add(ecx as usize) & 0x80 != 0 {
                eax = 0;
            } else {
                eax = (base as u32).wrapping_add(ecx.wrapping_mul(stride));
            }
            ecx = *(eax as *const u32).add(3);
            if ecx == 0xFFFFFFFF {
                return eax & 0xFFFFFF00;
            }
        }
        let mut list = *e.add(2);
        if list != 0 {
            list = *((list as *mut u32).add(0x2d));
        } else {
            list = *e.add(3);
            if list != 0 {
                list = *(list as *const u32);
            }
        }
        if list != 0 {
            let row2 = *((list as *mut u32).add(2)) as *mut u32;
            let inner = *(row2.add(3) as *const u16) as u32;
            if inner != 0 {
                let items = *row2.add(2) as *mut u32;
                let mut j = 0u32;
                while j < inner {
                    let it = *items.add(j as usize);
                    lf_checker_rt::callee_thiscall!(1, u32, it.wrapping_add(0x14), extra);
                    j += 1;
                }
            }
        }
        let tail = *e.add(3);
        if tail != 0 && *((tail as *mut u32).add(1)) != 0 {
            1
        } else {
            0
        }
    }
});
