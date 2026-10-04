// original: 0x00945bc0 kind_table_search
/// Search a kind-selected table for a target value.
///
/// The low byte of the kind selects one of three table pointers from globals
/// (other kinds go through a resolver call). A null table, or a zero count,
/// answers false. Otherwise up to `count` entries are probed: entry index
/// `(base - i + 9) % 10` where `base` is a table header byte, compared
/// against the dword behind the data pointer. Answers true on first match.
/// The entry array sits at a misaligned offset, hence unaligned reads.
lf_checker_rt::export!(stdcall, rw_00945bc0(a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        let esi: u32 = match (a2 & 0xff) as u8 {
            4 => *(lf_checker_rt::global::<u32>(0x11d7640) as *const u32),
            3 => *(lf_checker_rt::global::<u32>(0x11d7644) as *const u32),
            0 => *(lf_checker_rt::global::<u32>(0x11d7648) as *const u32),
            _ => lf_checker_rt::callee_stdcall!(1, u32, a2),
        };
        if esi == 0 {
            return 0;
        }
        let count = (a3 & 0xff) as u32;
        if count == 0 {
            return 0;
        }
        let target = *((a1.wrapping_add(4)) as *const u32);
        let base = *(esi.wrapping_add(0xf) as *const u8) as u32;
        let mut bl: u32 = 0;
        loop {
            let d = base.wrapping_sub(bl).wrapping_add(9) % 10;
            let p = esi.wrapping_add(0x11).wrapping_add(d * 4) as *const u32;
            if core::ptr::read_unaligned(p) == target {
                return 1;
            }
            bl = (bl + 1) & 0xff;
            if bl >= count {
                return 0;
            }
        }
    }
});
