// original: 0x008e03b0 find_slot_and_refresh_rows

/// Find `target` in the pool chain starting at the entry's word `+0xc`,
/// following each row's word `+0xc` until the target or -1, then refresh
/// every row of the entry's table through the item handler.
///
/// A start of -1, or a chain that reaches -1, misses and returns the last
/// row address with its low byte cleared (zero when nothing was walked: the
/// original returns whatever the entry `eax` held, so the contract pins it).
/// A dead flag on the walk faults on a null row read, like the original. On
/// a hit, each of the table's rows (counted by the 16-bit word at table
/// `+0x14`) has its items (counted by the word at row2 `+0xc`) refreshed
/// with `extra` forwarded, and the return is the last handler answer with
/// its low byte set (or the last row address, or the count, when no handler
/// ran after it: the original returns whatever `eax` last held).
///
/// Original: cdecl (entry, target, extra).
lf_checker_rt::export!(cdecl, rw_008e03b0(entry: *mut u32, target: u32, extra: u32) -> u32 {
    unsafe {
        let mut ecx = *entry.add(3);
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
        let tab = *entry as *mut u32;
        let count = *(tab.add(5) as *const u16) as u32;
        if count == 0 {
            return 1;
        }
        let rows = *tab.add(6) as *mut u32;
        let mut last = count;
        let mut i = 0u32;
        while i < count {
            let row = *rows.add(i as usize) as *mut u32;
            last = row as u32;
            let row2 = *row.add(2) as *mut u32;
            let inner = *(row2.add(3) as *const u16) as u32;
            if inner != 0 {
                let items = *row2.add(2) as *mut u32;
                let mut j = 0u32;
                while j < inner {
                    let it = *items.add(j as usize);
                    last = lf_checker_rt::callee_thiscall!(1, u32, it.wrapping_add(0x14), extra);
                    j += 1;
                }
            }
            i += 1;
        }
        (last & 0xFFFFFF00) | 1
    }
});
