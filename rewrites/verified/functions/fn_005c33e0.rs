// original: 0x005C33E0 ped_factory_process_collection_slots

/// Processes the active entries in a global ped collection. The manager
/// stores an entry array, a per-index flag byte, a signed count and a byte
/// stride at offsets 0, 4, 8 and 12. Entries with flag bit 7 set, a null
/// address or a zero first word are skipped. A nonzero input byte selects
/// unconditional cleanup; otherwise an entry with a zero state word is
/// checked by the category helper and cleaned only when result bits 1, 2, 6
/// and 7 are all clear. The function is cdecl with one stack word and has no
/// defined return value.
lf_checker_rt::export!(cdecl, rw_005c33e0(mode: u32) -> u32 {
    unsafe {
        const MANAGER_GLOBAL: u32 = 0x015f8ba4;
        const CATEGORY_GLOBAL: u32 = 0x0103fc40;
        const ITEMS_POINTER: u32 = 0x00;
        const FLAGS_POINTER: u32 = 0x04;
        const SIGNED_COUNT: u32 = 0x08;
        const ENTRY_STRIDE: u32 = 0x0c;
        const ENTRY_PRESENT: u32 = 0x00;
        const ENTRY_STATE: u32 = 0x04;
        const SKIP_MASK: u8 = 0x80;
        const CATEGORY_RESULT_MASK: u8 = 0xc6;
        const CATEGORY_CALLEE: u32 = 1;
        const CLEANUP_CALLEE: u32 = 2;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_byte(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        let manager = lf_checker_rt::global::<u32>(MANAGER_GLOBAL).read_unaligned();
        let count = read_word(manager + SIGNED_COUNT) as i32;
        let item_base = read_word(manager + ITEMS_POINTER);
        let flags = read_word(manager + FLAGS_POINTER);
        let stride = read_word(manager + ENTRY_STRIDE);
        let category = lf_checker_rt::global::<u32>(CATEGORY_GLOBAL).read_unaligned();
        let always_cleanup = mode as u8 != 0;
        let mut index = 0u32;

        while (index as i32) < count {
            if read_byte(flags.wrapping_add(index)) & SKIP_MASK == 0 {
                let entry = item_base.wrapping_add(stride.wrapping_mul(index));
                if entry != 0 && read_word(entry + ENTRY_PRESENT) != 0 {
                    let needs_cleanup = if always_cleanup {
                        true
                    } else if read_word(entry + ENTRY_STATE) != 0 {
                        false
                    } else {
                        let category_result: u32 =
                            lf_checker_rt::callee_cdecl!(CATEGORY_CALLEE, u32, index, category);
                        category_result as u8 & CATEGORY_RESULT_MASK == 0
                    };
                    if needs_cleanup {
                        let _: u32 = lf_checker_rt::callee_cdecl!(CLEANUP_CALLEE, u32, index);
                    }
                }
            }
            index += 1;
        }
        0
    }
});
