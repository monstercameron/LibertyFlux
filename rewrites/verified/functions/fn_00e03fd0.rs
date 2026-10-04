// original: 0x00e03fd0 find_section_by_rva
/// Finds the section header whose address range contains the target.
///
/// Walks the section table of the module at `base` (via e_lfanew and the
/// COFF header) and returns a pointer to the first entry with
/// VirtualAddress <= target < VirtualAddress + VirtualSize, or null.
export!(cdecl, rw_00e03fd0(base: u32, target: u32) -> u32 {
    unsafe {
        let nt = base.wrapping_add(*((base + 0x3C) as *const u32));
        let count = *((nt + 6) as *const u16) as u32;
        let mut sec = nt
            .wrapping_add(*((nt + 0x14) as *const u16) as u32)
            .wrapping_add(0x18);
        let mut i = 0u32;
        while i < count {
            let vaddr = *((sec + 0xC) as *const u32);
            if target >= vaddr
                && target < vaddr.wrapping_add(*((sec + 8) as *const u32))
            {
                return sec;
            }
            i += 1;
            sec = sec.wrapping_add(0x28);
        }
        0
    }
});
