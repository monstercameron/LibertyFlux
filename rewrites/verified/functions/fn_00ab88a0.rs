// original: 0x00AB88A0 load_player_diff_appearance
/// Load the appearance entries for one indexed record. The routine updates
/// three shared words, rotates the record's two byte selectors, builds and
/// scores bounded candidate pairs, then invokes the selected resource hooks.
/// The signed table index comes from the record at offset 0x2e; an inner
/// flag byte at offset 3 returns the table entry without side effects.
const TABLE_VA: u32 = 0x0129_5CD8;
const CONTEXT_VA: u32 = 0x0150_E0F4;
const GLOBAL_ARG4_VA: u32 = 0x0150_E0EC;
const GLOBAL_ARG5_VA: u32 = 0x0150_E0F0;
const GLOBAL_ZERO_VA: u32 = 0x0150_E128;
const RANDOM_SCALE_VA: u32 = 0x00FE_8680;
const ENTRY_INNER_OFFSET: u32 = 0x128;
const ENTRY_CONTEXT_OFFSET: u32 = 0x3C;
const RECORD_KEY_OFFSET: u32 = 0x5C;
const RECORD_TAG_OFFSET: u32 = 0x67;
const OUTER_LIMIT: u32 = 11;
const MAX_CANDIDATES: usize = 64;

#[inline(always)]
unsafe fn read_word(address: u32) -> u32 {
    (address as *const u32).read()
}

#[inline(always)]
unsafe fn read_byte(address: u32) -> u8 {
    (address as *const u8).read()
}

#[inline(always)]
unsafe fn write_byte(address: u32, value: u8) {
    (address as *mut u8).write(value);
}

#[derive(Clone, Copy)]
struct Candidate {
    selector: u32,
    group: u32,
    matches: u32,
}

impl Candidate {
    const EMPTY: Self = Self {
        selector: 0,
        group: 0,
        matches: 0,
    };
}

export!(cdecl, rw_00AB88A0(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        let record = a0;
        let signed_index = (record.wrapping_add(0x2E) as *const i16).read() as i32;
        let entry = *global::<u32>(TABLE_VA).offset(signed_index as isize);
        let inner = read_word(entry.wrapping_add(ENTRY_INNER_OFFSET));
        if read_byte(inner.wrapping_add(3)) != 0 {
            return entry;
        }

        let context_value = callee_thiscall!(
            1,
            u32,
            relocated(CONTEXT_VA),
            read_word(entry.wrapping_add(ENTRY_CONTEXT_OFFSET))
        );
        global::<u32>(GLOBAL_ARG4_VA).write(a4);
        global::<u32>(GLOBAL_ARG5_VA).write(a5);
        global::<u32>(GLOBAL_ZERO_VA).write(0);

        let mut output_count = 0u32;
        let mut output_records = [0u32; 5];
        let _id2_result = callee_cdecl!(
            2,
            u32,
            record,
            (&mut output_count as *mut u32) as u32,
            output_records.as_mut_ptr() as u32
        );

        let selector5_address = inner.wrapping_add(5);
        let selector6_address = inner.wrapping_add(6);
        let mut selector5 = read_byte(selector5_address).wrapping_add(1);
        let mut selector6 = read_byte(selector6_address);
        write_byte(selector5_address, selector5);

        let first_bound = callee_thiscall!(3, u32, inner, 0) & 0xFF;
        let page_bound = callee_thiscall!(
            4,
            u32,
            inner,
            0,
            u32::from(selector6)
        ) & 0xFF;
        if u32::from(selector5) >= page_bound {
            selector6 = selector6.wrapping_add(1);
            if u32::from(selector6) >= first_bound {
                selector6 = 0;
            }
            selector5 = 0;
            write_byte(selector6_address, selector6);
            write_byte(selector5_address, selector5);
        }

        let _ = callee_cdecl!(
            5,
            u32,
            record,
            a1,
            a2,
            a3,
            0,
            u32::from(selector6),
            u32::from(selector5)
        );
        let _ = callee_thiscall!(6, u32, context_value, 0, u32::from(selector6));
        let loop_context = callee_thiscall!(
            7,
            u32,
            inner,
            0,
            u32::from(selector5),
            u32::from(selector6)
        );

        let mut sentinel = u32::MAX;
        let mut outer = 1u32;
        while outer < OUTER_LIMIT {
            let middle_bound = callee_thiscall!(3, u32, inner, outer) & 0xFF;
            let mut candidates = [Candidate::EMPTY; MAX_CANDIDATES];
            let mut candidate_count = 0usize;
            let mut middle = 0u32;
            while middle < middle_bound {
                let middle_open = callee_thiscall!(8, u32, context_value, outer, middle) & 0xFF;
                if middle_open != 0 {
                    let inner_bound = callee_thiscall!(4, u32, inner, outer, middle) & 0xFF;
                    let mut selector = 0u32;
                    while selector < inner_bound {
                        let candidate = callee_thiscall!(
                            9,
                            u32,
                            inner,
                            outer,
                            middle,
                            selector
                        );
                        let accepted = callee_cdecl!(10, u32, candidate) & 0xFF;
                        if accepted != 0 && candidate_count < MAX_CANDIDATES {
                            candidates[candidate_count] = Candidate {
                                selector,
                                group: middle,
                                matches: 0,
                            };
                            candidate_count += 1;
                        }
                        selector = selector.wrapping_add(1);
                    }
                }
                middle = middle.wrapping_add(1);
            }

            let mut object_index = 0u32;
            while object_index < output_count {
                let object = output_records[object_index as usize];
                let key = u32::from(read_byte(
                    object.wrapping_add(outer).wrapping_add(RECORD_KEY_OFFSET)
                ));
                let tag = u32::from(read_byte(
                    object.wrapping_add(outer).wrapping_add(RECORD_TAG_OFFSET)
                ));
                let mut candidate_index = 0usize;
                while candidate_index < candidate_count {
                    let candidate = candidates[candidate_index];
                    if candidate.selector == key && candidate.group == tag {
                        candidates[candidate_index].matches =
                            candidates[candidate_index].matches.wrapping_add(1);
                    }
                    candidate_index += 1;
                }
                object_index = object_index.wrapping_add(1);
            }

            let mut least_matches = 0x270Fu32;
            let mut tied_candidates = 0u32;
            let mut candidate_index = 0usize;
            while candidate_index < candidate_count {
                let matches = candidates[candidate_index].matches;
                if matches < least_matches {
                    least_matches = matches;
                    tied_candidates = 0;
                }
                if matches == least_matches {
                    tied_candidates = tied_candidates.wrapping_add(1);
                }
                candidate_index += 1;
            }

            let random_word = callee_cdecl!(11, u32,) & 0xFFFF;
            let scale = global::<f32>(RANDOM_SCALE_VA).read();
            let random_f32 = random_word as f32;
            let scaled_random = core::hint::black_box(random_f32) * core::hint::black_box(scale);
            let tie_count_f32 = tied_candidates as f32;
            let weighted_random =
                core::hint::black_box(scaled_random) * core::hint::black_box(tie_count_f32);
            let mut selected_rank = weighted_random as i32;
            if selected_rank < 0 {
                selected_rank = i32::MIN;
            }

            let mut selected: Option<Candidate> = None;
            candidate_index = 0;
            while candidate_index < candidate_count {
                if candidates[candidate_index].matches == least_matches {
                    if selected_rank == 0 {
                        selected = Some(candidates[candidate_index]);
                        break;
                    }
                    selected_rank = selected_rank.wrapping_sub(1);
                }
                candidate_index += 1;
            }

            let mut current_selector = 0u32;
            let mut current_group = 0u32;
            if let Some(candidate) = selected {
                current_selector = candidate.selector;
                current_group = candidate.group;
                let _ = callee_thiscall!(14, u32, context_value, outer, current_group);
            }
            let id12_result = callee_thiscall!(12, u32, inner, outer, current_group) & 0xFF;
            if id12_result != 0 {
                let id13_result =
                    callee_thiscall!(13, u32, inner, outer, current_group) & 0xFF;
                if id13_result != 0 {
                    if sentinel == u32::MAX {
                        sentinel = current_selector;
                    } else {
                        current_selector = sentinel;
                    }
                }
            }

            let _ = callee_cdecl!(
                5,
                u32,
                record,
                a1,
                a2,
                a3,
                outer,
                current_group,
                current_selector
            );
            outer = outer.wrapping_add(1);
        }

        let mut run_final_helper = true;
        if a4 != 0 {
            let payload = u32::from(read_byte(record.wrapping_add(0x64)));
            let final_resource = callee_thiscall!(15, u32, inner, 8, payload);
            if read_word(final_resource.wrapping_add(8)) != 0 {
                run_final_helper = false;
            }
        }
        if run_final_helper {
            let _ = callee_cdecl!(5, u32, record, a1, a2, a3, 8, 0, 0);
        }

        callee_cdecl!(16, u32, record, a1, a2)
    }
});
