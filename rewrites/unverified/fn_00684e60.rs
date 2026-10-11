// original: 0x00684E60 track_scan_emit

/// Rebuild the track result list from flagged fixed size source records.
///
/// The source header supplies a record pointer, a 16 bit record count, and two
/// low control bits. The first control bit selects the indexed event path; the
/// second selects record stored indices instead of sequential indices. Each
/// record is 0xE0 bytes and carries two independent event flags, an index word,
/// and a pair of tag fields.
///
/// The function first counts the flagged events and optional pair of explicit
/// tags. It resets the current list and allocates a replacement when requested,
/// otherwise it asks the list to grow and removes one existing slot. It then
/// emits kind zero and kind one events according to the source controls and
/// optional callback, followed by the explicit tag five and six callbacks.
/// The finalizer result is stored in the owner and returned. A cleanup callback
/// runs when rebuilding is disabled or the scanned indices are not sequential.
///
/// All record and list arithmetic wraps as 32 bit address arithmetic; event
/// counts and stored indices retain their 16 bit behavior.
lf_checker_rt::export!(thiscall, rw_00684E60(
    this: u32,
    source: u32,
    explicit_tags: u32,
    explicit_index: u32,
    rebuild: u32,
    callback: u32
) -> u32 {
    unsafe {
        const OWNER_RESULT: u32 = 0x08;
        const OWNER_ARRAY: u32 = 0x0c;
        const OWNER_COUNT: u32 = 0x10;
        const SOURCE_ARRAY: u32 = 0x00;
        const SOURCE_COUNT: u32 = 0x14;
        const SOURCE_CONTROL: u32 = 0x1c;
        const RECORD_STRIDE: u32 = 0xe0;
        const RECORD_INDEX: u32 = 0x16;
        const RECORD_KIND_ONE: u32 = 0x1a;
        const RECORD_KIND_ZERO: u32 = 0x1b;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        #[inline(always)]
        unsafe fn write_u16(address: u32, value: u16) {
            unsafe { (address as *mut u16).write_unaligned(value) }
        }

        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }

        #[inline(always)]
        unsafe fn record_at(source: u32, index: u32) -> u32 {
            unsafe {
                read_u32(source.wrapping_add(SOURCE_ARRAY))
                    .wrapping_add(index.wrapping_mul(RECORD_STRIDE))
            }
        }

        unsafe fn record_index(source: u32, index: u32, indexed: bool) -> u16 {
            unsafe {
                if indexed {
                    read_u16(record_at(source, index).wrapping_add(RECORD_INDEX))
                } else {
                    index as u16
                }
            }
        }

        unsafe fn callback_accepts(
            callback: u32,
            kind: u32,
            index: u32,
            frame_word: &mut u32,
        ) -> bool {
            unsafe {
                let table = read_u32(callback);
                let target = read_u32(table.wrapping_add(0x0c));
                let call: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(target as usize);
                (call(callback, kind, index, frame_word as *mut u32 as u32) as u8) != 0
            }
        }

        unsafe fn append_event(this: u32, kind: u32, tag: u8, index: u32) {
            unsafe {
                let table = read_u32(this);
                let target = read_u32(table.wrapping_add(4));
                let make: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(target as usize);
                let entry = make(this, kind);
                write_u8(entry.wrapping_add(5), tag);
                write_u16(entry.wrapping_add(6), index as u16);
                let slot: u32 =
                    lf_checker_rt::callee_thiscall!(5, u32, this.wrapping_add(OWNER_ARRAY), 1);
                write_u32(slot, entry);
                write_u32(this.wrapping_add(OWNER_RESULT), 0);
            }
        }

        unsafe fn scan_group(
            this: u32,
            source: u32,
            callback: u32,
            record_count: u32,
            flag_offset: Option<u32>,
            indexed: bool,
            kind: u32,
            tag: u8,
            matching: &mut bool,
            use_callback: bool,
        ) {
            unsafe {
                let mut frame_word = 0u32;
                let mut index = 0u32;
                while index < record_count {
                    let record = record_at(source, index);
                    if flag_offset.is_some()
                        && index == 0
                        && read_u32(source.wrapping_add(SOURCE_ARRAY)) == 0
                    {
                        index = index.wrapping_add(1);
                        continue;
                    }
                    if let Some(offset) = flag_offset {
                        if read_u8(record.wrapping_add(offset)) == 0 {
                            index = index.wrapping_add(1);
                            continue;
                        }
                    }
                    let stored_index = record_index(source, index, indexed);
                    *matching = *matching && stored_index == index as u16;
                    let accepted = !use_callback
                        || callback_accepts(callback, kind, stored_index as u32, &mut frame_word);
                    if accepted {
                        append_event(this, kind, tag, stored_index as u32);
                    }
                    index = index.wrapping_add(1);
                }
            }
        }

        let source_count = read_u16(source.wrapping_add(SOURCE_COUNT)) as u32;
        let control = read_u8(source.wrapping_add(SOURCE_CONTROL));
        let indexed = (control & 0x02) != 0;
        let mut event_count = 0u16;
        let mut count_index = 0u32;
        while count_index < source_count {
            let record = record_at(source, count_index);
            if record != 0 {
                if read_u8(record.wrapping_add(RECORD_KIND_ZERO)) != 0 {
                    event_count = event_count.wrapping_add(1);
                }
                if read_u8(record.wrapping_add(RECORD_KIND_ONE)) != 0 {
                    event_count = event_count.wrapping_add(1);
                }
            }
            count_index = count_index.wrapping_add(1);
        }
        if (explicit_tags as u8) != 0 {
            event_count = event_count.wrapping_add(2);
        }

        if (rebuild as u8) != 0 && read_u16(this.wrapping_add(OWNER_COUNT)) != 0 {
            write_u32(this.wrapping_add(4), 0);
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
            let old_array = read_u32(this.wrapping_add(OWNER_ARRAY));
            if old_array != 0 {
                let context = lf_checker_rt::tls_slot(0);
                let disposer = read_u32(context.wrapping_add(8));
                let table = read_u32(disposer);
                let target = read_u32(table.wrapping_add(0x0c));
                let dispose: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(target as usize);
                let _: u32 = dispose(disposer, old_array);
            }
            write_u32(this.wrapping_add(OWNER_ARRAY), 0);
            write_u32(this.wrapping_add(OWNER_COUNT), 0);
            if event_count != 0 {
                let replacement: u32 =
                    lf_checker_rt::callee_stdcall!(3, u32, event_count as u32);
                write_u32(this.wrapping_add(OWNER_ARRAY), replacement);
                write_u16(this.wrapping_add(OWNER_COUNT + 2), event_count);
            } else {
                write_u32(this.wrapping_add(OWNER_ARRAY), 0);
                write_u16(this.wrapping_add(OWNER_COUNT + 2), 0);
            }
        } else if event_count != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                4,
                u32,
                this.wrapping_add(OWNER_ARRAY),
                event_count as u32
            );
            let old_count = read_u16(this.wrapping_add(OWNER_COUNT));
            let new_count = old_count.wrapping_sub(1);
            write_u16(this.wrapping_add(OWNER_COUNT), new_count);
            let array = read_u32(this.wrapping_add(OWNER_ARRAY));
            let value = read_u32(array.wrapping_add((new_count as u32).wrapping_mul(4)));
            write_u32(
                array.wrapping_add((old_count as u32).wrapping_mul(4).wrapping_sub(4)),
                value,
            );
        }

        let mut matching = true;
        if (control & 0x01) != 0 {
            if callback != 0 {
                let mut frame_word = explicit_tags;
                if callback_accepts(callback, 0, 0, &mut frame_word) {
                    append_event(this, 0, 0, 0);
                }
                scan_group(
                    this,
                    source,
                    callback,
                    source_count,
                    None,
                    indexed,
                    1,
                    1,
                    &mut matching,
                    true,
                );
            } else {
                append_event(this, 0, 0, 0);
                scan_group(
                    this,
                    source,
                    callback,
                    source_count,
                    None,
                    indexed,
                    1,
                    1,
                    &mut matching,
                    false,
                );
            }
        } else if callback != 0 {
            scan_group(
                this,
                source,
                callback,
                source_count,
                Some(RECORD_KIND_ONE),
                indexed,
                0,
                0,
                &mut matching,
                true,
            );
            scan_group(
                this,
                source,
                callback,
                source_count,
                Some(RECORD_KIND_ZERO),
                indexed,
                1,
                1,
                &mut matching,
                true,
            );
        } else {
            scan_group(
                this,
                source,
                callback,
                source_count,
                Some(RECORD_KIND_ONE),
                indexed,
                0,
                0,
                &mut matching,
                false,
            );
            scan_group(
                this,
                source,
                callback,
                source_count,
                Some(RECORD_KIND_ZERO),
                indexed,
                1,
                1,
                &mut matching,
                false,
            );
        }

        if (explicit_tags as u8) != 0 {
            let mut frame_word = explicit_index;
            if callback == 0 || callback_accepts(callback, 5, explicit_index, &mut frame_word) {
                append_event(this, 0, 5, explicit_index);
            }
            if callback == 0 || callback_accepts(callback, 6, explicit_index, &mut frame_word) {
                append_event(this, 1, 6, explicit_index);
            }
        }

        if (rebuild as u8) == 0 || !matching {
            let _: u32 = lf_checker_rt::callee_thiscall!(8, u32, this, 0);
        }
        let answer: u32 = lf_checker_rt::callee_thiscall!(9, u32, this);
        write_u32(this.wrapping_add(OWNER_RESULT), answer);
        answer
    }
});
