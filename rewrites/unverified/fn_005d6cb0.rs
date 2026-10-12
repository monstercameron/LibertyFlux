// original: 0x005d6cb0 html_link_hit_test
/// Return the index of the first link rectangle containing the supplied
/// screen point, or -1 when no item matches. The object stores a u16 item
/// count at +0x1c and a pointer array at +0x18. Each item provides left,
/// top, right and bottom floats at +0x20, +0x24, +0x2c and +0x30. A left
/// edge of -1.0 on type 0x12 follows the parent pointer at +8; a top edge
/// with that sentinel also triggers a parent walk. When the left edge is at
/// or below the image's clip
/// threshold, the selected horizontal item's margins at +0xb4 and +0xb0
/// are subtracted from its left edge. The vertical parent walk follows the
/// same sentinel/type condition and stops if that walk reaches a null parent;
/// the final vertical limit uses the original item's top and bottom fields.
/// X and Y arrive in XMM1 and XMM2.
/// Comparisons preserve unordered-float branch behavior; thiscall returns
/// the zero-based index in EAX or -1 when no item matches.
lf_checker_rt::export!(thiscall, rw_005d6cb0(this: u32) -> u32 {
    const ITEM_COUNT_OFFSET: u32 = 0x1c;
    const ITEM_LIST_OFFSET: u32 = 0x18;
    const ITEM_TYPE_OFFSET: u32 = 0x14;
    const ITEM_PARENT_OFFSET: u32 = 0x08;
    const LEFT_OFFSET: u32 = 0x20;
    const TOP_OFFSET: u32 = 0x24;
    const RIGHT_OFFSET: u32 = 0x2c;
    const BOTTOM_OFFSET: u32 = 0x30;
    const HORIZONTAL_MARGIN_A_OFFSET: u32 = 0xb4;
    const HORIZONTAL_MARGIN_B_OFFSET: u32 = 0xb0;
    const INHERITED_EDGE_TYPE: u32 = 0x12;
    const SENTINEL_ADDRESS: u32 = 0x00fe_8d94;
    const CLIP_THRESHOLD_ADDRESS: u32 = 0x00fe_8b48;

    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn read_f32(address: u32) -> f32 {
        f32::from_bits(unsafe { read_u32(address) })
    }
    #[inline(always)]
    fn add_ss(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }
    #[inline(always)]
    fn sub_ss(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    }
    #[inline(always)]
    fn below_or_unordered(left: f32, right: f32) -> bool {
        left < right || left.is_nan() || right.is_nan()
    }
    #[inline(always)]
    fn below_equal_or_unordered(left: f32, right: f32) -> bool {
        left <= right || left.is_nan() || right.is_nan()
    }

    unsafe {
        let item_count = read_u16(this.wrapping_add(ITEM_COUNT_OFFSET)) as u32;
        if item_count == 0 {
            return u32::MAX;
        }

        let item_list = read_u32(this.wrapping_add(ITEM_LIST_OFFSET));
        let point_x = f32::from_bits(lf_checker_rt::xmm_word(1, 0));
        let point_y = f32::from_bits(lf_checker_rt::xmm_word(2, 0));
        let sentinel = read_f32(lf_checker_rt::relocated(SENTINEL_ADDRESS));
        let clip_threshold = read_f32(lf_checker_rt::relocated(CLIP_THRESHOLD_ADDRESS));

        let mut index = 0u32;
        while index < item_count {
            let item = read_u32(item_list.wrapping_add(index.wrapping_mul(4)));
            let mut left = read_f32(item.wrapping_add(LEFT_OFFSET));
            let right = read_f32(item.wrapping_add(RIGHT_OFFSET));
            let bottom = read_f32(item.wrapping_add(BOTTOM_OFFSET));
            let top = read_f32(item.wrapping_add(TOP_OFFSET));

            if !below_or_unordered(clip_threshold, left) {
                let mut horizontal_item = item;
                loop {
                    let edge = read_f32(horizontal_item.wrapping_add(LEFT_OFFSET));
                    let item_type = read_u32(horizontal_item.wrapping_add(ITEM_TYPE_OFFSET));
                    if edge != sentinel || item_type != INHERITED_EDGE_TYPE {
                        left = edge;
                        break;
                    }
                    horizontal_item = read_u32(horizontal_item.wrapping_add(ITEM_PARENT_OFFSET));
                }
                let margin_a = read_f32(horizontal_item.wrapping_add(HORIZONTAL_MARGIN_A_OFFSET));
                let margin_b = read_f32(horizontal_item.wrapping_add(HORIZONTAL_MARGIN_B_OFFSET));
                left = sub_ss(left, add_ss(margin_a, margin_b));
            }

            if below_or_unordered(point_x, right) {
                index = index.wrapping_add(1);
                continue;
            }
            let horizontal_limit = add_ss(left, right);
            if below_equal_or_unordered(horizontal_limit, point_x) {
                index = index.wrapping_add(1);
                continue;
            }

            let mut vertical_item = item;
            loop {
                let scanned_top = read_f32(vertical_item.wrapping_add(TOP_OFFSET));
                let item_type = read_u32(vertical_item.wrapping_add(ITEM_TYPE_OFFSET));
                if scanned_top != sentinel || item_type != INHERITED_EDGE_TYPE {
                    break;
                }
                let parent = read_u32(vertical_item.wrapping_add(ITEM_PARENT_OFFSET));
                if parent == 0 {
                    break;
                }
                vertical_item = parent;
            }

            if below_or_unordered(point_y, bottom) {
                index = index.wrapping_add(1);
                continue;
            }
            let vertical_limit = add_ss(top, bottom);
            if vertical_limit > point_y {
                return index;
            }

            index = index.wrapping_add(1);
        }
        u32::MAX
    }
});
