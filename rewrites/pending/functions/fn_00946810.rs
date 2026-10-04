// original: 0x00946810 radio_emit_station_data
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

/// Emit one station's data words through a callback (original 0x00946810).
///
/// Calls the given callback with (pointer, length) pairs describing the
/// station table: three header words, then per-entry blocks selected by the
/// entry kind (resolved entries emit their item word, others a default
/// block), then six global words and a final table dump. Any callback answer
/// of zero aborts the walk and yields zero, otherwise one.
export!(cdecl, rw_00946810(emit: u32) -> u32 {
    const HEADER0: u32 = 0x01037606;
    const HEADER2: u32 = 0x01238955;
    const FINAL_TABLE: u32 = 0x011D7520;
    const COUNT_OFF: u32 = 0x011D74F2;
    const KIND_OFF: u32 = 0x011D74F1;
    const WORD0_OFF: u32 = 0x011D7640;
    const WORD1_OFF: u32 = 0x011D7644;
    const WORD2_OFF: u32 = 0x011D7648;
    unsafe {
        let emit: extern "cdecl" fn(u32, u32) -> u32 = core::mem::transmute(emit);
        // Frame mirror: slot i holds what the original keeps at the same
        // relative offset, so the callback's snapped windows match.
        let mut frame = [0u8; 0x60];
        // Head: three single words.
        if emit_word(&emit, relocated(HEADER0), 1) == 0 {
            return fail();
        }
        frame[0x12] = 0;
        if emit_word(&emit, frame.as_ptr().add(0x12) as u32, 1) == 0 {
            return fail();
        }
        if emit_word(&emit, relocated(HEADER2), 1) == 0 {
            return fail();
        }
        // Entry header block.
        frame[0x14] = 1;
        frame[0x15] = 2;
        frame[0x16] = 5;
        let mut index: u32 = 0;
        let count = global::<u8>(COUNT_OFF).read() as u32;
        while index < count {
            let kind = global::<u8>(KIND_OFF).read() as u32;
            if index >= kind {
                // Default block: three (word, blob) pairs plus trailer.
                for _ in 0..3 {
                    frame[0x11] = 0;
                    if emit_word(&emit, frame.as_ptr().add(0x11) as u32, 1) == 0 {
                        return fail();
                    }
                    if emit_word(&emit, frame.as_ptr().add(0x20) as u32, 0x28) == 0 {
                        return fail();
                    }
                }
                frame[0x13] = 0;
                if emit_word(&emit, frame.as_ptr().add(0x13) as u32, 1) == 0 {
                    return fail();
                }
                if emit_word(&emit, frame.as_ptr().add(0x48) as u32, 0x14) == 0 {
                    return fail();
                }
            } else {
                // Resolved block: look the entry up, then emit per item.
                let entry = callee_cdecl!(2, u32, index);
                frame[0x1C..0x20].copy_from_slice(&entry.to_le_bytes());
                for sub in 0..3u32 {
                    let tag = frame[(0x14 + sub) as usize] as u32;
                    let resolved = callee_thiscall!(3, u32, entry, tag);
                    if resolved != 0 {
                        if emit_word(&emit, resolved.wrapping_add(0x0F), 1) == 0 {
                            return fail();
                        }
                    } else {
                        frame[0x11] = 0;
                        if emit_word(&emit, frame.as_ptr().add(0x11) as u32, 1) == 0 {
                            return fail();
                        }
                    }
                    if emit_word(&emit, frame.as_ptr().add(0x20) as u32, 0x28) == 0 {
                        return fail();
                    }
                }
                if emit_word(&emit, entry.wrapping_add(0x1919), 1) == 0 {
                    return fail();
                }
                if emit_word(&emit, entry.wrapping_add(0x17D4), 0x14) == 0 {
                    return fail();
                }
            }
            index = index.wrapping_add(1);
            frame[0x18..0x1C].copy_from_slice(&index.to_le_bytes());
        }
        // Six global words.
        let words = [
            global::<u32>(WORD1_OFF).read(),
            global::<u32>(WORD0_OFF).read(),
            global::<u32>(WORD2_OFF).read(),
        ];
        for w in words {
            if emit_word(&emit, w.wrapping_add(0x0F), 1) == 0 {
                return fail();
            }
            if emit_word(&emit, w.wrapping_add(0x11), 0x28) == 0 {
                return fail();
            }
        }
        // Final table dump; result is nonzero iff the callback agrees.
        let table = global::<u8>(FINAL_TABLE) as *const u8 as u32;
        let ok = emit_word(&emit, table, 0x100) != 0;
        callee_cdecl!(4, u32,);
        if ok {
            1
        } else {
            0
        }
    }
});

/// One callback invocation; answers nonzero iff the callback agrees.
#[inline(always)]
unsafe fn emit_word(emit: &extern "cdecl" fn(u32, u32) -> u32, ptr: u32, len: u32) -> u32 {
    unsafe {
        if (emit(ptr, len) & 0xFF) == 0 {
            0
        } else {
            1
        }
    }
}

/// The abort path: run the epilogue cookie check and yield zero.
#[inline(always)]
unsafe fn fail() -> u32 {
    unsafe {
        callee_cdecl!(4, u32,);
        0
    }
}
