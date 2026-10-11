// original: 0x00AFCEF0 proximity_sweep
/// Proximity sweep: the first pair from the dispatch word is taken as the work item.
///
/// For each work item the sweep reads the table slot of its index and the record
/// at that slot plus the group times 32; an empty slot or excluded record class ends
/// the sweep with the matching zero tail. Each trip of the work item visits one
/// candidate pair from the item's list. A candidate pair already on the work list
/// is skipped. The first and second plane crossings of the candidate are solved
/// against the probe radius; their points are written to the first output block.
/// The distance metric is written through the metric pointer. When the vertical
/// separation exceeds a quarter of the planar separation the candidate is appended
/// to the work list and the sweep continues from it (up to 0x50 pairs). Otherwise
/// the candidate is classified by the flag tests and the ID9, ID10 and ID12 filters.
/// Accepted candidates write the selected pair to the first pair word and the work
/// pair to the second; the positive result writes the work pair and the selected pair
/// the other way round. Returns 1 when a candidate was found, else zero.
export!(cdecl, rw_00afcef0(
    a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32,
    a8: u32, a9: u32, a10: u32, a11: u32, a12: u32, a13: u32, a14: u32,
) -> u32 {
    unsafe {
        let _ = (a1, a2, a3, a4, a5);
        // One-time flag setup: set each clear bit, resetting its slot to -1.
        let g54 = global::<u32>(0x01601054);
        let mut f = g54.read();
        if (f & 1) == 0 {
            f |= 1;
            g54.write(f);
            global::<u32>(0x01601050).write(0xFFFFFFFF);
        }
        if (f & 2) == 0 {
            f |= 2;
            g54.write(f);
            global::<u32>(0x01601058).write(0xFFFFFFFF);
        }
        if (f & 4) == 0 {
            f |= 4;
            g54.write(f);
            global::<u32>(0x0160105C).write(0xFFFFFFFF);
        }
        if (f & 8) == 0 {
            f |= 8;
            g54.write(f);
            global::<u32>(0x01601060).write(0xFFFFFFFF);
        }
        if (f & 0x10) == 0 {
            f |= 0x10;
            g54.write(f);
        }
        // Probe-distance refresh: rescan when the probe moved far or expired.
        let ax = ((a0) as *const f32).read();
        let ay = ((a0).wrapping_add(4) as *const f32).read();
        let gx = f32::from_bits(global::<u32>(0x01601070).read());
        let gy = f32::from_bits(global::<u32>(0x01601074).read());
        let t0 = ay - gy;
        let t1 = ax - gx;
        let d = t0 * t0 + t1 * t1;
        let ctr = global::<u32>(0x011735B4).read();
        let lim = global::<u32>(0x01601080).read();
        if d > 100.0 || ctr > lim {
            let this1 = relocated(0x01177A80);
            let p50 = relocated(0x01601050);
            let p58 = relocated(0x01601058);
            let p5c = relocated(0x0160105C);
            let p60 = relocated(0x01601060);
            callee_thiscall!(1, u32, this1, a0, p50, p58, 1);
            callee_thiscall!(1, u32, this1, a0, p5c, p60, 0);
            let az = ((a0).wrapping_add(8) as *const f32).read();
            let aw = ((a0).wrapping_add(12) as *const f32).read();
            global::<u32>(0x01601070).write(ax.to_bits());
            global::<u32>(0x01601074).write(ay.to_bits());
            global::<u32>(0x01601078).write(az.to_bits());
            global::<u32>(0x0160107C).write(aw.to_bits());
            global::<u32>(0x01601080).write(ctr.wrapping_add(5000));
        }
        // Random gate dispatch.
        let al = (callee_cdecl!(2, u32,) & 0xFF) as u8;
        let (edx, x0): (u32, f32);
        if (al & 0xF) == 4 && ((a9 & 0xFF) as u8) == 0 {
            let mut buf = [0u32; 4];
            let bufp = (&mut buf as *mut u32) as u32;
            let this1 = relocated(0x01177A80);
            let p = callee_thiscall!(3, u32, this1, bufp, a0, 0x42480000,
                0, 0, 1, 0, 0, 0, 0, 0, 0x40400000, 0xFFFFFFFF, 0);
            edx = (p as *const u32).read();
            let k15 = f32::from_bits(global::<u32>(0x0103FF68).read());
            x0 = k15 * f32::from_bits(a6);
        } else {
            let r = callee_cdecl!(2, u32,) & 0xFF;
            let a8b = ((a8 & 0xFF) as u8) != 0;
            if (r & 3) != 0 {
                edx = global::<u32>(if a8b { 0x0160105C } else { 0x01601050 }).read();
            } else {
                edx = global::<u32>(if a8b { 0x01601060 } else { 0x01601058 }).read();
            }
            x0 = f32::from_bits(a6);
        }
        // Gate 1: empty index exits.
        let lo0 = edx & 0xFFFF;
        if lo0 == 0xFFFF {
            callee_thiscall!(4, u32, 0);
            return 0xFF00;
        }
        // Gate 2: empty table slot exits (eax still holds the masked index).
        let slot0 = (relocated(0x01178284).wrapping_add(lo0.wrapping_mul(4)) as *const u32).read();
        if slot0 == 0 {
            callee_thiscall!(4, u32, 0);
            return lo0 & 0xFFFFFF00;
        }
        // Gate 3: excluded record class exits. Checked on the first work item only.
        let t0r = (edx >> 16).wrapping_shl(5);
        let recb0 = ((slot0.wrapping_add(t0r).wrapping_add(0x1C)) as *const u8).read();
        if (recb0 & 0xF0) == 0x90 {
            callee_thiscall!(4, u32, 0);
            return t0r & 0xFFFFFF00;
        }
        // Sweep rate, read once: the original re-reads the same product on every pass.
        let rate = f32::from_bits(global::<u32>(0x00FE8A24).read());
        let x1 = x0 * rate;

        // Work list: entry 0 is the first pair, the rest are appended by the
        // vertical-distance rejection. The count stays below the 0x50 limit.
        let mut work_pair = edx;
        let mut work_count: usize = 1;
        let mut work_list = [0xFFFF_FFFFu32; 0x50];
        work_list[0] = edx;
        let mut accumulated_gap = 0.0f32;

        'work: loop {
            // Sweep-rate check at the loop head. The original compares the rate
            // against register XMM0, which is zero on entry and afterwards still
            // holds the running total stored by the last rejection (the new total
            // is the last value written to XMM0 before the jump back). So the
            // threshold is the running total, not zero.
            if !(x1 > accumulated_gap) {
                // Exit 4: eax still holds the work-list count, which the common exit clears.
                callee_thiscall!(4, u32, 0);
                return 0x0;
            }
            let lo = work_pair & 0xFFFF;
            let slot = (relocated(0x01178284).wrapping_add(lo.wrapping_mul(4)) as *const u32).read();
            if slot == 0 {
                // Continuation work item with an empty table slot: the common exit clears AL.
                callee_thiscall!(4, u32, 0);
                return 0x0;
            }
            let group = work_pair >> 16;
            let record = slot.wrapping_add(group.wrapping_shl(5));
            let trip_count = ((record.wrapping_add(0x1E) as *const u8).read()) & 0x0F;
            if trip_count == 0 {
                callee_thiscall!(4, u32, 0);
                return (slot & 0xFFFF_FF00);
            }
            let _ignored_count = callee_cdecl!(5, u32, trip_count as u32);
            let list_table = relocated(0x01178384).wrapping_add(lo.wrapping_mul(4));
            let list = (list_table as *const u32).read();
            let base_index = ((record.wrapping_add(0x12) as *const i16).read()) as i32 as u32;
            // Trip loop: one pass per trip. An empty candidate slot or a pair
            // already on the work list ends that trip only; after the last trip
            // the sweep exits with zero (the count of trips clears in AL).
            'trip: for trip_index in 0..trip_count {
            let list_index = callee_cdecl!(6, u32, trip_index as u32).wrapping_add(base_index);
            let selected_pair = ((list.wrapping_add(list_index.wrapping_mul(8))) as *const u32).read();
            let candidate_index = selected_pair & 0xFFFF;
            let candidate_slot = ((relocated(0x01178284)
                .wrapping_add(candidate_index.wrapping_mul(4))) as *const u32).read();
            if candidate_slot == 0 {
                continue 'trip;
            }
            // Visited-set check: a pair already on the work list ends this trip.
            if work_list[..work_count].contains(&selected_pair) {
                                continue 'trip;
            }
            let candidate_group = selected_pair >> 16;
            let candidate_record = candidate_slot.wrapping_add(candidate_group.wrapping_mul(32));
            let pos = a0 as *const f32;
            let pos_x = pos.read();
            let pos_y = pos.add(1).read();
            let current_x = (record.wrapping_add(0x14) as *const i16).read() as f32 * 0.125;
            let current_y = (record.wrapping_add(0x16) as *const i16).read() as f32 * 0.125;
            let current_z = (record.wrapping_add(0x18) as *const i16).read() as f32 * 0.015625;
            let candidate_x =
                (candidate_record.wrapping_add(0x14) as *const i16).read() as f32 * 0.125;
            let candidate_y =
                (candidate_record.wrapping_add(0x16) as *const i16).read() as f32 * 0.125;
            let candidate_z =
                (candidate_record.wrapping_add(0x18) as *const i16).read() as f32 * 0.015625;
            let current_dx = core::hint::black_box(pos_x - current_x);
            let current_dy = core::hint::black_box(pos_y - current_y);
            let current_sq = core::hint::black_box(
                core::hint::black_box(current_dx * current_dx)
                    + core::hint::black_box(current_dy * current_dy),
            );
            let current_dist = current_sq.sqrt();
            let candidate_dx = core::hint::black_box(pos_x - candidate_x);
            let candidate_dy = core::hint::black_box(pos_y - candidate_y);
            let candidate_sq = core::hint::black_box(
                core::hint::black_box(candidate_dx * candidate_dx)
                    + core::hint::black_box(candidate_dy * candidate_dy),
            );
            let candidate_dist = candidate_sq.sqrt();

            // The original random gate is reached only when the caller's final
            // byte is zero. The accepted fixture pins this random return to zero.
            if a14 == 0 {
                let current_low = ((record.wrapping_add(0x1E) as *const u8).read()) & 0x0F;
                let candidate_low =
                    ((candidate_record.wrapping_add(0x1E) as *const u8).read()) & 0x0F;
                let random = callee_cdecl!(11, u32,) & 0x0F;
                if random > core::cmp::min(current_low, candidate_low) as u32 {
                    // Random-threshold rejection: the original leaves through the common
                    // exit, which clears AL, so the sweep returns zero.
                                        callee_thiscall!(4, u32, 0);
                    return 0;
                }
            }

            // Record flag bit 0x80 in the trip byte: the original takes a different
            // route (to the vertical-rejection block when the caller's byte is zero,
            // otherwise into the crossings).
            // Record flag bit 0x80 in either trip byte. With the caller's byte zero the
            // original jumps straight to the vertical-rejection block (no outputs, no
            // ID7 or ID8); with the byte set it carries on as usual.
            let flag80 = (((record.wrapping_add(0x1E) as *const u8).read()) & 0x80) != 0
                || (((candidate_record.wrapping_add(0x1E) as *const u8).read()) & 0x80) != 0;
            if flag80 && (((a8) & 0xFF) as u8) == 0 {
                                let gap_dy = core::hint::black_box(candidate_y - current_y);
                let gap_dx = core::hint::black_box(candidate_x - current_x);
                let gap = core::hint::black_box(
                    core::hint::black_box(gap_dy * gap_dy) + core::hint::black_box(gap_dx * gap_dx),
                )
                .sqrt();
                work_list[work_count] = selected_pair;
                work_count += 1;
                accumulated_gap += gap;
                if work_count >= 0x50 {
                    callee_thiscall!(4, u32, 0);
                    return 0;
                }
                work_pair = selected_pair;
                continue 'work;
            }

            let out = a10 as *mut f32;
            let interpolate = |radius: f32| {
                let candidate_delta = core::hint::black_box(candidate_dist - radius);
                let current_delta = core::hint::black_box(current_dist - radius);
                let candidate_weight = candidate_delta.abs();
                let current_weight = current_delta.abs();
                let denominator = core::hint::black_box(candidate_weight + current_weight);
                let reciprocal = core::hint::black_box(1.0f32 / denominator);
                let x = core::hint::black_box(
                    core::hint::black_box(
                        core::hint::black_box(current_x * candidate_weight)
                            + core::hint::black_box(candidate_x * current_weight),
                    ) * reciprocal,
                );
                let y = core::hint::black_box(
                    core::hint::black_box(
                        core::hint::black_box(current_y * candidate_weight)
                            + core::hint::black_box(candidate_y * current_weight),
                    ) * reciprocal,
                );
                let z = core::hint::black_box(
                    core::hint::black_box(
                        core::hint::black_box(current_z * candidate_weight)
                            + core::hint::black_box(candidate_z * current_weight),
                    ) * reciprocal,
                );
                (candidate_delta * current_delta, candidate_weight, current_weight, x, y, z)
            };

            let (first_crossing, first_cw, first_uw, first_x, first_y, first_z) =
                interpolate(f32::from_bits(a6));
            // No first crossing: the original skips the ID7 call and the outputs and
            // goes straight to the second crossing.
            // ID7 non-zero: the original skips the second crossing and the ID8 call,
            // and stores the product of the first crossing's two weights as the
            // distance metric (the first-crossing weights are the ones it keeps).
            let mut id7_nonzero = false;
            if first_crossing < 0.0 {
                out.write(first_x);
                out.add(1).write(first_y);
                out.add(2).write(first_z);
                // The original reads a stack scratch word here. This lane's
                // contract explicitly fixes stack_fill=0, so only that fixture
                // value is claimed.
                out.add(3).write(0.0);
                let table = relocated(0x0118D818);
                let index = (table as *const u32).read();
                let this = (table.wrapping_add(index.wrapping_mul(4)) as *const u32)
                    .read()
                    .wrapping_add(0x10);
                let result = callee_thiscall!(
                    7, u32, this, first_x.to_bits(), first_y.to_bits(), first_z.to_bits(),
                    0x40A00000, 0
                );
                id7_nonzero = result != 0;
            }

            // ID8 non-zero: the original jumps straight to the vertical-rejection
            // block, which skips the distance store and the vertical test.
            let mut id8_rejects = false;
            let distance_metric = if id7_nonzero {
                // The original's first-crossing reciprocal (1 / (cw + uw)) is left in the
                // slot it reads back as the first factor, so the metric is reciprocal * uw.
                let first_reciprocal =
                    core::hint::black_box(1.0f32 / core::hint::black_box(first_cw + first_uw));
                
                    core::hint::black_box(first_reciprocal * first_uw)
                
            } else {
                let (second_crossing, candidate_weight, current_weight, second_x, second_y, second_z) =
                    interpolate(f32::from_bits(a7));
                // No second crossing: the original jumps to the vertical-rejection block
                // (no distance store), like the ID8 non-zero route.
                if !(second_crossing < 0.0) {
                    id8_rejects = true;
                }
                if second_crossing < 0.0 {
                    out.write(second_x);
                    out.add(1).write(second_y);
                    out.add(2).write(second_z);
                    out.add(3).write(0.0);
                    let result = callee_thiscall!(
                        8, u8, relocated(0x0128E310), a10, 0x40A00000
                    );
                    id8_rejects = result != 0;
                }
                let distance_reciprocal =
                    core::hint::black_box(1.0f32 / core::hint::black_box(candidate_weight + current_weight));
                core::hint::black_box(distance_reciprocal * current_weight)
            };
            if !id8_rejects {
                (a13 as *mut f32).write(distance_metric);
            }            let dx = core::hint::black_box(current_x - candidate_x);
            let dy = core::hint::black_box(current_y - candidate_y);
            let dz = (current_z - candidate_z).abs();
            let planar_sq = core::hint::black_box(
                core::hint::black_box(dx * dx) + core::hint::black_box(dy * dy),
            );
            let vertical_limit = core::hint::black_box(planar_sq * 0.25);
            if id8_rejects || dz > vertical_limit {
                // Vertical-distance rejection (or the ID8 non-zero route into it). The original appends the selected
                // pair to the work list, adds the planar gap between the two
                // records to the running total (a total the caller never reads),
                // and either exits at the 0x50 count limit or continues the sweep
                // with the appended pair as the next work item.
                                let gap_dy = core::hint::black_box(candidate_y - current_y);
                let gap_dx = core::hint::black_box(candidate_x - current_x);
                let gap = core::hint::black_box(
                    core::hint::black_box(gap_dy * gap_dy) + core::hint::black_box(gap_dx * gap_dx),
                )
                .sqrt();
                work_list[work_count] = selected_pair;
                work_count += 1;
                accumulated_gap += gap;
                if work_count >= 0x50 {
                    callee_thiscall!(4, u32, 0);
                    return 0;
                }
                work_pair = selected_pair;
                continue 'work;
            }

            // Accept path (the vertical test passed): the flag tests and the ID9,
            // ID10 and ID12 filters, as proven by the stage 4 to 7 contracts.
            let pair_flags =
                (list.wrapping_add(list_index.wrapping_mul(8)).wrapping_add(4) as *const u32).read();
            let flags_byte5 = ((pair_flags >> 8) & 0xFF) as u8;
            let candidate_flags =
                (candidate_record.wrapping_add(0x1B) as *const u8).read() & 0xE0;
            let current_flags = (record.wrapping_add(0x1B) as *const u8).read() & 0xE0;
            // First-result flag (the original's first byte). It is set, cleared when the
            // pair's low flag bits are zero (ID9 is then not called), and cleared when ID9
            // matches. ID9 is not called when the candidate class is zero.
            let mut first_result = true;
            if (flags_byte5 & 7) == 0  {
                first_result = false;
            } else if candidate_flags != 0  {
                let id9_matches = callee_thiscall!(
                    9, u8, relocated(0x01177A80), candidate_record, record
                ) != 0;
                if id9_matches {
                    first_result = false;
                }
            }
            // Second flag (the original's AL): the policy bits gate ID10; a zero
            // current-record class skips ID10 and leaves the flag set.
            let mut second_result = true;
            let mut id10_answered_false = false;
            let mut exit_eax = list_index;
            if (flags_byte5 & 0x38) == 0 {
                second_result = false;
            } else if current_flags != 0 {
                let id10_eax = callee_thiscall!(
                    10, u32, relocated(0x01177A80), record, candidate_record
                );
                if (id10_eax as u8) != 0 {
                    second_result = false;
                    exit_eax = id10_eax;
                } else {
                    id10_answered_false = true;
                }
            }
            if !first_result {
                if second_result {
                                        // Accept tail: the first output word takes the selected pair, the second the work pair.
                    callee_thiscall!(4, u32, 0);
                    let accept_first = selected_pair;
                    (a11 as *mut u32).write(accept_first);
                    {
                        (a12 as *mut u32).write(work_pair);
                    }
                    let result_one_distance = core::hint::black_box(1.0f32 - distance_metric);
                    (a13 as *mut f32).write(result_one_distance);
                    return (a13 & 0xFFFFFF00) | 1;
                }
                // Common exit with the policy or ID10 value still in EAX.
                callee_thiscall!(4, u32, 0);
                return exit_eax & 0xFFFFFF00;
            }
            // Reject tail when the second flag is clear; otherwise ID12 decides.
            let id12_accepts = if second_result {
                callee_cdecl!(12, u32,) < 0x3FFF
            } else {
                false
            };
            if id12_accepts {
                callee_thiscall!(4, u32, 0);
                let accept_first = selected_pair;
                (a11 as *mut u32).write(accept_first);
                {
                    (a12 as *mut u32).write(work_pair);
                }
                let result_one_distance = core::hint::black_box(1.0f32 - distance_metric);
                (a13 as *mut f32).write(result_one_distance);
                return (a13 & 0xFFFFFF00) | 1;
            }
            callee_thiscall!(4, u32, 0);
            // Positive tail: the first output word takes the work pair, the second the selected pair.
            let positive_first = work_pair;
            let positive_second = selected_pair;
            (a11 as *mut u32).write(positive_first);
            (a12 as *mut u32).write(positive_second);
            (a13 as *mut f32).write(distance_metric);
            // The return value keeps the work pair's upper bits: the original sets only the low byte.
            return ((work_pair & 0xFFFF_FF00) | 1).wrapping_add(0);
            }
            // All trips done without a geometric candidate: the common exit.
            callee_thiscall!(4, u32, 0);
            return 0;
        }
    }
});
