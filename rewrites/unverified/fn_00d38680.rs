// original: 0x00D38680 classify_seat_position

/// Classify a pair of seat-position values. The first cdecl argument selects
/// one byte from an eleven-entry category table using wrapping unsigned
/// addition by seven. The selected category accepts direction zero, accepts
/// any direction, requires direction two, accepts directions one or three,
/// compares the direction with the category index, or rejects the pair.
/// If that test rejects, a second wrapping unsigned bound check on the first
/// argument can return 1 when the second argument is nonzero; all other pairs
/// return 0. The function reads no object memory and makes no calls.
lf_checker_rt::export!(cdecl, rw_D38680(position_class: u32, direction: u32) -> u32 {
    const TABLE_LIMIT: u32 = 10;
    const ZERO_DIRECTION: u32 = 0;
    const CATEGORY_TWO_DIRECTION: u32 = 2;
    const SPECIAL_DIRECTION_A: u32 = 1;
    const SPECIAL_DIRECTION_B: u32 = 3;
    const SECOND_BOUND: u32 = 4;
    const CLASS_LOOKUP_TABLE: u32 = 0x00D38700;

    let table_index = position_class.wrapping_add(7);
    let matches_pair = if table_index > TABLE_LIMIT {
        false
    } else {
        let table_base = lf_checker_rt::relocated(CLASS_LOOKUP_TABLE);
        let selector = unsafe { core::ptr::read((table_base + table_index) as *const u8) };
        match selector {
            0 => direction == ZERO_DIRECTION,
            1 => true,
            2 => direction == CATEGORY_TWO_DIRECTION,
            3 => direction == SPECIAL_DIRECTION_A || direction == SPECIAL_DIRECTION_B,
            4 => direction == position_class,
            _ => false,
        }
    };

    if matches_pair {
        2
    } else if position_class.wrapping_add(6) <= SECOND_BOUND && direction != 0 {
        1
    } else {
        0
    }
});
