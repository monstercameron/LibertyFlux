// original: 0x00dfd8ac env_table_lookup
// rs03f09: name lookup in a global string table (cdecl/1).
//
// Searches the NULL-terminated global entry list for an entry longer than
// `name` carrying '=' right past the name length whose prefix then compares
// equal (cdecl/3 callee 2, zero on match), returning the address just past
// the '='. An empty list runs one initialization attempt (cdecl/0 callee 1)
// first. A clear enable flag, a null name or no match returns null.
//
// Lengths are measured inline: the original calls its own string helper for
// them, which the checker cannot stub with per-call answers, so those two
// call sites run natively on the original side while the rewrite measures
// directly; only the values are ever observed.
export!(cdecl, rw_rs03f09(name: u32) -> u32 {
    unsafe {
        if *global::<u32>(0x1C9B528) == 0 {
            return 0;
        }
        let mut list = *global::<u32>(0x17AB6D4);
        if list == 0 {
            if *global::<u32>(0x17AB6D8) == 0 {
                return 0;
            }
            if callee_cdecl!(1, u32,) != 0 {
                return 0;
            }
            list = *global::<u32>(0x17AB6D4);
            if list == 0 {
                return 0;
            }
        }
        if name == 0 {
            return 0;
        }
        let mut np = name as *const u8;
        while *np != 0 {
            np = np.add(1);
        }
        let nlen = (np as u32).wrapping_sub(name);
        let mut p = list as *const u32;
        loop {
            let entry = *p;
            if entry == 0 {
                return 0;
            }
            let mut ep = entry as *const u8;
            while *ep != 0 {
                ep = ep.add(1);
            }
            let elen = (ep as u32).wrapping_sub(entry);
            if elen > nlen
                && *((entry.wrapping_add(nlen)) as *const u8) == 0x3D
                && callee_cdecl!(2, u32, entry, name, nlen) == 0
            {
                return entry.wrapping_add(nlen).wrapping_add(1);
            }
            p = p.add(1);
        }
    }
});
