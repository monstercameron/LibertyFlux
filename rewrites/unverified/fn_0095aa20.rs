// original: 0x0095AA20 scan_task_marker_text

/// Scan a text view until a terminal marker or a character helper accepts an
/// early stop. The input points to a 12-byte view header: a text pointer at
/// offset 0, a starting byte offset at 4, and a marker selector byte at 8.
/// For each nonzero byte, the marker predicate is called with the current
/// character address; a false predicate or a following dword at least as
/// large as the predicate result returns 1. Byte 0x1c skips 32 bytes, byte
/// 0x12 optionally creates and updates a marker object before skipping 16
/// bytes, and every other byte advances by the scripted character width.
/// At a terminator, selector table value 2 returns 1; other values restart
/// scanning at the translated pointer selected by `(selector + 1) % count`.
/// The original writes the selector into its incoming cdecl argument slot as
/// scratch storage; this rewrite keeps that intermediate in a local. The
/// function returns its Boolean result in AL.
lf_checker_rt::export!(cdecl, rw_0095aa20(input: u32) -> u8 {
    unsafe {
        const MARKER_TABLE_VA: u32 = 0x011F7018;
        const MARKER_COUNT_VA: u32 = 0x011F6FFD;
        const MARKER_POINTERS_VA: u32 = 0x011F7000;
        const IS_MARKER_CHAR: u32 = 1;
        const CHAR_WIDTH: u32 = 2;
        const SKIP_MARKER: u32 = 3;
        const MAKE_MARKER: u32 = 4;
        const SET_MARKER: u32 = 5;
        const RELEASE_MARKER: u32 = 6;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        let mut text_base = read_u32(input);
        let mut base_offset = read_u32(input + 4);
        let mut marker = read_u8(input + 8) as u32;

        loop {
            let current = text_base.wrapping_add(base_offset);
            let character = read_u8(current);
            if character == 0 {
                let marker_table = lf_checker_rt::global::<u8>(MARKER_TABLE_VA);
                if marker_table.add(marker as usize).read() == 2 {
                    return 1;
                }

                let divisor = lf_checker_rt::global::<u8>(MARKER_COUNT_VA).read() as i32;
                let next_marker = ((marker.wrapping_add(1) as i32) % divisor) as u32;
                let marker_pointers = lf_checker_rt::global::<u32>(MARKER_POINTERS_VA);
                let translated_base = marker_pointers
                    .add(next_marker as usize)
                    .read_unaligned();
                text_base = 0;
                base_offset = translated_base;
                marker = next_marker;
                continue;
            }

            let marker_char = lf_checker_rt::callee_thiscall!(IS_MARKER_CHAR, u32, current);
            if marker_char & 0xff == 0 || read_u32(current + 4) >= marker_char {
                return 1;
            }

            match character {
                0x1c => {
                    let _: u32 = lf_checker_rt::callee_thiscall!(SKIP_MARKER, u32, current);
                    text_base = text_base.wrapping_add(0x20);
                }
                0x12 => {
                    let object_key = read_u32(current + 8);
                    if object_key != u32::MAX {
                        let object = lf_checker_rt::callee_cdecl!(MAKE_MARKER, u32, 1u32, object_key);
                        if object != 0 {
                            let _: u32 = lf_checker_rt::callee_thiscall!(SET_MARKER, u32, current, object);
                            let _: u32 = lf_checker_rt::callee_thiscall!(
                                RELEASE_MARKER,
                                u32,
                                object.wrapping_add(0x10e0)
                            );
                        }
                    }
                    text_base = text_base.wrapping_add(0x10);
                }
                _ => {
                    let width = lf_checker_rt::callee_cdecl!(CHAR_WIDTH, u32, character as u32);
                    text_base = text_base.wrapping_add(width);
                }
            }
        }
    }
});
