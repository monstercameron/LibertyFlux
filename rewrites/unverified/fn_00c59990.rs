// original: 0x00C59990 scanner_collect_candidates

/// Collect candidates for a scanner using one of two global candidate tables
/// or the batch-scan helper. The scanner owns its recent-candidate slots at
/// `+0x18`, a continuation slot at `+0x58`, and a count in the temporary
/// scan-context block at `+0x48`. The input point is three consecutive
/// single-precision values. The radius is squared in single precision before
/// the context initializer is called.
///
/// Modes 0 and 1 walk their selected table backward, skip entries marked by
/// bit `0x80`, and send each non-null candidate to the consideration helper.
/// Every other mode calls the batch helper with the consideration callback;
/// mode 2 supplies a limit of 16, while other values supply zero. A positive
/// signed cleanup count forwards that many non-null recent slots to the node
/// helper. Finally, a non-null first slot is copied to the continuation slot
/// and forwarded to the same helper.
///
/// The function uses the thiscall convention with four stack words: mode,
/// point pointer, radius bits, and excluded-item value. Its only known caller
/// ignores the return register.
lf_checker_rt::export!(thiscall, rw_scanner_collect_candidates(
    scanner: u32,
    mode: u32,
    point: u32,
    radius_bits: u32,
    excluded_item: u32,
) -> () {
    unsafe {
        const MODE_TABLE_A: u32 = 0;
        const MODE_TABLE_B: u32 = 1;
        const MODE_BATCH: u32 = 2;
        const TABLE_A_PTR_VA: u32 = 0x012e22a4;
        const TABLE_B_PTR_VA: u32 = 0x018b6f1c;
        const TABLE_DATA: u32 = 0;
        const TABLE_FLAGS: u32 = 4;
        const TABLE_COUNT: u32 = 8;
        const TABLE_STRIDE: u32 = 0x0c;
        const SCANNER_FIRST_SLOT: u32 = 0x18;
        const SCANNER_CONTINUATION: u32 = 0x58;
        const CONTEXT: u32 = 0x50;
        const FRAME_BYTES: usize = 0xc8;
        const BATCH_RESULT: u32 = 0x30;
        const BATCH_CONTEXT: u32 = 0x50;
        const BATCH_COUNT_FROM_RESULT: u32 = 0x68;
        const CALLBACK_CANDIDATE_CONSIDER: u32 = 0x00c59c90;
        const CALLEE_RESET: u32 = 1;
        const CALLEE_PLAYER_QUERY: u32 = 2;
        const CALLEE_INIT_CONTEXT: u32 = 3;
        const CALLEE_BATCH_SCAN: u32 = 4;
        const CALLEE_CONSIDER: u32 = 5;
        const CALLEE_LINK_NODE: u32 = 6;
        const CALLEE_COOKIE_CHECK: u32 = 7;
        const CONSIDERED_FLAG: u8 = 0x80;

        unsafe fn read_u32(address: u32) -> u32 {
            (address as *const u32).read_volatile()
        }

        unsafe fn read_u8(address: u32) -> u8 {
            (address as *const u8).read_volatile()
        }

        unsafe fn write_u32(address: u32, value: u32) {
            (address as *mut u32).write_volatile(value);
        }

        let mut frame_words = [0u32; FRAME_BYTES / 4];
        let frame_base = frame_words.as_mut_ptr() as u32;
        let context_ptr = frame_base.wrapping_add(CONTEXT);

        let point_x = read_u32(point);
        let point_y = read_u32(point.wrapping_add(4));
        let point_z = read_u32(point.wrapping_add(8));
        write_u32(frame_base.wrapping_add(0x20), point_x);
        write_u32(frame_base.wrapping_add(0x14), point_y);
        write_u32(frame_base.wrapping_add(0x18), point_z);

        let radius = f32::from_bits(radius_bits);
        let radius_squared = core::hint::black_box(core::hint::black_box(radius)
            * core::hint::black_box(radius));

        let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_RESET, u32, scanner);

        let player_answer = if excluded_item != 0 {
            lf_checker_rt::callee_cdecl!(CALLEE_PLAYER_QUERY, u32, 0)
        } else {
            0
        };
        let player_match = u32::from(player_answer == excluded_item);

        let _: u32 = lf_checker_rt::callee_thiscall!(
            CALLEE_INIT_CONTEXT,
            u32,
            context_ptr,
            scanner,
            point,
            excluded_item,
            player_match,
            radius_squared.to_bits(),
        );

        match mode {
            MODE_TABLE_A | MODE_TABLE_B => {
                let table_va = if mode == MODE_TABLE_A {
                    TABLE_A_PTR_VA
                } else {
                    TABLE_B_PTR_VA
                };
                let table = read_u32(lf_checker_rt::relocated(table_va));
                let data = read_u32(table.wrapping_add(TABLE_DATA));
                let flags = read_u32(table.wrapping_add(TABLE_FLAGS));
                let stride = read_u32(table.wrapping_add(TABLE_STRIDE));
                let mut remaining = read_u32(table.wrapping_add(TABLE_COUNT));
                while remaining != 0 {
                    remaining = remaining.wrapping_sub(1);
                    if read_u8(flags.wrapping_add(remaining)) & CONSIDERED_FLAG == 0 {
                        let candidate = data.wrapping_add(stride.wrapping_mul(remaining));
                        if candidate != 0 {
                            let _: u8 = lf_checker_rt::callee_cdecl!(
                                CALLEE_CONSIDER,
                                u8,
                                candidate,
                                context_ptr,
                            );
                        }
                    }
                }
            }
            _ => {
                write_u32(frame_base.wrapping_add(0x30), point_x);
                write_u32(frame_base.wrapping_add(0x34), point_y);
                write_u32(frame_base.wrapping_add(0x38), point_z);
                write_u32(frame_base.wrapping_add(0x40), radius_bits);
                let result_ptr = frame_base.wrapping_add(BATCH_RESULT);
                let callback_ptr = lf_checker_rt::relocated(CALLBACK_CANDIDATE_CONSIDER);
                let batch_context = frame_base.wrapping_add(BATCH_CONTEXT);
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    CALLEE_BATCH_SCAN,
                    u32,
                    result_ptr,
                    callback_ptr,
                    batch_context,
                    if mode == MODE_BATCH { 0x10 } else { 0 },
                    0x0c,
                );
            }
        }

        let cleanup_count = read_u32(
            frame_base
                .wrapping_add(BATCH_RESULT)
                .wrapping_add(BATCH_COUNT_FROM_RESULT),
        ) as i32;
        if cleanup_count > 0 {
            let mut index = 0u32;
            while index < cleanup_count as u32 {
                let slot = scanner
                    .wrapping_add(SCANNER_FIRST_SLOT)
                    .wrapping_add(index.wrapping_mul(4));
                let candidate = read_u32(slot);
                if candidate != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        CALLEE_LINK_NODE,
                        u32,
                        candidate,
                        slot,
                    );
                }
                index = index.wrapping_add(1);
            }
        }

        let first_candidate = read_u32(scanner.wrapping_add(SCANNER_FIRST_SLOT));
        if first_candidate != 0 {
            let continuation = scanner.wrapping_add(SCANNER_CONTINUATION);
            write_u32(continuation, first_candidate);
            let _: u32 = lf_checker_rt::callee_thiscall!(
                CALLEE_LINK_NODE,
                u32,
                first_candidate,
                continuation,
            );
        }

        // The original checks its stack cookie here. The contract deliberately
        // omits the stack-derived ECX value while preserving the call boundary.
        let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_COOKIE_CHECK, u32, 0);
    }
});
