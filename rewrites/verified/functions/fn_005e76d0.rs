// original: 0x005E76D0 NativeImpl_GET_NUMBER_OF_WEB_PAGE_LINKS
const WEB_PAGE_ARRAY_FILE_VA: u32 = 0x0118d7f0;
const WEB_PAGE_COUNT_FILE_VA: u32 = 0x0118d7f4;
const WEB_PAGE_ID_OFFSET: u32 = 0x53c;
const WEB_PAGE_SELECTOR_OFFSET: u32 = 0x760;
const WEB_PAGE_LINK_COUNT_OFFSET: u32 = 0x1c;
const WEB_PAGE_SELECTOR_TITLE_FILE_VA: u32 = 0x0114e350;
const WEB_PAGE_SELECTOR_TEXT_FILE_VA: u32 = 0x0114e3a0;

/// Finds a web page by its 32-bit identifier in the global page array, then
/// asks the selector helper for its link list. It returns the zero-extended
/// 16-bit link count at offset `0x1c` of that list. Page identifiers are
/// compared for equality; a missing identifier passes a null page pointer
/// to the helper. This cdecl function has one stack argument and returns in
/// EAX.
lf_checker_rt::export!(cdecl, rw_005e76d0(page_id: u32) -> u32 {
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
    let link_list = unsafe {
        (selector.wrapping_add(WEB_PAGE_SELECTOR_OFFSET) as *const u32).read_unaligned()
    };
    unsafe {
        u32::from(
            (link_list.wrapping_add(WEB_PAGE_LINK_COUNT_OFFSET) as *const u16)
                .read_unaligned(),
        )
    }
});
