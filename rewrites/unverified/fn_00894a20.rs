// original: 0x00894a20 factory_create_by_index (proposed)

/// Create one of 23 indexed sub-objects, or return null.
///
/// `index` selects the kind (1..=24) and `a0` is forwarded to the allocator
/// as its second stack word. For a live index the function allocates the
/// kind's fixed block size through the allocator object (a thiscall taking
/// `(size, a0, 0)`), returns 0 when that allocation fails, and otherwise
/// runs the kind's constructor (a thiscall of no stack arguments) on the
/// block and returns the block.
///
/// Index 15 has no kind and behaves exactly like an out-of-range index
/// (0, or anything above 24): the function returns 0 and calls nothing.
/// The original dispatches through a jump table over `index - 1` with an
/// unsigned bound of 23; index 15's table entry points at the same
/// return-null block as the out-of-range path.
///
/// Block sizes and constructor ids per index: 1:0xDC, 2:0xFC, 3:0xF8,
/// 4:0xB8, 5:0xB4, 6:0xB0, 7:0xD4, 8:0xE8, 9:0xE4, 10:0xB4, 11:0xD0,
/// 12:0xBC, 13:0xB8, 14:0xB0, 16:0xF4, 17:0xE0, 18:0xB4, 19:0xB0,
/// 20:0xB4, 21:0xBC, 22:0xC8, 23:0xE8, 24:0xB8.
///
/// Original: 0x00894a20 (cdecl, two stack words; no register inputs).
lf_checker_rt::export!(cdecl, rw_00894a20(a0: u32, index: u32) -> u32 {
    /// File VA of the allocator object passed in ECX to the allocator callee.
    const ALLOCATOR_OBJ: u32 = 0x0115_D8A0;
    /// Contract callee id of the allocator (thiscall, 3 stack args).
    const ALLOC_ID: u32 = 1;
    /// Allocate `size` bytes and run the `init_id` constructor on the
    /// block; 0 when allocation fails (the constructor never runs then).
    fn make(a0: u32, size: u32, init_id: u32) -> u32 {
        let block: u32 = lf_checker_rt::callee_thiscall!(
            ALLOC_ID, u32, lf_checker_rt::relocated(ALLOCATOR_OBJ), size, a0, 0);
        if block == 0 {
            0
        } else {
            let _: u32 = lf_checker_rt::callee_thiscall!(init_id, u32, block);
            block
        }
    }
    match index {
        1 => make(a0, 0xDC, 2),
        2 => make(a0, 0xFC, 3),
        3 => make(a0, 0xF8, 4),
        4 => make(a0, 0xB8, 5),
        5 => make(a0, 0xB4, 6),
        6 => make(a0, 0xB0, 7),
        7 => make(a0, 0xD4, 8),
        8 => make(a0, 0xE8, 9),
        9 => make(a0, 0xE4, 10),
        10 => make(a0, 0xB4, 11),
        11 => make(a0, 0xD0, 12),
        12 => make(a0, 0xBC, 13),
        13 => make(a0, 0xB8, 14),
        14 => make(a0, 0xB0, 15),
        16 => make(a0, 0xF4, 16),
        17 => make(a0, 0xE0, 17),
        18 => make(a0, 0xB4, 18),
        19 => make(a0, 0xB0, 19),
        20 => make(a0, 0xB4, 20),
        21 => make(a0, 0xBC, 21),
        22 => make(a0, 0xC8, 22),
        23 => make(a0, 0xE8, 23),
        24 => make(a0, 0xB8, 24),
        _ => 0,
    }
});
