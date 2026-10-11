// original: 0x005E7670 NativeImpl_GET_WEB_PAGE_HEIGHT
const WEB_PAGE_ARRAY_FILE_VA: u32 = 0x0118d7f0;
const WEB_PAGE_COUNT_FILE_VA: u32 = 0x0118d7f4;
const WEB_PAGE_ID_OFFSET: u32 = 0x53c;
const WEB_PAGE_SELECTOR_OFFSET: u32 = 0x760;
const WEB_PAGE_HEIGHT_OBJECT_OFFSET: u32 = 4;
const WEB_PAGE_HEIGHT_VALUE_OFFSET: u32 = 0x24;
const WEB_PAGE_SELECTOR_TITLE_FILE_VA: u32 = 0x0114e350;
const WEB_PAGE_SELECTOR_TEXT_FILE_VA: u32 = 0x0114e3a0;

/// Looks up a web page by its 32-bit identifier, asks the selector helper
/// for the page view, and returns the single-precision height stored at
/// offset `0x24` in that view. A missing identifier reaches the helper as a
/// null page pointer. This cdecl function takes one stack argument and
/// returns its float in the x87 register stack.
lf_checker_rt::export!(cdecl, rw_005e7670(page_id: u32) -> f32 {
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
    let height_object = unsafe {
        (selector.wrapping_add(WEB_PAGE_SELECTOR_OFFSET) as *const u32).read_unaligned()
    };
    let height_data = unsafe {
        (height_object.wrapping_add(WEB_PAGE_HEIGHT_OBJECT_OFFSET) as *const u32)
            .read_unaligned()
    };
    let height_bits = unsafe {
        (height_data.wrapping_add(WEB_PAGE_HEIGHT_VALUE_OFFSET) as *const u32)
            .read_unaligned()
    };
    f32::from_bits(height_bits)
});
