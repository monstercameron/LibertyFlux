// original: 0x005E7730 NativeImpl_GET_WEB_PAGE_LINK_HREF
const WEB_PAGE_ARRAY_FILE_VA: u32 = 0x0118d7f0;
const WEB_PAGE_COUNT_FILE_VA: u32 = 0x0118d7f4;
const WEB_PAGE_ID_OFFSET: u32 = 0x53c;
const WEB_PAGE_SELECTOR_OFFSET: u32 = 0x760;
const WEB_PAGE_LINK_TABLE_OFFSET: u32 = 0x18;
const WEB_PAGE_LINK_HREF_OFFSET: u32 = 0xe0;
const WEB_PAGE_LINK_HREF_STATUS_OFFSET: u32 = 0xe4;
const WEB_PAGE_SELECTOR_TITLE_FILE_VA: u32 = 0x0114e350;
const WEB_PAGE_SELECTOR_TEXT_FILE_VA: u32 = 0x0114e3a0;
const DEFAULT_WEB_PAGE_HREF_VA: u32 = 0x00fc9c85;

/// Finds the page by its 32-bit identifier, obtains the selector's indexed
/// link record, and returns that record's href pointer when its 16-bit status
/// is nonzero. Otherwise it returns the relocated default href pointer. The
/// second stack argument is an unsigned link index. A missing page is passed
/// as null to the selector helper; this cdecl function returns in EAX.
lf_checker_rt::export!(cdecl, rw_005e7730(page_id: u32, link_index: u32) -> u32 {
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
    let links = unsafe {
        (link_list.wrapping_add(WEB_PAGE_LINK_TABLE_OFFSET) as *const u32).read_unaligned()
    };
    let link = unsafe {
        (links.wrapping_add(link_index.wrapping_mul(4)) as *const u32).read_unaligned()
    };
    let has_href = unsafe {
        (link.wrapping_add(WEB_PAGE_LINK_HREF_STATUS_OFFSET) as *const u16).read_unaligned()
    };
    if has_href == 0 {
        lf_checker_rt::relocated(DEFAULT_WEB_PAGE_HREF_VA)
    } else {
        unsafe {
            (link.wrapping_add(WEB_PAGE_LINK_HREF_OFFSET) as *const u32).read_unaligned()
        }
    }
});
