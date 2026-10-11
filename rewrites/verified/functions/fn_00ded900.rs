// original: 0x00DED900 RP_CLIP
//! Builds the clip metadata document through its typed fields, stores each
//! constructed child in the owning document, and returns the cookie result.

use lf_checker_rt::{callee_cdecl, callee_thiscall, relocated};

const FRAME_ORIGIN_WORD: usize = 256;
const FRAME_WORD_COUNT: usize = 512;
const FLOAT_ADJUSTMENT_BITS: u32 = 0x41D00000;
const FIELD_KIND_FLOAT: u32 = 0x00000002;
const WORD_00000003: u32 = 0x00000003;
const FIELD_KIND_INTEGER: u32 = 0x00000004;
const WORD_00000006: u32 = 0x00000006;
const FIELD_KIND_FLAG: u32 = 0x00000008;
const WORD_0000000C: u32 = 0x0000000C;
const FIELD_KIND_VECTOR: u32 = 0x00000010;
const WORD_0000003E: u32 = 0x0000003E;
const WORD_00000100: u32 = 0x00000100;
const WORD_000001E4: u32 = 0x000001E4;
const OBJECT_BYTES_HEADER: u32 = 0x000001F0;
const OBJECT_BYTES_ENTRY: u32 = 0x000001F8;
const OBJECT_BYTES_CLIP_ITEM: u32 = 0x00000224;
const OBJECT_BYTES_DOCUMENT: u32 = 0x0000025C;
const OBJECT_BYTES_CLIP_SET: u32 = 0x00000328;
const IMAGE_REF_00F005D0: u32 = 0x00F005D0;
const IMAGE_REF_00F005E4: u32 = 0x00F005E4;
const IMAGE_REF_00F006F0: u32 = 0x00F006F0;
const IMAGE_REF_00F00704: u32 = 0x00F00704;
const IMAGE_REF_00F00710: u32 = 0x00F00710;
const IMAGE_REF_00F00718: u32 = 0x00F00718;
const IMAGE_REF_00F00728: u32 = 0x00F00728;
const IMAGE_REF_00F00734: u32 = 0x00F00734;
const IMAGE_REF_00F0073C: u32 = 0x00F0073C;
const IMAGE_REF_00F00748: u32 = 0x00F00748;
const IMAGE_REF_00F00754: u32 = 0x00F00754;
const IMAGE_REF_00F00760: u32 = 0x00F00760;
const IMAGE_REF_00F00770: u32 = 0x00F00770;
const IMAGE_REF_00F0077C: u32 = 0x00F0077C;
const IMAGE_REF_00F00788: u32 = 0x00F00788;
const IMAGE_REF_00F00790: u32 = 0x00F00790;
const IMAGE_REF_00F0079C: u32 = 0x00F0079C;
const IMAGE_REF_00F007B0: u32 = 0x00F007B0;
const IMAGE_REF_00F007C4: u32 = 0x00F007C4;
const IMAGE_REF_00F007D0: u32 = 0x00F007D0;
const IMAGE_REF_00F007E8: u32 = 0x00F007E8;
const IMAGE_REF_01168DD8: u32 = 0x01168DD8;
const IMAGE_REF_01981A4C: u32 = 0x01981A4C;
const F32_METADATA_SCALE_BITS: u32 = 0x3F50A3D7;
const F32_TWO_BITS: u32 = 0x40000000;
const F32_FIVE_BITS: u32 = 0x40A00000;
const F32_TWENTY_FIVE_BITS: u32 = 0x41C80000;
const WORD_41D00000: u32 = 0x41D00000;
const F32_EIGHTY_BITS: u32 = 0x42A00000;
const F32_142_BITS: u32 = 0x430E0000;
const F32_THREE_HUNDRED_TWENTY_BITS: u32 = 0x43A00000;
const F32_NEGATIVE_30_BITS: u32 = 0xC1F00000;
const F32_NEGATIVE_58_BITS: u32 = 0xC2680000;
const F32_NEGATIVE_86_BITS: u32 = 0xC2AC0000;
const DEFAULT_DOCUMENT_COLOR: u32 = 0xFF171717;
const DEFAULT_PROPERTY_COLOR: u32 = 0xFF6F6F6F;
const NO_FIELD_INDEX: u32 = 0xFFFFFFFF;

#[inline(always)]
fn frame_pointer(frame_slots: &mut [u32; FRAME_WORD_COUNT], frame_offset: u32) -> u32 {
    let word_index = FRAME_ORIGIN_WORD - (frame_offset as usize / 4);
    unsafe { frame_slots.as_mut_ptr().add(word_index) as usize as u32 }
}

#[inline(always)]
fn store_frame_word(frame_slots: &mut [u32; FRAME_WORD_COUNT], frame_offset: u32, value: u32) {
    let word_index = FRAME_ORIGIN_WORD - (frame_offset as usize / 4);
    frame_slots[word_index] = value;
}

#[inline(always)]
unsafe fn read_word(address: u32) -> u32 {
    (address as *const u32).read_unaligned()
}

#[inline(always)]
unsafe fn write_word(address: u32, value: u32) {
    (address as *mut u32).write_unaligned(value);
}

#[unsafe(no_mangle)]
pub unsafe extern "thiscall" fn rw_ded900(this_: u32) -> u32 {
    let entry_guard = callee_thiscall!(25, u32, this_);
    if (entry_guard as u8) != 0 {
        return callee_thiscall!(20, u32, 0u32);
    }

    let mut frame_slots = [0u32; FRAME_WORD_COUNT];
    store_frame_word(&mut frame_slots, 0x128, DEFAULT_DOCUMENT_COLOR);
    let mut call_results = [0u32; 248];
    let call_result_2 = callee_cdecl!(1, u32, OBJECT_BYTES_DOCUMENT);
    call_results[2] = call_result_2;

    let call_result_3 = callee_thiscall!(26, u32, this_);
    call_results[3] = call_result_3;

    let call_result_4 = callee_thiscall!(4, u32, call_results[2], relocated(IMAGE_REF_00F005D0), call_results[3]);
    call_results[4] = call_result_4;

    let call_result_5 = callee_thiscall!(5, u32, call_results[4], 0u32, 0u32, frame_pointer(&mut frame_slots, 0x128), NO_FIELD_INDEX);
    call_results[5] = call_result_5;

    store_frame_word(&mut frame_slots, 0x124, 0u32);
    store_frame_word(&mut frame_slots, 0x120, 0u32);
    let call_result_6 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x124), 0u32, 0u32);
    call_results[6] = call_result_6;

    let call_result_7 = callee_thiscall!(28, u32, call_results[4], FIELD_KIND_INTEGER);
    call_results[7] = call_result_7;

    let buffer_source_8 = call_results[6];
    let field_words_8 = unsafe { [
        read_word(buffer_source_8.wrapping_add(0)),
        read_word(buffer_source_8.wrapping_add(4)),
        read_word(buffer_source_8.wrapping_add(8)),
        read_word(buffer_source_8.wrapping_add(12)),
        read_word(buffer_source_8.wrapping_add(16)),
        read_word(buffer_source_8.wrapping_add(20)),
    ] };
    let field_tail_8 = unsafe { field_words_8.as_ptr().add(4) as usize as u32 };
    let call_result_8 = callee_thiscall!(2, u32, field_tail_8,
        field_words_8[0],
        field_words_8[1],
        field_words_8[2],
        field_words_8[3],
        field_words_8[4],
        field_words_8[5],
    );
    call_results[8] = call_result_8;

    store_frame_word(&mut frame_slots, 0x124, 0u32);
    store_frame_word(&mut frame_slots, 0x120, 0u32);
    let call_result_9 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x124), 0u32, 0u32);
    call_results[9] = call_result_9;

    let call_result_10 = callee_thiscall!(28, u32, call_results[4], FIELD_KIND_VECTOR);
    call_results[10] = call_result_10;

    let buffer_source_11 = call_results[9];
    let field_words_11 = unsafe { [
        read_word(buffer_source_11.wrapping_add(0)),
        read_word(buffer_source_11.wrapping_add(4)),
        read_word(buffer_source_11.wrapping_add(8)),
        read_word(buffer_source_11.wrapping_add(12)),
        read_word(buffer_source_11.wrapping_add(16)),
        read_word(buffer_source_11.wrapping_add(20)),
    ] };
    let field_tail_11 = unsafe { field_words_11.as_ptr().add(4) as usize as u32 };
    let call_result_11 = callee_thiscall!(2, u32, field_tail_11,
        field_words_11[0],
        field_words_11[1],
        field_words_11[2],
        field_words_11[3],
        field_words_11[4],
        field_words_11[5],
    );
    call_results[11] = call_result_11;

    store_frame_word(&mut frame_slots, 0x124, 0u32);
    store_frame_word(&mut frame_slots, 0x120, 0u32);
    let call_result_12 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x124), 0u32, 0u32);
    call_results[12] = call_result_12;

    let call_result_13 = callee_thiscall!(28, u32, call_results[4], FIELD_KIND_FLOAT);
    call_results[13] = call_result_13;

    let buffer_source_14 = call_results[12];
    let field_words_14 = unsafe { [
        read_word(buffer_source_14.wrapping_add(0)),
        read_word(buffer_source_14.wrapping_add(4)),
        read_word(buffer_source_14.wrapping_add(8)),
        read_word(buffer_source_14.wrapping_add(12)),
        read_word(buffer_source_14.wrapping_add(16)),
        read_word(buffer_source_14.wrapping_add(20)),
    ] };
    let field_tail_14 = unsafe { field_words_14.as_ptr().add(4) as usize as u32 };
    let call_result_14 = callee_thiscall!(2, u32, field_tail_14,
        field_words_14[0],
        field_words_14[1],
        field_words_14[2],
        field_words_14[3],
        field_words_14[4],
        field_words_14[5],
    );
    call_results[14] = call_result_14;

    store_frame_word(&mut frame_slots, 0x124, 0u32);
    store_frame_word(&mut frame_slots, 0x120, 0u32);
    let call_result_15 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x124), 0u32, 0u32);
    call_results[15] = call_result_15;

    let call_result_16 = callee_thiscall!(28, u32, call_results[4], FIELD_KIND_FLAG);
    call_results[16] = call_result_16;

    let buffer_source_17 = call_results[15];
    let field_words_17 = unsafe { [
        read_word(buffer_source_17.wrapping_add(0)),
        read_word(buffer_source_17.wrapping_add(4)),
        read_word(buffer_source_17.wrapping_add(8)),
        read_word(buffer_source_17.wrapping_add(12)),
        read_word(buffer_source_17.wrapping_add(16)),
        read_word(buffer_source_17.wrapping_add(20)),
    ] };
    let field_tail_17 = unsafe { field_words_17.as_ptr().add(4) as usize as u32 };
    let call_result_17 = callee_thiscall!(2, u32, field_tail_17,
        field_words_17[0],
        field_words_17[1],
        field_words_17[2],
        field_words_17[3],
        field_words_17[4],
        field_words_17[5],
    );
    call_results[17] = call_result_17;

    let call_result_18 = callee_cdecl!(1, u32, OBJECT_BYTES_HEADER);
    call_results[18] = call_result_18;

    let call_result_19 = callee_thiscall!(26, u32, this_);
    call_results[19] = call_result_19;

    let call_result_20 = callee_thiscall!(10, u32, call_results[18], relocated(IMAGE_REF_00F005E4), call_results[19]);
    call_results[20] = call_result_20;
    unsafe { write_word(this_.wrapping_add(0x1E4), call_results[20]); }

    let call_result_21 = callee_thiscall!(13, u32, call_results[20]);
    call_results[21] = call_result_21;

    store_frame_word(&mut frame_slots, 0x124, 0u32);
    store_frame_word(&mut frame_slots, 0x120, 0u32);
    let call_result_22 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x124), 0u32, F32_FIVE_BITS);
    call_results[22] = call_result_22;

    let call_result_23 = callee_thiscall!(29, u32, unsafe { read_word(this_.wrapping_add(0x1E4)) }, FIELD_KIND_INTEGER, relocated(IMAGE_REF_00F006F0), FIELD_KIND_INTEGER);
    call_results[23] = call_result_23;

    let buffer_source_24 = call_results[22];
    let field_words_24 = unsafe { [
        read_word(buffer_source_24.wrapping_add(0)),
        read_word(buffer_source_24.wrapping_add(4)),
        read_word(buffer_source_24.wrapping_add(8)),
        read_word(buffer_source_24.wrapping_add(12)),
        read_word(buffer_source_24.wrapping_add(16)),
        read_word(buffer_source_24.wrapping_add(20)),
    ] };
    let field_tail_24 = unsafe { field_words_24.as_ptr().add(4) as usize as u32 };
    let call_result_24 = callee_thiscall!(2, u32, field_tail_24,
        field_words_24[0],
        field_words_24[1],
        field_words_24[2],
        field_words_24[3],
        field_words_24[4],
        field_words_24[5],
    );
    call_results[24] = call_result_24;

    store_frame_word(&mut frame_slots, 0x124, 0u32);
    store_frame_word(&mut frame_slots, 0x120, 0u32);
    let call_result_25 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x124), 0u32, 0u32);
    call_results[25] = call_result_25;

    let call_result_26 = callee_thiscall!(28, u32, unsafe { read_word(this_.wrapping_add(0x1E4)) }, FIELD_KIND_FLOAT);
    call_results[26] = call_result_26;

    let buffer_source_27 = call_results[25];
    let field_words_27 = unsafe { [
        read_word(buffer_source_27.wrapping_add(0)),
        read_word(buffer_source_27.wrapping_add(4)),
        read_word(buffer_source_27.wrapping_add(8)),
        read_word(buffer_source_27.wrapping_add(12)),
        read_word(buffer_source_27.wrapping_add(16)),
        read_word(buffer_source_27.wrapping_add(20)),
    ] };
    let field_tail_27 = unsafe { field_words_27.as_ptr().add(4) as usize as u32 };
    let call_result_27 = callee_thiscall!(2, u32, field_tail_27,
        field_words_27[0],
        field_words_27[1],
        field_words_27[2],
        field_words_27[3],
        field_words_27[4],
        field_words_27[5],
    );
    call_results[27] = call_result_27;

    store_frame_word(&mut frame_slots, 0x124, 0u32);
    store_frame_word(&mut frame_slots, 0x120, 0u32);
    let call_result_28 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x124), 0u32, 0u32);
    call_results[28] = call_result_28;

    let call_result_29 = callee_thiscall!(28, u32, unsafe { read_word(this_.wrapping_add(0x1E4)) }, FIELD_KIND_FLAG);
    call_results[29] = call_result_29;

    let buffer_source_30 = call_results[28];
    let field_words_30 = unsafe { [
        read_word(buffer_source_30.wrapping_add(0)),
        read_word(buffer_source_30.wrapping_add(4)),
        read_word(buffer_source_30.wrapping_add(8)),
        read_word(buffer_source_30.wrapping_add(12)),
        read_word(buffer_source_30.wrapping_add(16)),
        read_word(buffer_source_30.wrapping_add(20)),
    ] };
    let field_tail_30 = unsafe { field_words_30.as_ptr().add(4) as usize as u32 };
    let call_result_30 = callee_thiscall!(2, u32, field_tail_30,
        field_words_30[0],
        field_words_30[1],
        field_words_30[2],
        field_words_30[3],
        field_words_30[4],
        field_words_30[5],
    );
    call_results[30] = call_result_30;

    let call_result_31 = callee_thiscall!(27, u32, this_);
    call_results[31] = call_result_31;

    let call_result_32 = callee_thiscall!(31, u32, unsafe { read_word(this_.wrapping_add(0x1E4)) }, F32_METADATA_SCALE_BITS, call_results[31]);
    call_results[32] = call_result_32;

    let call_result_33 = callee_thiscall!(27, u32, this_);
    call_results[33] = call_result_33;

    let call_result_34 = callee_thiscall!(32, u32, unsafe { read_word(this_.wrapping_add(0x1E4)) }, call_results[33]);
    call_results[34] = call_result_34;

    let call_result_35 = callee_cdecl!(1, u32, OBJECT_BYTES_ENTRY);
    call_results[35] = call_result_35;

    let call_result_36 = callee_thiscall!(26, u32, unsafe { read_word(this_.wrapping_add(0x1E4)) });
    call_results[36] = call_result_36;

    let call_result_37 = callee_thiscall!(9, u32, call_results[35], relocated(IMAGE_REF_00F00704), call_results[36]);
    call_results[37] = call_result_37;

    store_frame_word(&mut frame_slots, 0x108, 0u32);
    store_frame_word(&mut frame_slots, 0x104, 0u32);
    store_frame_word(&mut frame_slots, 0x100, 0u32);
    store_frame_word(&mut frame_slots, 0x0FC, 0u32);
    store_frame_word(&mut frame_slots, 0x0F8, 0u32);
    store_frame_word(&mut frame_slots, 0x0F4, 0u32);
    store_frame_word(&mut frame_slots, 0x0F0, 0u32);
    store_frame_word(&mut frame_slots, 0x0EC, 0u32);
    let call_result_38 = callee_cdecl!(24, u32, frame_pointer(&mut frame_slots, 0x108), FIELD_KIND_FLOAT);
    call_results[38] = call_result_38;

    store_frame_word(&mut frame_slots, 0x10C, 0u32);
    store_frame_word(&mut frame_slots, 0x108, 0u32);
    store_frame_word(&mut frame_slots, 0x104, 0u32);
    store_frame_word(&mut frame_slots, 0x100, 0u32);
    store_frame_word(&mut frame_slots, 0x0FC, 0u32);
    store_frame_word(&mut frame_slots, 0x0F8, 0u32);
    store_frame_word(&mut frame_slots, 0x0F4, 0u32);
    store_frame_word(&mut frame_slots, 0x0F0, 0u32);
    let call_result_39 = callee_cdecl!(24, u32, frame_pointer(&mut frame_slots, 0x10C), WORD_0000003E);
    call_results[39] = call_result_39;

    store_frame_word(&mut frame_slots, 0x128, 0u32);
    store_frame_word(&mut frame_slots, 0x124, 0u32);
    store_frame_word(&mut frame_slots, 0x120, 0u32);
    store_frame_word(&mut frame_slots, 0x11C, 0u32);
    store_frame_word(&mut frame_slots, 0x118, 0u32);
    store_frame_word(&mut frame_slots, 0x114, 0u32);
    store_frame_word(&mut frame_slots, 0x110, 0u32);
    store_frame_word(&mut frame_slots, 0x10C, 0u32);
    store_frame_word(&mut frame_slots, 0x128, DEFAULT_PROPERTY_COLOR);
    let call_result_40 = callee_thiscall!(12, u32, call_results[37], 0u32, relocated(IMAGE_REF_00F00710), frame_pointer(&mut frame_slots, 0x128), call_results[39], call_results[38]);
    call_results[40] = call_result_40;

    store_frame_word(&mut frame_slots, 0x124, 0u32);
    store_frame_word(&mut frame_slots, 0x120, 0u32);
    let call_result_41 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x124), 0u32, 0u32);
    call_results[41] = call_result_41;

    let call_result_42 = callee_thiscall!(28, u32, call_results[37], WORD_00000006);
    call_results[42] = call_result_42;

    let buffer_source_43 = call_results[41];
    let field_words_43 = unsafe { [
        read_word(buffer_source_43.wrapping_add(0)),
        read_word(buffer_source_43.wrapping_add(4)),
        read_word(buffer_source_43.wrapping_add(8)),
        read_word(buffer_source_43.wrapping_add(12)),
        read_word(buffer_source_43.wrapping_add(16)),
        read_word(buffer_source_43.wrapping_add(20)),
    ] };
    let field_tail_43 = unsafe { field_words_43.as_ptr().add(4) as usize as u32 };
    let call_result_43 = callee_thiscall!(2, u32, field_tail_43,
        field_words_43[0],
        field_words_43[1],
        field_words_43[2],
        field_words_43[3],
        field_words_43[4],
        field_words_43[5],
    );
    call_results[43] = call_result_43;

    store_frame_word(&mut frame_slots, 0x124, 0u32);
    store_frame_word(&mut frame_slots, 0x120, 0u32);
    let call_result_44 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x124), 0u32, 0u32);
    call_results[44] = call_result_44;

    let call_result_45 = callee_thiscall!(28, u32, call_results[37], WORD_0000000C);
    call_results[45] = call_result_45;

    let buffer_source_46 = call_results[44];
    let field_words_46 = unsafe { [
        read_word(buffer_source_46.wrapping_add(0)),
        read_word(buffer_source_46.wrapping_add(4)),
        read_word(buffer_source_46.wrapping_add(8)),
        read_word(buffer_source_46.wrapping_add(12)),
        read_word(buffer_source_46.wrapping_add(16)),
        read_word(buffer_source_46.wrapping_add(20)),
    ] };
    let field_tail_46 = unsafe { field_words_46.as_ptr().add(4) as usize as u32 };
    let call_result_46 = callee_thiscall!(2, u32, field_tail_46,
        field_words_46[0],
        field_words_46[1],
        field_words_46[2],
        field_words_46[3],
        field_words_46[4],
        field_words_46[5],
    );
    call_results[46] = call_result_46;

    let call_result_47 = callee_thiscall!(35, u32, call_results[37], WORD_41D00000);
    call_results[47] = call_result_47;

    let call_result_48 = callee_thiscall!(27, u32, this_);
    call_results[48] = call_result_48;

    let call_result_49 = callee_thiscall!(32, u32, call_results[37], call_results[48]);
    call_results[49] = call_result_49;

    let call_result_50 = callee_thiscall!(27, u32, this_);
    call_results[50] = call_result_50;

    let call_result_51 = callee_thiscall!(33, u32, call_results[37], call_results[50]);
    call_results[51] = call_result_51;

    let call_result_52 = callee_thiscall!(36, u32, call_results[37], 1u32);
    call_results[52] = call_result_52;

    let call_result_53 = callee_cdecl!(21, u32, frame_pointer(&mut frame_slots, 0x104), 0u32, WORD_00000100);
    call_results[53] = call_result_53;

    let call_result_54 = callee_cdecl!(23, u32, relocated(IMAGE_REF_00F00718), 0u32);
    call_results[54] = call_result_54;

    let call_result_55 = callee_cdecl!(22, u32, frame_pointer(&mut frame_slots, 0x104), relocated(IMAGE_REF_01168DD8), 0u32);
    call_results[55] = call_result_55;

    let call_result_56 = callee_cdecl!(1, u32, OBJECT_BYTES_CLIP_SET);
    call_results[56] = call_result_56;

    let call_result_57 = callee_thiscall!(26, u32, unsafe { read_word(this_.wrapping_add(0x1E4)) });
    call_results[57] = call_result_57;

    let call_result_58 = callee_thiscall!(18, u32, call_results[56], relocated(IMAGE_REF_00F00728), call_results[57]);
    call_results[58] = call_result_58;
    unsafe { write_word(this_.wrapping_add(0x1F4), call_results[58]); }

    let call_result_59 = callee_thiscall!(19, u32, call_results[58], frame_pointer(&mut frame_slots, 0x104), relocated(IMAGE_REF_00F00734), FIELD_KIND_FLOAT, F32_142_BITS, F32_EIGHTY_BITS, F32_FIVE_BITS, F32_FIVE_BITS);
    call_results[59] = call_result_59;

    store_frame_word(&mut frame_slots, 0x124, 0u32);
    store_frame_word(&mut frame_slots, 0x120, 0u32);
    let call_result_60 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x124), 0u32, 0u32);
    call_results[60] = call_result_60;

    let call_result_61 = callee_thiscall!(27, u32, call_results[37]);
    call_results[61] = call_result_61;

    let call_result_62 = callee_thiscall!(30, u32, unsafe { read_word(this_.wrapping_add(0x1F4)) }, FIELD_KIND_INTEGER, call_results[61], FIELD_KIND_VECTOR);
    call_results[62] = call_result_62;

    let buffer_source_63 = call_results[60];
    let field_words_63 = unsafe { [
        read_word(buffer_source_63.wrapping_add(0)),
        read_word(buffer_source_63.wrapping_add(4)),
        read_word(buffer_source_63.wrapping_add(8)),
        read_word(buffer_source_63.wrapping_add(12)),
        read_word(buffer_source_63.wrapping_add(16)),
        read_word(buffer_source_63.wrapping_add(20)),
    ] };
    let field_tail_63 = unsafe { field_words_63.as_ptr().add(4) as usize as u32 };
    let call_result_63 = callee_thiscall!(2, u32, field_tail_63,
        field_words_63[0],
        field_words_63[1],
        field_words_63[2],
        field_words_63[3],
        field_words_63[4],
        field_words_63[5],
    );
    call_results[63] = call_result_63;

    store_frame_word(&mut frame_slots, 0x128, 0u32);
    store_frame_word(&mut frame_slots, 0x124, 0u32);
    let call_result_64 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x128), 0u32, F32_NEGATIVE_86_BITS);
    call_results[64] = call_result_64;

    let call_result_65 = callee_thiscall!(27, u32, this_);
    call_results[65] = call_result_65;

    let call_result_66 = callee_thiscall!(30, u32, unsafe { read_word(this_.wrapping_add(0x1F4)) }, FIELD_KIND_VECTOR, call_results[65], FIELD_KIND_VECTOR);
    call_results[66] = call_result_66;

    let buffer_source_67 = call_results[64];
    let field_words_67 = unsafe { [
        read_word(buffer_source_67.wrapping_add(0)),
        read_word(buffer_source_67.wrapping_add(4)),
        read_word(buffer_source_67.wrapping_add(8)),
        read_word(buffer_source_67.wrapping_add(12)),
        read_word(buffer_source_67.wrapping_add(16)),
        read_word(buffer_source_67.wrapping_add(20)),
    ] };
    let field_tail_67 = unsafe { field_words_67.as_ptr().add(4) as usize as u32 };
    let call_result_67 = callee_thiscall!(2, u32, field_tail_67,
        field_words_67[0],
        field_words_67[1],
        field_words_67[2],
        field_words_67[3],
        field_words_67[4],
        field_words_67[5],
    );
    call_results[67] = call_result_67;

    store_frame_word(&mut frame_slots, 0x12C, 0u32);
    store_frame_word(&mut frame_slots, 0x128, 0u32);
    let call_result_68 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x12C), 0u32, 0u32);
    call_results[68] = call_result_68;

    let call_result_69 = callee_thiscall!(28, u32, unsafe { read_word(this_.wrapping_add(0x1F4)) }, FIELD_KIND_FLOAT);
    call_results[69] = call_result_69;

    let buffer_source_70 = call_results[68];
    let field_words_70 = unsafe { [
        read_word(buffer_source_70.wrapping_add(0)),
        read_word(buffer_source_70.wrapping_add(4)),
        read_word(buffer_source_70.wrapping_add(8)),
        read_word(buffer_source_70.wrapping_add(12)),
        read_word(buffer_source_70.wrapping_add(16)),
        read_word(buffer_source_70.wrapping_add(20)),
    ] };
    let field_tail_70 = unsafe { field_words_70.as_ptr().add(4) as usize as u32 };
    let call_result_70 = callee_thiscall!(2, u32, field_tail_70,
        field_words_70[0],
        field_words_70[1],
        field_words_70[2],
        field_words_70[3],
        field_words_70[4],
        field_words_70[5],
    );
    call_results[70] = call_result_70;

    store_frame_word(&mut frame_slots, 0x12C, 0u32);
    store_frame_word(&mut frame_slots, 0x128, 0u32);
    let call_result_71 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x12C), 0u32, 0u32);
    call_results[71] = call_result_71;

    let call_result_72 = callee_thiscall!(28, u32, unsafe { read_word(this_.wrapping_add(0x1F4)) }, FIELD_KIND_FLAG);
    call_results[72] = call_result_72;

    let buffer_source_73 = call_results[71];
    let field_words_73 = unsafe { [
        read_word(buffer_source_73.wrapping_add(0)),
        read_word(buffer_source_73.wrapping_add(4)),
        read_word(buffer_source_73.wrapping_add(8)),
        read_word(buffer_source_73.wrapping_add(12)),
        read_word(buffer_source_73.wrapping_add(16)),
        read_word(buffer_source_73.wrapping_add(20)),
    ] };
    let field_tail_73 = unsafe { field_words_73.as_ptr().add(4) as usize as u32 };
    let call_result_73 = callee_thiscall!(2, u32, field_tail_73,
        field_words_73[0],
        field_words_73[1],
        field_words_73[2],
        field_words_73[3],
        field_words_73[4],
        field_words_73[5],
    );
    call_results[73] = call_result_73;

    let call_result_74 = callee_thiscall!(37, u32, unsafe { read_word(this_.wrapping_add(0x1F4)) }, 1u32);
    call_results[74] = call_result_74;

    let call_result_75 = callee_cdecl!(1, u32, OBJECT_BYTES_HEADER);
    call_results[75] = call_result_75;

    let call_result_76 = callee_thiscall!(26, u32, this_);
    call_results[76] = call_result_76;

    let call_result_77 = callee_thiscall!(10, u32, call_results[75], relocated(IMAGE_REF_00F0073C), call_results[76]);
    call_results[77] = call_result_77;
    unsafe { write_word(this_.wrapping_add(0x1E8), call_results[77]); }

    let call_result_78 = callee_thiscall!(13, u32, call_results[77]);
    call_results[78] = call_result_78;

    store_frame_word(&mut frame_slots, 0x12C, 0u32);
    store_frame_word(&mut frame_slots, 0x128, 0u32);
    let call_result_79 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x12C), 0u32, F32_TWO_BITS);
    call_results[79] = call_result_79;

    let call_result_80 = callee_thiscall!(26, u32, unsafe { read_word(this_.wrapping_add(0x1E4)) });
    call_results[80] = call_result_80;

    let call_result_81 = callee_thiscall!(29, u32, unsafe { read_word(this_.wrapping_add(0x1E8)) }, FIELD_KIND_INTEGER, call_results[80], FIELD_KIND_VECTOR);
    call_results[81] = call_result_81;

    let buffer_source_82 = call_results[79];
    let field_words_82 = unsafe { [
        read_word(buffer_source_82.wrapping_add(0)),
        read_word(buffer_source_82.wrapping_add(4)),
        read_word(buffer_source_82.wrapping_add(8)),
        read_word(buffer_source_82.wrapping_add(12)),
        read_word(buffer_source_82.wrapping_add(16)),
        read_word(buffer_source_82.wrapping_add(20)),
    ] };
    let field_tail_82 = unsafe { field_words_82.as_ptr().add(4) as usize as u32 };
    let call_result_82 = callee_thiscall!(2, u32, field_tail_82,
        field_words_82[0],
        field_words_82[1],
        field_words_82[2],
        field_words_82[3],
        field_words_82[4],
        field_words_82[5],
    );
    call_results[82] = call_result_82;

    store_frame_word(&mut frame_slots, 0x130, 0u32);
    store_frame_word(&mut frame_slots, 0x12C, 0u32);
    let call_result_83 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x130), 0u32, 0u32);
    call_results[83] = call_result_83;

    let call_result_84 = callee_thiscall!(28, u32, unsafe { read_word(this_.wrapping_add(0x1E8)) }, FIELD_KIND_FLOAT);
    call_results[84] = call_result_84;

    let buffer_source_85 = call_results[83];
    let field_words_85 = unsafe { [
        read_word(buffer_source_85.wrapping_add(0)),
        read_word(buffer_source_85.wrapping_add(4)),
        read_word(buffer_source_85.wrapping_add(8)),
        read_word(buffer_source_85.wrapping_add(12)),
        read_word(buffer_source_85.wrapping_add(16)),
        read_word(buffer_source_85.wrapping_add(20)),
    ] };
    let field_tail_85 = unsafe { field_words_85.as_ptr().add(4) as usize as u32 };
    let call_result_85 = callee_thiscall!(2, u32, field_tail_85,
        field_words_85[0],
        field_words_85[1],
        field_words_85[2],
        field_words_85[3],
        field_words_85[4],
        field_words_85[5],
    );
    call_results[85] = call_result_85;

    store_frame_word(&mut frame_slots, 0x130, 0u32);
    store_frame_word(&mut frame_slots, 0x12C, 0u32);
    let call_result_86 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x130), 0u32, 0u32);
    call_results[86] = call_result_86;

    let call_result_87 = callee_thiscall!(28, u32, unsafe { read_word(this_.wrapping_add(0x1E8)) }, FIELD_KIND_FLAG);
    call_results[87] = call_result_87;

    let buffer_source_88 = call_results[86];
    let field_words_88 = unsafe { [
        read_word(buffer_source_88.wrapping_add(0)),
        read_word(buffer_source_88.wrapping_add(4)),
        read_word(buffer_source_88.wrapping_add(8)),
        read_word(buffer_source_88.wrapping_add(12)),
        read_word(buffer_source_88.wrapping_add(16)),
        read_word(buffer_source_88.wrapping_add(20)),
    ] };
    let field_tail_88 = unsafe { field_words_88.as_ptr().add(4) as usize as u32 };
    let call_result_88 = callee_thiscall!(2, u32, field_tail_88,
        field_words_88[0],
        field_words_88[1],
        field_words_88[2],
        field_words_88[3],
        field_words_88[4],
        field_words_88[5],
    );
    call_results[88] = call_result_88;

    let call_result_89 = callee_thiscall!(27, u32, this_);
    call_results[89] = call_result_89;

    let call_result_90 = callee_thiscall!(31, u32, unsafe { read_word(this_.wrapping_add(0x1E8)) }, F32_EIGHTY_BITS, call_results[89]);
    call_results[90] = call_result_90;

    let call_result_91 = callee_thiscall!(27, u32, this_);
    call_results[91] = call_result_91;

    let call_result_92 = callee_thiscall!(32, u32, unsafe { read_word(this_.wrapping_add(0x1E8)) }, call_results[91]);
    call_results[92] = call_result_92;

    let call_result_93 = callee_cdecl!(1, u32, OBJECT_BYTES_ENTRY);
    call_results[93] = call_result_93;

    let call_result_94 = callee_thiscall!(26, u32, unsafe { read_word(this_.wrapping_add(0x1E8)) });
    call_results[94] = call_result_94;

    let call_result_95 = callee_thiscall!(9, u32, call_results[93], relocated(IMAGE_REF_00F00748), call_results[94]);
    call_results[95] = call_result_95;

    store_frame_word(&mut frame_slots, 0x118, 0u32);
    store_frame_word(&mut frame_slots, 0x114, 0u32);
    store_frame_word(&mut frame_slots, 0x110, 0u32);
    store_frame_word(&mut frame_slots, 0x10C, 0u32);
    store_frame_word(&mut frame_slots, 0x108, 0u32);
    store_frame_word(&mut frame_slots, 0x104, 0u32);
    store_frame_word(&mut frame_slots, 0x100, 0u32);
    store_frame_word(&mut frame_slots, 0x0FC, 0u32);
    let call_result_96 = callee_cdecl!(24, u32, frame_pointer(&mut frame_slots, 0x118), FIELD_KIND_FLOAT);
    call_results[96] = call_result_96;

    store_frame_word(&mut frame_slots, 0x114, 0u32);
    store_frame_word(&mut frame_slots, 0x110, 0u32);
    store_frame_word(&mut frame_slots, 0x10C, 0u32);
    store_frame_word(&mut frame_slots, 0x108, 0u32);
    store_frame_word(&mut frame_slots, 0x104, 0u32);
    store_frame_word(&mut frame_slots, 0x100, 0u32);
    store_frame_word(&mut frame_slots, 0x0FC, 0u32);
    store_frame_word(&mut frame_slots, 0x0F8, 0u32);
    let call_result_97 = callee_cdecl!(24, u32, frame_pointer(&mut frame_slots, 0x114), WORD_0000003E);
    call_results[97] = call_result_97;

    store_frame_word(&mut frame_slots, 0x134, 0u32);
    store_frame_word(&mut frame_slots, 0x130, 0u32);
    store_frame_word(&mut frame_slots, 0x12C, 0u32);
    store_frame_word(&mut frame_slots, 0x128, 0u32);
    store_frame_word(&mut frame_slots, 0x124, 0u32);
    store_frame_word(&mut frame_slots, 0x120, 0u32);
    store_frame_word(&mut frame_slots, 0x11C, 0u32);
    store_frame_word(&mut frame_slots, 0x118, 0u32);
    store_frame_word(&mut frame_slots, 0x134, DEFAULT_PROPERTY_COLOR);
    let call_result_98 = callee_thiscall!(12, u32, call_results[95], 1u32, relocated(IMAGE_REF_00F00754), frame_pointer(&mut frame_slots, 0x134), call_results[97], call_results[96]);
    call_results[98] = call_result_98;

    store_frame_word(&mut frame_slots, 0x130, 0u32);
    store_frame_word(&mut frame_slots, 0x12C, 0u32);
    let call_result_99 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x130), 0u32, 0u32);
    call_results[99] = call_result_99;

    let call_result_100 = callee_thiscall!(28, u32, call_results[95], WORD_00000006);
    call_results[100] = call_result_100;

    let buffer_source_101 = call_results[99];
    let field_words_101 = unsafe { [
        read_word(buffer_source_101.wrapping_add(0)),
        read_word(buffer_source_101.wrapping_add(4)),
        read_word(buffer_source_101.wrapping_add(8)),
        read_word(buffer_source_101.wrapping_add(12)),
        read_word(buffer_source_101.wrapping_add(16)),
        read_word(buffer_source_101.wrapping_add(20)),
    ] };
    let field_tail_101 = unsafe { field_words_101.as_ptr().add(4) as usize as u32 };
    let call_result_101 = callee_thiscall!(2, u32, field_tail_101,
        field_words_101[0],
        field_words_101[1],
        field_words_101[2],
        field_words_101[3],
        field_words_101[4],
        field_words_101[5],
    );
    call_results[101] = call_result_101;

    store_frame_word(&mut frame_slots, 0x130, 0u32);
    store_frame_word(&mut frame_slots, 0x12C, 0u32);
    let call_result_102 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x130), 0u32, 0u32);
    call_results[102] = call_result_102;

    let call_result_103 = callee_thiscall!(28, u32, call_results[95], WORD_0000000C);
    call_results[103] = call_result_103;

    let buffer_source_104 = call_results[102];
    let field_words_104 = unsafe { [
        read_word(buffer_source_104.wrapping_add(0)),
        read_word(buffer_source_104.wrapping_add(4)),
        read_word(buffer_source_104.wrapping_add(8)),
        read_word(buffer_source_104.wrapping_add(12)),
        read_word(buffer_source_104.wrapping_add(16)),
        read_word(buffer_source_104.wrapping_add(20)),
    ] };
    let field_tail_104 = unsafe { field_words_104.as_ptr().add(4) as usize as u32 };
    let call_result_104 = callee_thiscall!(2, u32, field_tail_104,
        field_words_104[0],
        field_words_104[1],
        field_words_104[2],
        field_words_104[3],
        field_words_104[4],
        field_words_104[5],
    );
    call_results[104] = call_result_104;

    let call_result_105 = callee_thiscall!(35, u32, call_results[95], WORD_41D00000);
    call_results[105] = call_result_105;

    let call_result_106 = callee_thiscall!(27, u32, this_);
    call_results[106] = call_result_106;

    let call_result_107 = callee_thiscall!(32, u32, call_results[95], call_results[106]);
    call_results[107] = call_result_107;

    let call_result_108 = callee_thiscall!(27, u32, this_);
    call_results[108] = call_result_108;

    let call_result_109 = callee_thiscall!(33, u32, call_results[95], call_results[108]);
    call_results[109] = call_result_109;

    let call_result_110 = callee_thiscall!(36, u32, call_results[95], 1u32);
    call_results[110] = call_result_110;

    let call_result_111 = callee_cdecl!(1, u32, WORD_000001E4);
    call_results[111] = call_result_111;

    let call_result_112 = callee_thiscall!(26, u32, unsafe { read_word(this_.wrapping_add(0x1E8)) });
    call_results[112] = call_result_112;

    let call_result_113 = callee_thiscall!(7, u32, call_results[111], relocated(IMAGE_REF_00F00760), call_results[112]);
    call_results[113] = call_result_113;
    unsafe { write_word(this_.wrapping_add(0x1F8), call_results[113]); }

    let call_result_114 = callee_thiscall!(8, u32, call_results[113]);
    call_results[114] = call_result_114;

    store_frame_word(&mut frame_slots, 0x130, 0u32);
    store_frame_word(&mut frame_slots, 0x12C, 0u32);
    let call_result_115 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x130), 0u32, 0u32);
    call_results[115] = call_result_115;

    let call_result_116 = callee_thiscall!(27, u32, call_results[95]);
    call_results[116] = call_result_116;

    let call_result_117 = callee_thiscall!(30, u32, unsafe { read_word(this_.wrapping_add(0x1F8)) }, FIELD_KIND_INTEGER, call_results[116], FIELD_KIND_VECTOR);
    call_results[117] = call_result_117;

    let buffer_source_118 = call_results[115];
    let field_words_118 = unsafe { [
        read_word(buffer_source_118.wrapping_add(0)),
        read_word(buffer_source_118.wrapping_add(4)),
        read_word(buffer_source_118.wrapping_add(8)),
        read_word(buffer_source_118.wrapping_add(12)),
        read_word(buffer_source_118.wrapping_add(16)),
        read_word(buffer_source_118.wrapping_add(20)),
    ] };
    let field_tail_118 = unsafe { field_words_118.as_ptr().add(4) as usize as u32 };
    let call_result_118 = callee_thiscall!(2, u32, field_tail_118,
        field_words_118[0],
        field_words_118[1],
        field_words_118[2],
        field_words_118[3],
        field_words_118[4],
        field_words_118[5],
    );
    call_results[118] = call_result_118;

    store_frame_word(&mut frame_slots, 0x134, 0u32);
    store_frame_word(&mut frame_slots, 0x130, 0u32);
    let call_result_119 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x134), 0u32, F32_NEGATIVE_58_BITS);
    call_results[119] = call_result_119;

    let call_result_120 = callee_thiscall!(27, u32, this_);
    call_results[120] = call_result_120;

    let call_result_121 = callee_thiscall!(30, u32, unsafe { read_word(this_.wrapping_add(0x1F8)) }, FIELD_KIND_VECTOR, call_results[120], FIELD_KIND_VECTOR);
    call_results[121] = call_result_121;

    let buffer_source_122 = call_results[119];
    let field_words_122 = unsafe { [
        read_word(buffer_source_122.wrapping_add(0)),
        read_word(buffer_source_122.wrapping_add(4)),
        read_word(buffer_source_122.wrapping_add(8)),
        read_word(buffer_source_122.wrapping_add(12)),
        read_word(buffer_source_122.wrapping_add(16)),
        read_word(buffer_source_122.wrapping_add(20)),
    ] };
    let field_tail_122 = unsafe { field_words_122.as_ptr().add(4) as usize as u32 };
    let call_result_122 = callee_thiscall!(2, u32, field_tail_122,
        field_words_122[0],
        field_words_122[1],
        field_words_122[2],
        field_words_122[3],
        field_words_122[4],
        field_words_122[5],
    );
    call_results[122] = call_result_122;

    store_frame_word(&mut frame_slots, 0x138, 0u32);
    store_frame_word(&mut frame_slots, 0x134, 0u32);
    let call_result_123 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x138), 0u32, 0u32);
    call_results[123] = call_result_123;

    let call_result_124 = callee_thiscall!(28, u32, unsafe { read_word(this_.wrapping_add(0x1F8)) }, FIELD_KIND_FLOAT);
    call_results[124] = call_result_124;

    let buffer_source_125 = call_results[123];
    let field_words_125 = unsafe { [
        read_word(buffer_source_125.wrapping_add(0)),
        read_word(buffer_source_125.wrapping_add(4)),
        read_word(buffer_source_125.wrapping_add(8)),
        read_word(buffer_source_125.wrapping_add(12)),
        read_word(buffer_source_125.wrapping_add(16)),
        read_word(buffer_source_125.wrapping_add(20)),
    ] };
    let field_tail_125 = unsafe { field_words_125.as_ptr().add(4) as usize as u32 };
    let call_result_125 = callee_thiscall!(2, u32, field_tail_125,
        field_words_125[0],
        field_words_125[1],
        field_words_125[2],
        field_words_125[3],
        field_words_125[4],
        field_words_125[5],
    );
    call_results[125] = call_result_125;

    store_frame_word(&mut frame_slots, 0x138, 0u32);
    store_frame_word(&mut frame_slots, 0x134, 0u32);
    let call_result_126 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x138), 0u32, 0u32);
    call_results[126] = call_result_126;

    let call_result_127 = callee_thiscall!(28, u32, unsafe { read_word(this_.wrapping_add(0x1F8)) }, FIELD_KIND_FLAG);
    call_results[127] = call_result_127;

    let buffer_source_128 = call_results[126];
    let field_words_128 = unsafe { [
        read_word(buffer_source_128.wrapping_add(0)),
        read_word(buffer_source_128.wrapping_add(4)),
        read_word(buffer_source_128.wrapping_add(8)),
        read_word(buffer_source_128.wrapping_add(12)),
        read_word(buffer_source_128.wrapping_add(16)),
        read_word(buffer_source_128.wrapping_add(20)),
    ] };
    let field_tail_128 = unsafe { field_words_128.as_ptr().add(4) as usize as u32 };
    let call_result_128 = callee_thiscall!(2, u32, field_tail_128,
        field_words_128[0],
        field_words_128[1],
        field_words_128[2],
        field_words_128[3],
        field_words_128[4],
        field_words_128[5],
    );
    call_results[128] = call_result_128;

    let call_float_129 = callee_thiscall!(39, f32, unsafe { read_word(this_.wrapping_add(0x1F8)) });
    call_results[129] = call_float_129.to_bits();

    let call_result_130 = callee_thiscall!(35, u32, unsafe { read_word(this_.wrapping_add(0x1E8)) }, (core::hint::black_box(f32::from_bits(call_results[129])) + core::hint::black_box(f32::from_bits(FLOAT_ADJUSTMENT_BITS))).to_bits());
    call_results[130] = call_result_130;

    let call_result_131 = callee_thiscall!(11, u32, unsafe { read_word(this_.wrapping_add(0x1E8)) });
    call_results[131] = call_result_131;

    let call_result_132 = callee_cdecl!(1, u32, OBJECT_BYTES_HEADER);
    call_results[132] = call_result_132;

    let call_result_133 = callee_thiscall!(26, u32, this_);
    call_results[133] = call_result_133;

    let call_result_134 = callee_thiscall!(10, u32, call_results[132], relocated(IMAGE_REF_00F00770), call_results[133]);
    call_results[134] = call_result_134;
    unsafe { write_word(this_.wrapping_add(0x1EC), call_results[134]); }

    let call_result_135 = callee_thiscall!(13, u32, call_results[134]);
    call_results[135] = call_result_135;

    store_frame_word(&mut frame_slots, 0x138, 0u32);
    store_frame_word(&mut frame_slots, 0x134, 0u32);
    let call_result_136 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x138), 0u32, F32_TWO_BITS);
    call_results[136] = call_result_136;

    let call_result_137 = callee_thiscall!(26, u32, unsafe { read_word(this_.wrapping_add(0x1E8)) });
    call_results[137] = call_result_137;

    let call_result_138 = callee_thiscall!(29, u32, unsafe { read_word(this_.wrapping_add(0x1EC)) }, FIELD_KIND_INTEGER, call_results[137], FIELD_KIND_VECTOR);
    call_results[138] = call_result_138;

    let buffer_source_139 = call_results[136];
    let field_words_139 = unsafe { [
        read_word(buffer_source_139.wrapping_add(0)),
        read_word(buffer_source_139.wrapping_add(4)),
        read_word(buffer_source_139.wrapping_add(8)),
        read_word(buffer_source_139.wrapping_add(12)),
        read_word(buffer_source_139.wrapping_add(16)),
        read_word(buffer_source_139.wrapping_add(20)),
    ] };
    let field_tail_139 = unsafe { field_words_139.as_ptr().add(4) as usize as u32 };
    let call_result_139 = callee_thiscall!(2, u32, field_tail_139,
        field_words_139[0],
        field_words_139[1],
        field_words_139[2],
        field_words_139[3],
        field_words_139[4],
        field_words_139[5],
    );
    call_results[139] = call_result_139;

    store_frame_word(&mut frame_slots, 0x13C, 0u32);
    store_frame_word(&mut frame_slots, 0x138, 0u32);
    let call_result_140 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x13C), 0u32, 0u32);
    call_results[140] = call_result_140;

    let call_result_141 = callee_thiscall!(28, u32, unsafe { read_word(this_.wrapping_add(0x1EC)) }, FIELD_KIND_FLOAT);
    call_results[141] = call_result_141;

    let buffer_source_142 = call_results[140];
    let field_words_142 = unsafe { [
        read_word(buffer_source_142.wrapping_add(0)),
        read_word(buffer_source_142.wrapping_add(4)),
        read_word(buffer_source_142.wrapping_add(8)),
        read_word(buffer_source_142.wrapping_add(12)),
        read_word(buffer_source_142.wrapping_add(16)),
        read_word(buffer_source_142.wrapping_add(20)),
    ] };
    let field_tail_142 = unsafe { field_words_142.as_ptr().add(4) as usize as u32 };
    let call_result_142 = callee_thiscall!(2, u32, field_tail_142,
        field_words_142[0],
        field_words_142[1],
        field_words_142[2],
        field_words_142[3],
        field_words_142[4],
        field_words_142[5],
    );
    call_results[142] = call_result_142;

    store_frame_word(&mut frame_slots, 0x13C, 0u32);
    store_frame_word(&mut frame_slots, 0x138, 0u32);
    let call_result_143 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x13C), 0u32, 0u32);
    call_results[143] = call_result_143;

    let call_result_144 = callee_thiscall!(28, u32, unsafe { read_word(this_.wrapping_add(0x1EC)) }, FIELD_KIND_FLAG);
    call_results[144] = call_result_144;

    let buffer_source_145 = call_results[143];
    let field_words_145 = unsafe { [
        read_word(buffer_source_145.wrapping_add(0)),
        read_word(buffer_source_145.wrapping_add(4)),
        read_word(buffer_source_145.wrapping_add(8)),
        read_word(buffer_source_145.wrapping_add(12)),
        read_word(buffer_source_145.wrapping_add(16)),
        read_word(buffer_source_145.wrapping_add(20)),
    ] };
    let field_tail_145 = unsafe { field_words_145.as_ptr().add(4) as usize as u32 };
    let call_result_145 = callee_thiscall!(2, u32, field_tail_145,
        field_words_145[0],
        field_words_145[1],
        field_words_145[2],
        field_words_145[3],
        field_words_145[4],
        field_words_145[5],
    );
    call_results[145] = call_result_145;

    let call_result_146 = callee_thiscall!(35, u32, unsafe { read_word(this_.wrapping_add(0x1EC)) }, F32_THREE_HUNDRED_TWENTY_BITS);
    call_results[146] = call_result_146;

    let call_result_147 = callee_thiscall!(27, u32, this_);
    call_results[147] = call_result_147;

    let call_result_148 = callee_thiscall!(32, u32, unsafe { read_word(this_.wrapping_add(0x1EC)) }, call_results[147]);
    call_results[148] = call_result_148;

    let call_result_149 = callee_cdecl!(1, u32, OBJECT_BYTES_ENTRY);
    call_results[149] = call_result_149;

    let call_result_150 = callee_thiscall!(26, u32, unsafe { read_word(this_.wrapping_add(0x1EC)) });
    call_results[150] = call_result_150;

    let call_result_151 = callee_thiscall!(9, u32, call_results[149], relocated(IMAGE_REF_00F0077C), call_results[150]);
    call_results[151] = call_result_151;

    store_frame_word(&mut frame_slots, 0x124, 0u32);
    store_frame_word(&mut frame_slots, 0x120, 0u32);
    store_frame_word(&mut frame_slots, 0x11C, 0u32);
    store_frame_word(&mut frame_slots, 0x118, 0u32);
    store_frame_word(&mut frame_slots, 0x114, 0u32);
    store_frame_word(&mut frame_slots, 0x110, 0u32);
    store_frame_word(&mut frame_slots, 0x10C, 0u32);
    store_frame_word(&mut frame_slots, 0x108, 0u32);
    let call_result_152 = callee_cdecl!(24, u32, frame_pointer(&mut frame_slots, 0x124), FIELD_KIND_FLOAT);
    call_results[152] = call_result_152;

    store_frame_word(&mut frame_slots, 0x120, 0u32);
    store_frame_word(&mut frame_slots, 0x11C, 0u32);
    store_frame_word(&mut frame_slots, 0x118, 0u32);
    store_frame_word(&mut frame_slots, 0x114, 0u32);
    store_frame_word(&mut frame_slots, 0x110, 0u32);
    store_frame_word(&mut frame_slots, 0x10C, 0u32);
    store_frame_word(&mut frame_slots, 0x108, 0u32);
    store_frame_word(&mut frame_slots, 0x104, 0u32);
    let call_result_153 = callee_cdecl!(24, u32, frame_pointer(&mut frame_slots, 0x120), WORD_0000003E);
    call_results[153] = call_result_153;

    store_frame_word(&mut frame_slots, 0x140, 0u32);
    store_frame_word(&mut frame_slots, 0x13C, 0u32);
    store_frame_word(&mut frame_slots, 0x138, 0u32);
    store_frame_word(&mut frame_slots, 0x134, 0u32);
    store_frame_word(&mut frame_slots, 0x130, 0u32);
    store_frame_word(&mut frame_slots, 0x12C, 0u32);
    store_frame_word(&mut frame_slots, 0x128, 0u32);
    store_frame_word(&mut frame_slots, 0x124, 0u32);
    store_frame_word(&mut frame_slots, 0x140, DEFAULT_PROPERTY_COLOR);
    let call_result_154 = callee_thiscall!(12, u32, call_results[151], FIELD_KIND_FLOAT, relocated(IMAGE_REF_00F00788), frame_pointer(&mut frame_slots, 0x140), call_results[153], call_results[152]);
    call_results[154] = call_result_154;

    store_frame_word(&mut frame_slots, 0x13C, 0u32);
    store_frame_word(&mut frame_slots, 0x138, 0u32);
    let call_result_155 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x13C), 0u32, 0u32);
    call_results[155] = call_result_155;

    let call_result_156 = callee_thiscall!(28, u32, call_results[151], WORD_00000006);
    call_results[156] = call_result_156;

    let buffer_source_157 = call_results[155];
    let field_words_157 = unsafe { [
        read_word(buffer_source_157.wrapping_add(0)),
        read_word(buffer_source_157.wrapping_add(4)),
        read_word(buffer_source_157.wrapping_add(8)),
        read_word(buffer_source_157.wrapping_add(12)),
        read_word(buffer_source_157.wrapping_add(16)),
        read_word(buffer_source_157.wrapping_add(20)),
    ] };
    let field_tail_157 = unsafe { field_words_157.as_ptr().add(4) as usize as u32 };
    let call_result_157 = callee_thiscall!(2, u32, field_tail_157,
        field_words_157[0],
        field_words_157[1],
        field_words_157[2],
        field_words_157[3],
        field_words_157[4],
        field_words_157[5],
    );
    call_results[157] = call_result_157;

    store_frame_word(&mut frame_slots, 0x13C, 0u32);
    store_frame_word(&mut frame_slots, 0x138, 0u32);
    let call_result_158 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x13C), 0u32, 0u32);
    call_results[158] = call_result_158;

    let call_result_159 = callee_thiscall!(28, u32, call_results[151], WORD_0000000C);
    call_results[159] = call_result_159;

    let buffer_source_160 = call_results[158];
    let field_words_160 = unsafe { [
        read_word(buffer_source_160.wrapping_add(0)),
        read_word(buffer_source_160.wrapping_add(4)),
        read_word(buffer_source_160.wrapping_add(8)),
        read_word(buffer_source_160.wrapping_add(12)),
        read_word(buffer_source_160.wrapping_add(16)),
        read_word(buffer_source_160.wrapping_add(20)),
    ] };
    let field_tail_160 = unsafe { field_words_160.as_ptr().add(4) as usize as u32 };
    let call_result_160 = callee_thiscall!(2, u32, field_tail_160,
        field_words_160[0],
        field_words_160[1],
        field_words_160[2],
        field_words_160[3],
        field_words_160[4],
        field_words_160[5],
    );
    call_results[160] = call_result_160;

    let call_result_161 = callee_thiscall!(35, u32, call_results[151], WORD_41D00000);
    call_results[161] = call_result_161;

    let call_result_162 = callee_thiscall!(27, u32, this_);
    call_results[162] = call_result_162;

    let call_result_163 = callee_thiscall!(32, u32, call_results[151], call_results[162]);
    call_results[163] = call_result_163;

    let call_result_164 = callee_thiscall!(27, u32, this_);
    call_results[164] = call_result_164;

    let call_result_165 = callee_thiscall!(33, u32, call_results[151], call_results[164]);
    call_results[165] = call_result_165;

    let call_result_166 = callee_thiscall!(36, u32, call_results[151], 1u32);
    call_results[166] = call_result_166;

    let call_result_167 = callee_cdecl!(1, u32, OBJECT_BYTES_CLIP_ITEM);
    call_results[167] = call_result_167;

    let call_result_168 = callee_thiscall!(26, u32, unsafe { read_word(this_.wrapping_add(0x1EC)) });
    call_results[168] = call_result_168;

    let call_result_169 = callee_thiscall!(14, u32, call_results[167], relocated(IMAGE_REF_00F00790), call_results[168]);
    call_results[169] = call_result_169;
    unsafe { write_word(this_.wrapping_add(0x1FC), call_results[169]); }

    let call_result_170 = callee_thiscall!(15, u32, call_results[169]);
    call_results[170] = call_result_170;

    store_frame_word(&mut frame_slots, 0x13C, 0u32);
    store_frame_word(&mut frame_slots, 0x138, 0u32);
    let call_result_171 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x13C), 0u32, 0u32);
    call_results[171] = call_result_171;

    let call_result_172 = callee_thiscall!(27, u32, call_results[151]);
    call_results[172] = call_result_172;

    let call_result_173 = callee_thiscall!(30, u32, unsafe { read_word(this_.wrapping_add(0x1FC)) }, FIELD_KIND_INTEGER, call_results[172], FIELD_KIND_VECTOR);
    call_results[173] = call_result_173;

    let buffer_source_174 = call_results[171];
    let field_words_174 = unsafe { [
        read_word(buffer_source_174.wrapping_add(0)),
        read_word(buffer_source_174.wrapping_add(4)),
        read_word(buffer_source_174.wrapping_add(8)),
        read_word(buffer_source_174.wrapping_add(12)),
        read_word(buffer_source_174.wrapping_add(16)),
        read_word(buffer_source_174.wrapping_add(20)),
    ] };
    let field_tail_174 = unsafe { field_words_174.as_ptr().add(4) as usize as u32 };
    let call_result_174 = callee_thiscall!(2, u32, field_tail_174,
        field_words_174[0],
        field_words_174[1],
        field_words_174[2],
        field_words_174[3],
        field_words_174[4],
        field_words_174[5],
    );
    call_results[174] = call_result_174;

    store_frame_word(&mut frame_slots, 0x140, 0u32);
    store_frame_word(&mut frame_slots, 0x13C, 0u32);
    let call_result_175 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x140), 0u32, F32_NEGATIVE_30_BITS);
    call_results[175] = call_result_175;

    let call_result_176 = callee_thiscall!(27, u32, this_);
    call_results[176] = call_result_176;

    let call_result_177 = callee_thiscall!(30, u32, unsafe { read_word(this_.wrapping_add(0x1FC)) }, FIELD_KIND_VECTOR, call_results[176], FIELD_KIND_VECTOR);
    call_results[177] = call_result_177;

    let buffer_source_178 = call_results[175];
    let field_words_178 = unsafe { [
        read_word(buffer_source_178.wrapping_add(0)),
        read_word(buffer_source_178.wrapping_add(4)),
        read_word(buffer_source_178.wrapping_add(8)),
        read_word(buffer_source_178.wrapping_add(12)),
        read_word(buffer_source_178.wrapping_add(16)),
        read_word(buffer_source_178.wrapping_add(20)),
    ] };
    let field_tail_178 = unsafe { field_words_178.as_ptr().add(4) as usize as u32 };
    let call_result_178 = callee_thiscall!(2, u32, field_tail_178,
        field_words_178[0],
        field_words_178[1],
        field_words_178[2],
        field_words_178[3],
        field_words_178[4],
        field_words_178[5],
    );
    call_results[178] = call_result_178;

    store_frame_word(&mut frame_slots, 0x144, 0u32);
    store_frame_word(&mut frame_slots, 0x140, 0u32);
    let call_result_179 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x144), 0u32, 0u32);
    call_results[179] = call_result_179;

    let call_result_180 = callee_thiscall!(28, u32, unsafe { read_word(this_.wrapping_add(0x1FC)) }, FIELD_KIND_FLOAT);
    call_results[180] = call_result_180;

    let buffer_source_181 = call_results[179];
    let field_words_181 = unsafe { [
        read_word(buffer_source_181.wrapping_add(0)),
        read_word(buffer_source_181.wrapping_add(4)),
        read_word(buffer_source_181.wrapping_add(8)),
        read_word(buffer_source_181.wrapping_add(12)),
        read_word(buffer_source_181.wrapping_add(16)),
        read_word(buffer_source_181.wrapping_add(20)),
    ] };
    let field_tail_181 = unsafe { field_words_181.as_ptr().add(4) as usize as u32 };
    let call_result_181 = callee_thiscall!(2, u32, field_tail_181,
        field_words_181[0],
        field_words_181[1],
        field_words_181[2],
        field_words_181[3],
        field_words_181[4],
        field_words_181[5],
    );
    call_results[181] = call_result_181;

    store_frame_word(&mut frame_slots, 0x144, 0u32);
    store_frame_word(&mut frame_slots, 0x140, 0u32);
    let call_result_182 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x144), 0u32, 0u32);
    call_results[182] = call_result_182;

    let call_result_183 = callee_thiscall!(28, u32, unsafe { read_word(this_.wrapping_add(0x1FC)) }, FIELD_KIND_FLAG);
    call_results[183] = call_result_183;

    let buffer_source_184 = call_results[182];
    let field_words_184 = unsafe { [
        read_word(buffer_source_184.wrapping_add(0)),
        read_word(buffer_source_184.wrapping_add(4)),
        read_word(buffer_source_184.wrapping_add(8)),
        read_word(buffer_source_184.wrapping_add(12)),
        read_word(buffer_source_184.wrapping_add(16)),
        read_word(buffer_source_184.wrapping_add(20)),
    ] };
    let field_tail_184 = unsafe { field_words_184.as_ptr().add(4) as usize as u32 };
    let call_result_184 = callee_thiscall!(2, u32, field_tail_184,
        field_words_184[0],
        field_words_184[1],
        field_words_184[2],
        field_words_184[3],
        field_words_184[4],
        field_words_184[5],
    );
    call_results[184] = call_result_184;

    let call_float_185 = callee_thiscall!(39, f32, unsafe { read_word(this_.wrapping_add(0x1FC)) });
    call_results[185] = call_float_185.to_bits();

    let call_result_186 = callee_thiscall!(35, u32, unsafe { read_word(this_.wrapping_add(0x1EC)) }, (core::hint::black_box(f32::from_bits(call_results[185])) + core::hint::black_box(f32::from_bits(FLOAT_ADJUSTMENT_BITS))).to_bits());
    call_results[186] = call_result_186;

    let call_result_187 = callee_thiscall!(11, u32, unsafe { read_word(this_.wrapping_add(0x1EC)) });
    call_results[187] = call_result_187;

    let call_result_188 = callee_cdecl!(1, u32, OBJECT_BYTES_HEADER);
    call_results[188] = call_result_188;

    let call_result_189 = callee_thiscall!(26, u32, this_);
    call_results[189] = call_result_189;

    let call_result_190 = callee_thiscall!(10, u32, call_results[188], relocated(IMAGE_REF_00F0079C), call_results[189]);
    call_results[190] = call_result_190;
    unsafe { write_word(this_.wrapping_add(0x1F0), call_results[190]); }

    let call_result_191 = callee_thiscall!(13, u32, call_results[190]);
    call_results[191] = call_result_191;

    store_frame_word(&mut frame_slots, 0x144, 0u32);
    store_frame_word(&mut frame_slots, 0x140, 0u32);
    let call_result_192 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x144), 0u32, F32_TWO_BITS);
    call_results[192] = call_result_192;

    let call_result_193 = callee_thiscall!(26, u32, unsafe { read_word(this_.wrapping_add(0x1EC)) });
    call_results[193] = call_result_193;

    let call_result_194 = callee_thiscall!(29, u32, unsafe { read_word(this_.wrapping_add(0x1F0)) }, FIELD_KIND_INTEGER, call_results[193], FIELD_KIND_VECTOR);
    call_results[194] = call_result_194;

    let buffer_source_195 = call_results[192];
    let field_words_195 = unsafe { [
        read_word(buffer_source_195.wrapping_add(0)),
        read_word(buffer_source_195.wrapping_add(4)),
        read_word(buffer_source_195.wrapping_add(8)),
        read_word(buffer_source_195.wrapping_add(12)),
        read_word(buffer_source_195.wrapping_add(16)),
        read_word(buffer_source_195.wrapping_add(20)),
    ] };
    let field_tail_195 = unsafe { field_words_195.as_ptr().add(4) as usize as u32 };
    let call_result_195 = callee_thiscall!(2, u32, field_tail_195,
        field_words_195[0],
        field_words_195[1],
        field_words_195[2],
        field_words_195[3],
        field_words_195[4],
        field_words_195[5],
    );
    call_results[195] = call_result_195;

    store_frame_word(&mut frame_slots, 0x148, 0u32);
    store_frame_word(&mut frame_slots, 0x144, 0u32);
    let call_result_196 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x148), 0u32, 0u32);
    call_results[196] = call_result_196;

    let call_result_197 = callee_thiscall!(28, u32, unsafe { read_word(this_.wrapping_add(0x1F0)) }, FIELD_KIND_FLOAT);
    call_results[197] = call_result_197;

    let buffer_source_198 = call_results[196];
    let field_words_198 = unsafe { [
        read_word(buffer_source_198.wrapping_add(0)),
        read_word(buffer_source_198.wrapping_add(4)),
        read_word(buffer_source_198.wrapping_add(8)),
        read_word(buffer_source_198.wrapping_add(12)),
        read_word(buffer_source_198.wrapping_add(16)),
        read_word(buffer_source_198.wrapping_add(20)),
    ] };
    let field_tail_198 = unsafe { field_words_198.as_ptr().add(4) as usize as u32 };
    let call_result_198 = callee_thiscall!(2, u32, field_tail_198,
        field_words_198[0],
        field_words_198[1],
        field_words_198[2],
        field_words_198[3],
        field_words_198[4],
        field_words_198[5],
    );
    call_results[198] = call_result_198;

    store_frame_word(&mut frame_slots, 0x148, 0u32);
    store_frame_word(&mut frame_slots, 0x144, 0u32);
    let call_result_199 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x148), 0u32, 0u32);
    call_results[199] = call_result_199;

    let call_result_200 = callee_thiscall!(28, u32, unsafe { read_word(this_.wrapping_add(0x1F0)) }, FIELD_KIND_FLAG);
    call_results[200] = call_result_200;

    let buffer_source_201 = call_results[199];
    let field_words_201 = unsafe { [
        read_word(buffer_source_201.wrapping_add(0)),
        read_word(buffer_source_201.wrapping_add(4)),
        read_word(buffer_source_201.wrapping_add(8)),
        read_word(buffer_source_201.wrapping_add(12)),
        read_word(buffer_source_201.wrapping_add(16)),
        read_word(buffer_source_201.wrapping_add(20)),
    ] };
    let field_tail_201 = unsafe { field_words_201.as_ptr().add(4) as usize as u32 };
    let call_result_201 = callee_thiscall!(2, u32, field_tail_201,
        field_words_201[0],
        field_words_201[1],
        field_words_201[2],
        field_words_201[3],
        field_words_201[4],
        field_words_201[5],
    );
    call_results[201] = call_result_201;

    store_frame_word(&mut frame_slots, 0x148, 0u32);
    store_frame_word(&mut frame_slots, 0x144, 0u32);
    let call_result_202 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x148), 0u32, 0u32);
    call_results[202] = call_result_202;

    let call_result_203 = callee_thiscall!(27, u32, this_);
    call_results[203] = call_result_203;

    let call_result_204 = callee_thiscall!(30, u32, unsafe { read_word(this_.wrapping_add(0x1F0)) }, FIELD_KIND_VECTOR, call_results[203], FIELD_KIND_VECTOR);
    call_results[204] = call_result_204;

    let buffer_source_205 = call_results[202];
    let field_words_205 = unsafe { [
        read_word(buffer_source_205.wrapping_add(0)),
        read_word(buffer_source_205.wrapping_add(4)),
        read_word(buffer_source_205.wrapping_add(8)),
        read_word(buffer_source_205.wrapping_add(12)),
        read_word(buffer_source_205.wrapping_add(16)),
        read_word(buffer_source_205.wrapping_add(20)),
    ] };
    let field_tail_205 = unsafe { field_words_205.as_ptr().add(4) as usize as u32 };
    let call_result_205 = callee_thiscall!(2, u32, field_tail_205,
        field_words_205[0],
        field_words_205[1],
        field_words_205[2],
        field_words_205[3],
        field_words_205[4],
        field_words_205[5],
    );
    call_results[205] = call_result_205;

    let call_result_206 = callee_thiscall!(27, u32, this_);
    call_results[206] = call_result_206;

    let call_result_207 = callee_thiscall!(32, u32, unsafe { read_word(this_.wrapping_add(0x1F0)) }, call_results[206]);
    call_results[207] = call_result_207;

    let call_result_208 = callee_cdecl!(1, u32, OBJECT_BYTES_ENTRY);
    call_results[208] = call_result_208;

    let call_result_209 = callee_thiscall!(26, u32, unsafe { read_word(this_.wrapping_add(0x1F0)) });
    call_results[209] = call_result_209;

    let call_result_210 = callee_thiscall!(9, u32, call_results[208], relocated(IMAGE_REF_00F007B0), call_results[209]);
    call_results[210] = call_result_210;

    store_frame_word(&mut frame_slots, 0x134, 0u32);
    store_frame_word(&mut frame_slots, 0x130, 0u32);
    store_frame_word(&mut frame_slots, 0x12C, 0u32);
    store_frame_word(&mut frame_slots, 0x128, 0u32);
    store_frame_word(&mut frame_slots, 0x124, 0u32);
    store_frame_word(&mut frame_slots, 0x120, 0u32);
    store_frame_word(&mut frame_slots, 0x11C, 0u32);
    store_frame_word(&mut frame_slots, 0x118, 0u32);
    let call_result_211 = callee_cdecl!(24, u32, frame_pointer(&mut frame_slots, 0x134), FIELD_KIND_FLOAT);
    call_results[211] = call_result_211;

    store_frame_word(&mut frame_slots, 0x130, 0u32);
    store_frame_word(&mut frame_slots, 0x12C, 0u32);
    store_frame_word(&mut frame_slots, 0x128, 0u32);
    store_frame_word(&mut frame_slots, 0x124, 0u32);
    store_frame_word(&mut frame_slots, 0x120, 0u32);
    store_frame_word(&mut frame_slots, 0x11C, 0u32);
    store_frame_word(&mut frame_slots, 0x118, 0u32);
    store_frame_word(&mut frame_slots, 0x114, 0u32);
    let call_result_212 = callee_cdecl!(24, u32, frame_pointer(&mut frame_slots, 0x130), WORD_0000003E);
    call_results[212] = call_result_212;

    store_frame_word(&mut frame_slots, 0x150, 0u32);
    store_frame_word(&mut frame_slots, 0x14C, 0u32);
    store_frame_word(&mut frame_slots, 0x148, 0u32);
    store_frame_word(&mut frame_slots, 0x144, 0u32);
    store_frame_word(&mut frame_slots, 0x140, 0u32);
    store_frame_word(&mut frame_slots, 0x13C, 0u32);
    store_frame_word(&mut frame_slots, 0x138, 0u32);
    store_frame_word(&mut frame_slots, 0x134, 0u32);
    store_frame_word(&mut frame_slots, 0x150, DEFAULT_PROPERTY_COLOR);
    let call_result_213 = callee_thiscall!(12, u32, call_results[210], WORD_00000003, relocated(IMAGE_REF_00F007C4), frame_pointer(&mut frame_slots, 0x150), call_results[212], call_results[211]);
    call_results[213] = call_result_213;

    store_frame_word(&mut frame_slots, 0x14C, 0u32);
    store_frame_word(&mut frame_slots, 0x148, 0u32);
    let call_result_214 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x14C), 0u32, 0u32);
    call_results[214] = call_result_214;

    let call_result_215 = callee_thiscall!(28, u32, call_results[210], WORD_00000006);
    call_results[215] = call_result_215;

    let buffer_source_216 = call_results[214];
    let field_words_216 = unsafe { [
        read_word(buffer_source_216.wrapping_add(0)),
        read_word(buffer_source_216.wrapping_add(4)),
        read_word(buffer_source_216.wrapping_add(8)),
        read_word(buffer_source_216.wrapping_add(12)),
        read_word(buffer_source_216.wrapping_add(16)),
        read_word(buffer_source_216.wrapping_add(20)),
    ] };
    let field_tail_216 = unsafe { field_words_216.as_ptr().add(4) as usize as u32 };
    let call_result_216 = callee_thiscall!(2, u32, field_tail_216,
        field_words_216[0],
        field_words_216[1],
        field_words_216[2],
        field_words_216[3],
        field_words_216[4],
        field_words_216[5],
    );
    call_results[216] = call_result_216;

    store_frame_word(&mut frame_slots, 0x14C, 0u32);
    store_frame_word(&mut frame_slots, 0x148, 0u32);
    let call_result_217 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x14C), 0u32, 0u32);
    call_results[217] = call_result_217;

    let call_result_218 = callee_thiscall!(28, u32, call_results[210], WORD_0000000C);
    call_results[218] = call_result_218;

    let buffer_source_219 = call_results[217];
    let field_words_219 = unsafe { [
        read_word(buffer_source_219.wrapping_add(0)),
        read_word(buffer_source_219.wrapping_add(4)),
        read_word(buffer_source_219.wrapping_add(8)),
        read_word(buffer_source_219.wrapping_add(12)),
        read_word(buffer_source_219.wrapping_add(16)),
        read_word(buffer_source_219.wrapping_add(20)),
    ] };
    let field_tail_219 = unsafe { field_words_219.as_ptr().add(4) as usize as u32 };
    let call_result_219 = callee_thiscall!(2, u32, field_tail_219,
        field_words_219[0],
        field_words_219[1],
        field_words_219[2],
        field_words_219[3],
        field_words_219[4],
        field_words_219[5],
    );
    call_results[219] = call_result_219;

    let call_result_220 = callee_thiscall!(35, u32, call_results[210], WORD_41D00000);
    call_results[220] = call_result_220;

    let call_result_221 = callee_thiscall!(27, u32, this_);
    call_results[221] = call_result_221;

    let call_result_222 = callee_thiscall!(32, u32, call_results[210], call_results[221]);
    call_results[222] = call_result_222;

    let call_result_223 = callee_thiscall!(27, u32, this_);
    call_results[223] = call_result_223;

    let call_result_224 = callee_thiscall!(33, u32, call_results[210], call_results[223]);
    call_results[224] = call_result_224;

    let call_result_225 = callee_thiscall!(36, u32, call_results[210], 1u32);
    call_results[225] = call_result_225;

    let call_result_226 = callee_cdecl!(1, u32, WORD_000001E4);
    call_results[226] = call_result_226;

    let call_result_227 = callee_thiscall!(26, u32, unsafe { read_word(this_.wrapping_add(0x1F0)) });
    call_results[227] = call_result_227;

    let call_result_228 = callee_thiscall!(16, u32, call_results[226], relocated(IMAGE_REF_00F007D0), call_results[227]);
    call_results[228] = call_result_228;
    unsafe { write_word(this_.wrapping_add(0x200), call_results[228]); }

    let call_result_229 = callee_thiscall!(17, u32, call_results[228]);
    call_results[229] = call_result_229;

    store_frame_word(&mut frame_slots, 0x14C, 0u32);
    store_frame_word(&mut frame_slots, 0x148, 0u32);
    let call_result_230 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x14C), 0u32, 0u32);
    call_results[230] = call_result_230;

    let call_result_231 = callee_thiscall!(27, u32, call_results[210]);
    call_results[231] = call_result_231;

    let call_result_232 = callee_thiscall!(30, u32, unsafe { read_word(this_.wrapping_add(0x200)) }, FIELD_KIND_INTEGER, call_results[231], FIELD_KIND_VECTOR);
    call_results[232] = call_result_232;

    let buffer_source_233 = call_results[230];
    let field_words_233 = unsafe { [
        read_word(buffer_source_233.wrapping_add(0)),
        read_word(buffer_source_233.wrapping_add(4)),
        read_word(buffer_source_233.wrapping_add(8)),
        read_word(buffer_source_233.wrapping_add(12)),
        read_word(buffer_source_233.wrapping_add(16)),
        read_word(buffer_source_233.wrapping_add(20)),
    ] };
    let field_tail_233 = unsafe { field_words_233.as_ptr().add(4) as usize as u32 };
    let call_result_233 = callee_thiscall!(2, u32, field_tail_233,
        field_words_233[0],
        field_words_233[1],
        field_words_233[2],
        field_words_233[3],
        field_words_233[4],
        field_words_233[5],
    );
    call_results[233] = call_result_233;

    store_frame_word(&mut frame_slots, 0x150, 0u32);
    store_frame_word(&mut frame_slots, 0x14C, 0u32);
    let call_result_234 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x150), 0u32, F32_TWENTY_FIVE_BITS);
    call_results[234] = call_result_234;

    let call_result_235 = callee_thiscall!(27, u32, this_);
    call_results[235] = call_result_235;

    let call_result_236 = callee_thiscall!(30, u32, unsafe { read_word(this_.wrapping_add(0x200)) }, FIELD_KIND_VECTOR, call_results[235], FIELD_KIND_VECTOR);
    call_results[236] = call_result_236;

    let buffer_source_237 = call_results[234];
    let field_words_237 = unsafe { [
        read_word(buffer_source_237.wrapping_add(0)),
        read_word(buffer_source_237.wrapping_add(4)),
        read_word(buffer_source_237.wrapping_add(8)),
        read_word(buffer_source_237.wrapping_add(12)),
        read_word(buffer_source_237.wrapping_add(16)),
        read_word(buffer_source_237.wrapping_add(20)),
    ] };
    let field_tail_237 = unsafe { field_words_237.as_ptr().add(4) as usize as u32 };
    let call_result_237 = callee_thiscall!(2, u32, field_tail_237,
        field_words_237[0],
        field_words_237[1],
        field_words_237[2],
        field_words_237[3],
        field_words_237[4],
        field_words_237[5],
    );
    call_results[237] = call_result_237;

    store_frame_word(&mut frame_slots, 0x154, 0u32);
    store_frame_word(&mut frame_slots, 0x150, 0u32);
    let call_result_238 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x154), 0u32, 0u32);
    call_results[238] = call_result_238;

    let call_result_239 = callee_thiscall!(28, u32, unsafe { read_word(this_.wrapping_add(0x200)) }, FIELD_KIND_FLOAT);
    call_results[239] = call_result_239;

    let buffer_source_240 = call_results[238];
    let field_words_240 = unsafe { [
        read_word(buffer_source_240.wrapping_add(0)),
        read_word(buffer_source_240.wrapping_add(4)),
        read_word(buffer_source_240.wrapping_add(8)),
        read_word(buffer_source_240.wrapping_add(12)),
        read_word(buffer_source_240.wrapping_add(16)),
        read_word(buffer_source_240.wrapping_add(20)),
    ] };
    let field_tail_240 = unsafe { field_words_240.as_ptr().add(4) as usize as u32 };
    let call_result_240 = callee_thiscall!(2, u32, field_tail_240,
        field_words_240[0],
        field_words_240[1],
        field_words_240[2],
        field_words_240[3],
        field_words_240[4],
        field_words_240[5],
    );
    call_results[240] = call_result_240;

    store_frame_word(&mut frame_slots, 0x154, 0u32);
    store_frame_word(&mut frame_slots, 0x150, 0u32);
    let call_result_241 = callee_thiscall!(3, u32, frame_pointer(&mut frame_slots, 0x154), 0u32, 0u32);
    call_results[241] = call_result_241;

    let call_result_242 = callee_thiscall!(28, u32, unsafe { read_word(this_.wrapping_add(0x200)) }, FIELD_KIND_FLAG);
    call_results[242] = call_result_242;

    let buffer_source_243 = call_results[241];
    let field_words_243 = unsafe { [
        read_word(buffer_source_243.wrapping_add(0)),
        read_word(buffer_source_243.wrapping_add(4)),
        read_word(buffer_source_243.wrapping_add(8)),
        read_word(buffer_source_243.wrapping_add(12)),
        read_word(buffer_source_243.wrapping_add(16)),
        read_word(buffer_source_243.wrapping_add(20)),
    ] };
    let field_tail_243 = unsafe { field_words_243.as_ptr().add(4) as usize as u32 };
    let call_result_243 = callee_thiscall!(2, u32, field_tail_243,
        field_words_243[0],
        field_words_243[1],
        field_words_243[2],
        field_words_243[3],
        field_words_243[4],
        field_words_243[5],
    );
    call_results[243] = call_result_243;

    let call_result_244 = callee_thiscall!(11, u32, unsafe { read_word(this_.wrapping_add(0x1F0)) });
    call_results[244] = call_result_244;

    let call_result_245 = callee_thiscall!(6, u32, relocated(IMAGE_REF_01981A4C), relocated(IMAGE_REF_00F007E8));
    call_results[245] = call_result_245;
    unsafe { write_word(this_.wrapping_add(0x1BC), call_results[245].wrapping_add(OBJECT_BYTES_ENTRY)); }

    let call_result_246 = callee_thiscall!(38, u32, this_, 1u32);
    call_results[246] = call_result_246;

    let call_result_247 = callee_thiscall!(20, u32, 0u32);
    call_results[247] = call_result_247;

    call_results[247]
}
