// original: 0x005E75B0 NativeImpl_LOAD_WEB_PAGE
const WEB_PAGE_ARRAY_FILE_VA: u32 = 0x0118d7f0;
const WEB_PAGE_COUNT_FILE_VA: u32 = 0x0118d7f4;
const WEB_PAGE_ID_OFFSET: u32 = 0x53c;
const WEB_PAGE_SELECTOR_TITLE_FILE_VA: u32 = 0x0114e350;
const WEB_PAGE_SELECTOR_TEXT_FILE_VA: u32 = 0x0114e3a0;

/// Finds a web page by its identifier in the global page array, asks the
/// selector helper to resolve it, then forwards that selector and the
/// caller's second stack word to the load method. The identifier and page field are
/// unsigned 32-bit values; a missing page is forwarded as a null pointer.
/// This cdecl function has two stack arguments: the page identifier and a
/// second word forwarded unchanged to the load method. It returns the load
/// method's EAX value.
lf_checker_rt::export!(cdecl, rw_005e75b0(page_id: u32, forwarded_arg: u32) -> u32 {
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
    unsafe { lf_checker_rt::callee_thiscall!(2, u32, selector, forwarded_arg) }
});
