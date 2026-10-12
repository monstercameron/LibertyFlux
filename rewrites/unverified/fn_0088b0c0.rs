// original: 0x0088B0C0 audio_gather_match_words
/// Collects eligible node words into two caller-selected output arrays.
///
/// The primary output accepts nodes with status zero, subject to the group
/// threshold and remaining-capacity rules. Once the quota is exhausted, the
/// secondary output accepts nodes whose status is one. The count words live
/// at offsets `+8` and `+0xA` in the caller's count record.
const PRIMARY_WORDS_FILE_VA: u32 = 0x0115_A450;
const SECONDARY_WORDS_FILE_VA: u32 = 0x0115_A454;
const GROUP_HEAD_OFFSET: usize = 0x00;
const GROUP_THRESHOLD_LOW_OFFSET: usize = 0x08;
const GROUP_THRESHOLD_HIGH_OFFSET: usize = 0x0C;
const GROUP_LIMIT_OFFSET: usize = 0x14;
const GROUP_NEXT_OFFSET: usize = 0x18;
const NODE_WORD_OFFSET: usize = 0x00;
const NODE_KEY_OFFSET: usize = 0x04;
const NODE_STATUS_OFFSET: usize = 0x08;
const NODE_NEXT_OFFSET: usize = 0x0C;
const PRIMARY_COUNT_OFFSET: usize = 0x08;
const SECONDARY_COUNT_OFFSET: usize = 0x0A;

#[inline(always)]
unsafe fn read_u16(base: *const u8, offset: usize) -> u16 {
    // SAFETY: every caller passes a live node or count record supplied by the contract.
    unsafe { base.add(offset).cast::<u16>().read_unaligned() }
}

#[inline(always)]
unsafe fn read_u32(base: *const u8, offset: usize) -> u32 {
    // SAFETY: every caller passes a live node or group record supplied by the contract.
    unsafe { base.add(offset).cast::<u32>().read_unaligned() }
}

#[inline(always)]
unsafe fn write_u16(base: *mut u8, offset: usize, value: u16) {
    // SAFETY: every caller passes a writable count record supplied by the caller.
    unsafe { base.add(offset).cast::<u16>().write_unaligned(value) };
}

#[inline(always)]
unsafe fn append_word(words: *mut u16, index: u16, value: u16) {
    // SAFETY: the contract supplies an output array long enough for all selected nodes.
    unsafe { words.add(index as usize).write_unaligned(value) };
}

#[inline(always)]
unsafe fn append_primary(
    words: *mut u16,
    counts: *mut u8,
    count: &mut u16,
    node: *const u8,
) {
    let value = unsafe { read_u16(node, NODE_WORD_OFFSET) };
    unsafe { append_word(words, *count, value) };
    *count = count.wrapping_add(1);
    unsafe { write_u16(counts, PRIMARY_COUNT_OFFSET, *count) };
}

#[inline(always)]
unsafe fn append_secondary(
    words: *mut u16,
    counts: *mut u8,
    count: &mut u16,
    node: *const u8,
) {
    let value = unsafe { read_u16(node, NODE_WORD_OFFSET) };
    unsafe { append_word(words, *count, value) };
    *count = count.wrapping_add(1);
    unsafe { write_u16(counts, SECONDARY_COUNT_OFFSET, *count) };
}

#[inline(always)]
unsafe fn collect_group_primary(
    group: *const u8,
    capacity: &mut u32,
    quota: &mut u32,
    words: *mut u16,
    counts: *mut u8,
    count: &mut u16,
) {
    let limit = unsafe { read_u32(group, GROUP_LIMIT_OFFSET) };
    if *quota < limit {
        return;
    }

    let mut remaining_limit = limit;
    let mut node = unsafe { read_u32(group, GROUP_HEAD_OFFSET) } as *const u8;
    while !node.is_null() {
        let status = unsafe { read_u32(node, NODE_STATUS_OFFSET) };
        if status == 0 {
            if *capacity < remaining_limit {
                break;
            }
            unsafe { append_primary(words, counts, count, node) };
            *capacity = capacity.wrapping_sub(1);
            remaining_limit = remaining_limit.wrapping_sub(1);
        }
        node = unsafe { read_u32(node, NODE_NEXT_OFFSET) } as *const u8;
        *quota = quota.wrapping_sub(1);
    }
}

#[inline(always)]
unsafe fn collect_group_secondary(
    group: *const u8,
    words: *mut u16,
    counts: *mut u8,
    count: &mut u16,
) {
    let mut node = unsafe { read_u32(group, GROUP_HEAD_OFFSET) } as *const u8;
    while !node.is_null() {
        if unsafe { read_u32(node, NODE_STATUS_OFFSET) } == 1 {
            unsafe { append_secondary(words, counts, count, node) };
        }
        node = unsafe { read_u32(node, NODE_NEXT_OFFSET) } as *const u8;
    }
}

lf_checker_rt::export!(cdecl, fn_0088b0c0(
    first_group: *const u8,
    primary_head: *const u8,
    capacity: u32,
    quota: u32,
    counts: *mut u8,
) -> () {
    unsafe {
        let primary_words = lf_checker_rt::global::<u32>(PRIMARY_WORDS_FILE_VA)
            .read_unaligned() as *mut u16;
        let secondary_words = lf_checker_rt::global::<u32>(SECONDARY_WORDS_FILE_VA)
            .read_unaligned() as *mut u16;

        write_u16(counts, PRIMARY_COUNT_OFFSET, 0);
        let mut capacity = capacity;
        let mut quota = quota;
        let mut primary_count = 0u16;
        let mut secondary_count = read_u16(counts, SECONDARY_COUNT_OFFSET);
        let mut group = first_group;
        let mut primary = primary_head;

        while !primary.is_null() || !group.is_null() {
            if quota == 0 {
                if !primary.is_null() {
                    if read_u32(primary, NODE_STATUS_OFFSET) == 1 {
                        append_secondary(
                            secondary_words,
                            counts,
                            &mut secondary_count,
                            primary,
                        );
                    }
                    primary = read_u32(primary, NODE_NEXT_OFFSET) as *const u8;
                }

                if group.is_null() {
                    continue;
                }
                collect_group_secondary(
                    group,
                    secondary_words,
                    counts,
                    &mut secondary_count,
                );
                group = read_u32(group, GROUP_NEXT_OFFSET) as *const u8;
                continue;
            }

            if group.is_null() {
                if read_u32(primary, NODE_STATUS_OFFSET) == 0 && capacity != 0 {
                    append_primary(
                        primary_words,
                        counts,
                        &mut primary_count,
                        primary,
                    );
                    capacity = capacity.wrapping_sub(1);
                }
                primary = read_u32(primary, NODE_NEXT_OFFSET) as *const u8;
                quota = quota.wrapping_sub(1);
                continue;
            }

            if primary.is_null() {
                collect_group_primary(
                    group,
                    &mut capacity,
                    &mut quota,
                    primary_words,
                    counts,
                    &mut primary_count,
                );
                group = read_u32(group, GROUP_NEXT_OFFSET) as *const u8;
                primary = primary_head;
                continue;
            }

            let key = read_u32(primary, NODE_KEY_OFFSET);
            let threshold_low = read_u32(group, GROUP_THRESHOLD_LOW_OFFSET);
            let threshold_high = read_u32(group, GROUP_THRESHOLD_HIGH_OFFSET);
            let below_threshold = threshold_high != 0 || key < threshold_low;

            if below_threshold {
                if read_u32(primary, NODE_STATUS_OFFSET) == 0 && capacity != 0 {
                    append_primary(
                        primary_words,
                        counts,
                        &mut primary_count,
                        primary,
                    );
                    capacity = capacity.wrapping_sub(1);
                }
                primary = read_u32(primary, NODE_NEXT_OFFSET) as *const u8;
                quota = quota.wrapping_sub(1);
                continue;
            }

            collect_group_primary(
                group,
                &mut capacity,
                &mut quota,
                primary_words,
                counts,
                &mut primary_count,
            );
            group = read_u32(group, GROUP_NEXT_OFFSET) as *const u8;
            primary = primary_head;
        }
    }
});
