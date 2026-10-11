// original: 0x005E7610 NativeImpl_RELOAD_WEB_PAGE
const WEB_PAGE_ARRAY_FILE_VA: u32 = 0x0118d7f0;
const WEB_PAGE_COUNT_FILE_VA: u32 = 0x0118d7f4;
const WEB_PAGE_ID_OFFSET: u32 = 0x53c;
const WEB_PAGE_SELECTOR_TITLE_FILE_VA: u32 = 0x0114e350;
const WEB_PAGE_SELECTOR_TEXT_FILE_VA: u32 = 0x0114e3a0;

/// Finds a web page by its identifier in the global page array, resolves a
/// selector for it, then asks the reload method to reload the selector's
/// embedded page view. The identifier field and comparison are unsigned
/// 32-bit values; a missing page reaches the selector helper as null. This
/// cdecl method takes one stack argument and returns the reload helper's EAX.
lf_checker_rt::export!(cdecl, rw_005e7610(page_id: u32) -> u32 {
    let page_count = unsafe {
        lf_checker_rt::global::<u16>(WEB_PAGE_COUNT_FILE_VA).read_unaligned()
    } as u32;
    let pages = unsafe {
        lf_checker_rt::global::<u32>(WEB_PAGE_ARRAY_FILE_VA).read_unaligned()
    };

    let mut selected_page = 0;
    let mut index = 0u32;
    while index < page_count {
        let page = unsafe {
            (pages.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned()
        };
        let candidate_id = unsafe {
            (page.wrapping_add(WEB_PAGE_ID_OFFSET) as *const u32).read_unaligned()
        };
        if candidate_id == page_id {
            selected_page = page;
            break;
        }
        index = index.wrapping_add(1);
    }

    let selector = unsafe {
        lf_checker_rt::callee_cdecl!(
            1,
            u32,
            selected_page,
            0,
            lf_checker_rt::relocated(WEB_PAGE_SELECTOR_TITLE_FILE_VA),
            lf_checker_rt::relocated(WEB_PAGE_SELECTOR_TEXT_FILE_VA),
            0
        )
    };
    let view = selector.wrapping_add(0x560);
    unsafe { lf_checker_rt::callee_thiscall!(2, u32, selector, view) }
});
