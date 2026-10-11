// original: 0x00B0CA60 PIGEON_LOOP

/// Process one pool entry using its table index and an output pointer.
///
/// The routine first polls the indexed object's third virtual slot, then
/// follows one of two mode paths: a nonnegative selector may invoke a helper
/// against a global byte table, while a negative selector chooses a branch
/// from the indexed global identifiers. Those paths either create an output
/// object or leave the output null. A nonnull result is initialized, marked
/// from the pool entry's flags, optionally updated from a table float, and
/// passed through the final helper sequence. The output object and the
/// indexed table object are supplied by the proof contract.
///
/// The contract intercepts every outgoing call. The original and rewrite
/// compare the return-independent effects, written heap, stack balance,
/// globals, and declared call data.
lf_checker_rt::export!(thiscall, rw_00B0CA60(this_ptr: u32, out_slot: u32, selector: u32) -> u32 {
    unsafe { pigeon_loop(this_ptr, out_slot, selector, false) }
});

#[inline(always)]
unsafe fn rd8(address: u32) -> u8 {
    unsafe { (address as *const u8).read() }
}

#[inline(always)]
unsafe fn rd16(address: u32) -> u16 {
    unsafe { (address as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn rd32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn global32(address: u32) -> u32 {
    unsafe { rd32(lf_checker_rt::relocated(address)) }
}

#[inline(always)]
unsafe fn global8(address: u32) -> u8 {
    unsafe { rd8(lf_checker_rt::relocated(address)) }
}

#[inline(always)]
unsafe fn indexed_object(index: u32) -> u32 {
    const OBJECT_TABLE: u32 = 0x0129_5CD8;
    let slot = lf_checker_rt::relocated(OBJECT_TABLE).wrapping_add(index.wrapping_mul(4));
    unsafe { rd32(slot) }
}

#[inline(always)]
unsafe fn poll_indexed_object(index: u32) -> u32 {
    const VTABLE_SLOT_THREE: u32 = 0x0C;
    let object = unsafe { indexed_object(index) };
    let vtable = unsafe { rd32(object) };
    let target = unsafe { rd32(vtable.wrapping_add(VTABLE_SLOT_THREE)) };
    let callback: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(target as usize) };
    callback(object)
}

#[inline(always)]
unsafe fn pigeon_loop(this_ptr: u32, out_slot: u32, selector: u32, mutant: bool) -> u32 {
    const GLOBAL_STRUCT: u32 = 0x1632_C60;
    const MODE_ID_A: u32 = 0x12F9_DA4;
    const MODE_ID_B: u32 = 0x12F9_DB0;
    const HELPER_VALUE: u32 = 0x12B4_138;
    const FINAL_ID: u32 = 0x12F9_E94;
    const ENABLED_FLAG: u32 = 0x1615_626;
    const FLOAT_SCALE: u32 = 0x00FE_8AB8;
    const WORD_INDEX: u32 = 0x40;
    const OUT_FLAGS: u32 = 0x210;

    let entry_index = unsafe { rd16(this_ptr.wrapping_add(WORD_INDEX)) as i16 as i32 as u32 };
    unsafe { wr32(out_slot, 0) };

    if (unsafe { poll_indexed_object(entry_index) } as u8) == 4 {
        let indexed = unsafe { indexed_object(entry_index) };
        let context = unsafe { rd32(indexed.wrapping_add(0x60)) };
        let value: u32 = lf_checker_rt::callee_cdecl!(2, u32, context);
        let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, value);
    }

    if (selector as i32) >= 0 {
        let global_struct = unsafe { global32(GLOBAL_STRUCT) };
        let byte_base = unsafe { rd32(global_struct.wrapping_add(4)) };
        let flags = unsafe { rd8(selector.wrapping_add(byte_base)) };
        if flags & 0x80 == 0 {
            let stride = unsafe { rd32(global_struct.wrapping_add(0x0C)) };
            let base = unsafe { rd32(global_struct) };
            let candidate = stride.wrapping_mul(selector).wrapping_add(base);
            if candidate != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, global_struct, candidate);
            }
        }
    }

    if unsafe { rd32(out_slot) } == 0 {
        if (selector as i32) >= 0 {
            let created: u32 = lf_checker_rt::callee_cdecl!(5, u32, entry_index, 6u32, 0u32, 1u32, 1u32, u32::MAX);
            unsafe { wr32(out_slot, created) };
        } else if entry_index != unsafe { global32(MODE_ID_A) }
            && entry_index != unsafe { global32(MODE_ID_B) }
        {
            let created: u32 = lf_checker_rt::callee_cdecl!(5, u32, entry_index, 6u32, 0u32, 1u32, 1u32, u32::MAX);
            unsafe { wr32(out_slot, created) };
        } else {
            let helper_value = unsafe { global32(HELPER_VALUE) };
            let ready: u32 = lf_checker_rt::callee_cdecl!(6, u32, entry_index, helper_value);
            if (ready as u8) == 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(7, u32, entry_index, helper_value, 8u32);
            } else {
                let created: u32 = lf_checker_rt::callee_cdecl!(5, u32, entry_index, 6u32, 1u32, 1u32, 1u32, u32::MAX);
                unsafe { wr32(out_slot, created) };
                if created != 0 {
                    let mut first_frame_word = 0u32;
                    let mut second_frame_word = 0u32;
                    let first_frame = &mut first_frame_word as *mut u32 as u32;
                    let second_frame = &mut second_frame_word as *mut u32 as u32;
                    let _: u32 = lf_checker_rt::callee_thiscall!(8, u32, first_frame);
                    let _: u32 = lf_checker_rt::callee_thiscall!(9, u32, second_frame);
                    let mut output_frame = 0u32;
                    let output_frame_ptr = &mut output_frame as *mut u32 as u32;
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        10,
                        u32,
                        0x0123_1800u32,
                        0x00EA_A704u32,
                        output_frame_ptr,
                        second_frame,
                        u32::MAX,
                        0u32,
                        0u32
                    );
                    if output_frame != 0 {
                        let _: u32 = lf_checker_rt::callee_thiscall!(11, u32, output_frame);
                        let _: u32 = lf_checker_rt::callee_thiscall!(12, u32, output_frame, 0u32, 0u32, 0u32);
                    }
                }
            }
        }
    }

    let output_object = unsafe { rd32(out_slot) };
    if output_object == 0 {
        return 0;
    }

    let mut callback_frame = [0u32; 8];
    let callback_frame_ptr = callback_frame.as_mut_ptr() as u32;
    let _: u32 = lf_checker_rt::callee_thiscall!(
        13,
        u32,
        callback_frame_ptr,
        this_ptr.wrapping_add(0x30)
    );

    let vtable = unsafe { rd32(output_object) };
    let target = unsafe { rd32(vtable.wrapping_add(4)) };
    let finish: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
        unsafe { core::mem::transmute(target as usize) };
    let _ = finish(output_object, 0, 0, callback_frame_ptr);

    unsafe {
        if !mutant {
            let flags = rd32(output_object.wrapping_add(0x118));
            wr32(output_object.wrapping_add(0x118), flags | 0x1000);
        }

        let mut object_flags = rd32(output_object.wrapping_add(OUT_FLAGS)) | 1;
        let entry_flags = rd8(this_ptr.wrapping_add(0x45));
        if entry_flags & 0x40 == 0 {
            object_flags |= 2;
        } else {
            object_flags &= !2;
        }
        wr32(output_object.wrapping_add(OUT_FLAGS), object_flags);
        let mode_word = rd32(output_object.wrapping_add(0x28));
        wr32(output_object.wrapping_add(0x28), mode_word | 0x10);
    }

    let ready: u32 = lf_checker_rt::callee_thiscall!(14, u32, this_ptr);
    unsafe {
        let mut object_flags = rd32(output_object.wrapping_add(OUT_FLAGS));
        if (ready as u8) != 0 {
            object_flags |= 0x0010_0000;
        } else {
            object_flags &= !0x0010_0000;
        }
        wr32(output_object.wrapping_add(OUT_FLAGS), object_flags);
        let clear_word = rd32(output_object.wrapping_add(0x24));
        wr32(output_object.wrapping_add(0x24), clear_word & !1);
    }

    if unsafe { global8(ENABLED_FLAG) } != 0 {
        let indexed = unsafe { indexed_object(entry_index) };
        let value = f32::from_bits(unsafe { rd32(indexed.wrapping_add(0x2C)) });
        let scale = f32::from_bits(unsafe { global32(FLOAT_SCALE) });
        let product = core::hint::black_box(value) * core::hint::black_box(scale);
        unsafe { wr32(output_object.wrapping_add(0x50), product.to_bits()) };

        let second_ready = unsafe { poll_indexed_object(entry_index) };
        if (second_ready as u8) == 4 || entry_index == unsafe { global32(FINAL_ID) } {
            unsafe {
                let object_flags = rd32(output_object.wrapping_add(OUT_FLAGS));
                wr32(output_object.wrapping_add(OUT_FLAGS), object_flags | 8);
            }
        }
    }

    let output_word = unsafe { rd32(output_object.wrapping_add(0x20)) };
    let _: u32 = lf_checker_rt::callee_thiscall!(15, u32, output_object, out_slot);
    let _: u32 = lf_checker_rt::callee_thiscall!(
        16,
        u32,
        this_ptr,
        0u32,
        0x4120_0000u32,
        0u32,
        0x0FA0u32,
        0x3F00_0000u32
    );
    let _: u32 = lf_checker_rt::callee_thiscall!(17, u32, this_ptr, output_object);
    let _: u32 = lf_checker_rt::callee_thiscall!(18, u32, this_ptr, output_word);

    0
}
