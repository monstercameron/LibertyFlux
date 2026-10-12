// original: 0x00BEB9B0 CVehInterp::~CVehInterp__deleting

/// Populate the two cached frame views held by an interpolation record.
///
/// `this` is the record and `frame` is a source frame descriptor. The record
/// keeps the source pointer, byte counts and pointers to sections within the
/// frame. Its first view is built before the readiness checks. When both
/// checks allow the update, a seed helper supplies a second frame pointer and
/// the second view is added. Version `0x00ff_ffff` clears the second view;
/// version zero leaves it untouched. The one stack argument is a frame
/// pointer, and the thiscall callee removes that word.
lf_checker_rt::export!(thiscall, rw_00beb9b0(this: u32, frame: u32) -> u32 {
    unsafe {
        const SOURCE_PTR: u32 = 0x08;
        const FIRST_BYTES: u32 = 0x10;
        const FIRST_COUNT: u32 = 0x18;
        const FIRST_BASE: u32 = 0x1c;
        const FIRST_ITEMS: u32 = 0x24;
        const FIRST_ITEM_PTR: u32 = 0x2c;
        const FIRST_FLAG: u32 = 0x34;
        const FIRST_FLAG_PTR: u32 = 0x38;
        const FIRST_EXTRA: u32 = 0x40;
        const FIRST_EXTRA_PTR: u32 = 0x48;
        const SECOND_BYTES: u32 = 0x14;
        const SECOND_BASE: u32 = 0x20;
        const SECOND_ITEMS: u32 = 0x28;
        const SECOND_ITEM_PTR: u32 = 0x30;
        const SECOND_FLAG: u32 = 0x35;
        const SECOND_FLAG_PTR: u32 = 0x3c;
        const SECOND_EXTRA: u32 = 0x44;
        const SECOND_EXTRA_PTR: u32 = 0x4c;
        const FRAME_LENGTH: u32 = 0x43;
        const FRAME_ITEMS: u32 = 0x42;
        const FRAME_FLAGS: u32 = 0x40;
        const FRAME_MODE: u32 = 0x18;
        const FRAME_EXTRA: u32 = 0x5c;
        const CLOCK_QWORD: u32 = 0x0120_ca30;
        const CLOCK_WORD: u32 = 0x0120_ca38;
        const NOT_READY: u32 = 0x00ff_ffff;

        #[inline(always)]
        unsafe fn rd8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }
        #[inline(always)]
        unsafe fn wr32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        fn char_size(code: u8) -> u32 {
            lf_checker_rt::callee_cdecl!(1, u32, code as u32)
        }
        #[inline(always)]
        fn frame_version(frame: u32) -> u32 {
            lf_checker_rt::callee_thiscall!(2, u32, frame)
        }
        #[inline(always)]
        fn section_pointer(base: u32, chars: u32, items: u32) -> u32 {
            base.wrapping_add(chars.wrapping_mul(24)).wrapping_add(items.wrapping_mul(8))
        }

        // The seed helper writes over these three words; initializing them
        // with the globals preserves the values visible in its call snapshot.
        let qword = lf_checker_rt::relocated(CLOCK_QWORD);
        let mut seed_words = [rd32(qword), rd32(qword.wrapping_add(4)), rd32(lf_checker_rt::relocated(CLOCK_WORD))];

        wr32(this.wrapping_add(SOURCE_PTR), frame);
        let source_start = char_size(rd8(frame));
        let first_bytes = rd32(this.wrapping_add(FIRST_BYTES)).wrapping_add(source_start);
        wr32(this.wrapping_add(FIRST_BYTES), first_bytes);

        let first_count = rd8(frame.wrapping_add(FRAME_LENGTH)) as u32;
        wr32(this.wrapping_add(FIRST_COUNT), first_count);
        let source = rd32(this.wrapping_add(SOURCE_PTR));
        let source_chars = char_size(rd8(source));
        let mut first_base = source.wrapping_add(source_chars);
        let mode = rd8(frame.wrapping_add(FRAME_MODE));
        if mode == 2 || mode == 3 {
            first_base = 0;
        }
        wr32(this.wrapping_add(FIRST_BASE), first_base);
        let first_bytes = rd32(this.wrapping_add(FIRST_BYTES))
            .wrapping_add(rd32(this.wrapping_add(FIRST_COUNT)).wrapping_mul(24));
        wr32(this.wrapping_add(FIRST_BYTES), first_bytes);

        let first_items = rd8(frame.wrapping_add(FRAME_ITEMS)) as u32;
        wr32(this.wrapping_add(FIRST_ITEMS), first_items);
        if first_items > 0 {
            let char_width = char_size(rd8(source));
            let pointer = section_pointer(
                source.wrapping_add(char_width),
                rd32(this.wrapping_add(FIRST_COUNT)),
                0,
            );
            wr32(this.wrapping_add(FIRST_ITEM_PTR), pointer);
            let amount = rd32(this.wrapping_add(FIRST_ITEMS)).wrapping_mul(8);
            let bytes = rd32(this.wrapping_add(FIRST_BYTES)).wrapping_add(amount);
            wr32(this.wrapping_add(FIRST_BYTES), bytes);
        } else {
            wr32(this.wrapping_add(FIRST_ITEM_PTR), 0);
        }

        let flags = rd8(frame.wrapping_add(FRAME_FLAGS));
        let has_flag_section = (flags >> 7) as u32;
        wr8(this.wrapping_add(FIRST_FLAG), has_flag_section as u8);
        if has_flag_section != 0 {
            let char_width = char_size(rd8(source));
            let pointer = section_pointer(
                source.wrapping_add(char_width),
                rd32(this.wrapping_add(FIRST_COUNT)),
                rd32(this.wrapping_add(FIRST_ITEMS)),
            );
            wr32(this.wrapping_add(FIRST_FLAG_PTR), pointer);
            let bytes = rd32(this.wrapping_add(FIRST_BYTES)).wrapping_add(24);
            wr32(this.wrapping_add(FIRST_BYTES), bytes);
        } else {
            wr32(this.wrapping_add(FIRST_FLAG_PTR), 0);
        }

        let extra_count = if (flags & 0x80) != 0 { rd8(frame.wrapping_add(FRAME_EXTRA)) as u32 } else { 0 };
        wr32(this.wrapping_add(FIRST_EXTRA), extra_count);
        if extra_count > 0 {
            let char_width = char_size(rd8(source));
            let pointer = section_pointer(
                source.wrapping_add(char_width),
                rd32(this.wrapping_add(FIRST_COUNT)).wrapping_add(1),
                rd32(this.wrapping_add(FIRST_ITEMS)),
            );
            wr32(this.wrapping_add(FIRST_EXTRA_PTR), pointer);
            let bytes = rd32(this.wrapping_add(FIRST_BYTES)).wrapping_add(extra_count.wrapping_mul(20));
            wr32(this.wrapping_add(FIRST_BYTES), bytes);
        } else {
            wr32(this.wrapping_add(FIRST_EXTRA_PTR), 0);
        }

        let first_version = frame_version(frame);
        if first_version == NOT_READY {
            wr32(this.wrapping_add(SECOND_BYTES), 0);
            wr32(this.wrapping_add(0x0c), 0);
            wr32(this.wrapping_add(SECOND_BASE), 0);
            wr32(this.wrapping_add(SECOND_ITEMS), 0);
            wr32(this.wrapping_add(SECOND_ITEM_PTR), 0);
            wr8(this.wrapping_add(SECOND_FLAG), 0);
            wr32(this.wrapping_add(SECOND_FLAG_PTR), 0);
            wr32(this.wrapping_add(SECOND_EXTRA), 0);
            wr32(this.wrapping_add(SECOND_EXTRA_PTR), 0);
            return NOT_READY;
        }
        if frame_version(frame) == 0 {
            return 0;
        }

        let seed_ptr = seed_words.as_mut_ptr() as u32;
        lf_checker_rt::callee_cdecl!(3, u32, seed_ptr, frame);
        let record_frame = rd32(seed_ptr).wrapping_add(rd32(seed_ptr.wrapping_add(4)));
        wr32(this.wrapping_add(0x0c), record_frame);

        let second_start = char_size(rd8(record_frame));
        let second_bytes = rd32(this.wrapping_add(SECOND_BYTES)).wrapping_add(second_start);
        wr32(this.wrapping_add(SECOND_BYTES), second_bytes);
        let record_chars = char_size(rd8(record_frame));
        let mut second_base = record_frame.wrapping_add(record_chars);
        let record_mode = rd8(record_frame.wrapping_add(FRAME_MODE));
        if record_mode == 2 || record_mode == 3 {
            second_base = 0;
        }
        wr32(this.wrapping_add(SECOND_BASE), second_base);
        let second_bytes = rd32(this.wrapping_add(SECOND_BYTES))
            .wrapping_add(rd32(this.wrapping_add(FIRST_COUNT)).wrapping_mul(24));
        wr32(this.wrapping_add(SECOND_BYTES), second_bytes);

        let second_items = rd8(record_frame.wrapping_add(FRAME_ITEMS)) as u32;
        wr32(this.wrapping_add(SECOND_ITEMS), second_items);
        if second_items > 0 {
            let char_width = char_size(rd8(record_frame));
            let pointer = section_pointer(
                record_frame.wrapping_add(char_width),
                rd32(this.wrapping_add(FIRST_COUNT)),
                0,
            );
            wr32(this.wrapping_add(SECOND_ITEM_PTR), pointer);
            let amount = rd32(this.wrapping_add(SECOND_ITEMS)).wrapping_mul(8);
            let bytes = rd32(this.wrapping_add(SECOND_BYTES)).wrapping_add(amount);
            wr32(this.wrapping_add(SECOND_BYTES), bytes);
        } else {
            wr32(this.wrapping_add(SECOND_ITEM_PTR), 0);
        }

        let record_flags = rd8(record_frame.wrapping_add(FRAME_FLAGS));
        let has_second_flag_section = (record_flags >> 7) as u32;
        wr8(this.wrapping_add(SECOND_FLAG), has_second_flag_section as u8);
        if has_second_flag_section != 0 {
            let char_width = char_size(rd8(record_frame));
            let pointer = section_pointer(
                record_frame.wrapping_add(char_width),
                rd32(this.wrapping_add(FIRST_COUNT)),
                rd32(this.wrapping_add(SECOND_ITEMS)),
            );
            wr32(this.wrapping_add(SECOND_FLAG_PTR), pointer);
            let bytes = rd32(this.wrapping_add(SECOND_BYTES)).wrapping_add(24);
            wr32(this.wrapping_add(SECOND_BYTES), bytes);
        } else {
            wr32(this.wrapping_add(SECOND_FLAG_PTR), 0);
        }

        let second_extra_count = if (record_flags & 0x80) != 0 { rd8(record_frame.wrapping_add(FRAME_EXTRA)) as u32 } else { 0 };
        wr32(this.wrapping_add(SECOND_EXTRA), second_extra_count);
        if second_extra_count > 0 {
            let char_width = char_size(rd8(record_frame));
            let pointer = section_pointer(
                record_frame.wrapping_add(char_width),
                rd32(this.wrapping_add(FIRST_COUNT)).wrapping_add(1),
                rd32(this.wrapping_add(SECOND_ITEMS)),
            );
            wr32(this.wrapping_add(SECOND_EXTRA_PTR), pointer);
            let bytes = rd32(this.wrapping_add(SECOND_BYTES)).wrapping_add(second_extra_count.wrapping_mul(20));
            wr32(this.wrapping_add(SECOND_BYTES), bytes);
        } else {
            wr32(this.wrapping_add(SECOND_EXTRA_PTR), 0);
        }
        rd32(this.wrapping_add(SECOND_EXTRA)).wrapping_mul(20)
    }
});
