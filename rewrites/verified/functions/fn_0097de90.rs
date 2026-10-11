// original: 0x0097DE90 PED_RINGTONE

/// Applies the initialization gates, updates an existing voice slot or
/// builds a request for a newly resolved ring-tone object, and writes the
/// resulting value to the tone's output word.
///
/// The proof varies the epoch and mode exits, the existing-slot skip, a null
/// setup result, and the live-object request path. It also varies the
/// incoming tail word across positive and nonpositive values. The first
/// gate's entry-EAX exit stays closed because a rewrite cannot read that
/// input. Stack comparison is disabled for the function's scratch frame and
/// caller-slot effects. Frame pointers passed to the initializer, frame
/// constructor, and three processing calls are not compared directly; the
/// setup block, request arguments, and processing slice are compared through
/// snapshots. The cookie check call is captured, but its frame-derived ECX
/// value is not compared. Callee internals are stubbed; their call records
/// and scripted outputs are checked.
const GATE_INIT: u32 = 0x11F7060;
const GATE_EPOCH: u32 = 0x12088B4;
const GATE_EPOCH_COPY: u32 = 0xF1C040;
const GATE_MODE: u32 = 0x1037720;
const COOKIE_WORD: u32 = 0x1057FB4;
const TAIL_ADDEND: u32 = 0x11735B4;
const GATE_MODE_EXIT: u32 = 0x12;
const OUTPUT_SLOT: u32 = 0x130;
const OUTPUT_VALUE: u32 = 0x138;
const THIS_TAG: u32 = 0x08;
const THIS_SOURCE: u32 = 0x120;
const THIS_LEVEL: u32 = 0x12C;
const BLOCK_PRESET: u32 = 0x780;
const CALLBACK_NAME: u32 = 0xE8CBAC;
const SETUP_NAME: u32 = 0xE8CB8C;
const CREATE_NAME: u32 = 0xE8CBB8;
const CALL_BLOCK_INIT: u32 = 1;
const CALL_SETUP: u32 = 2;
const CALL_CALLBACK: u32 = 3;
const CALL_OBJECT_UPDATE: u32 = 4;
const CALL_FRAME_INIT: u32 = 5;
const CALL_CREATE: u32 = 6;
const CALL_RESOLVE: u32 = 7;
const CALL_POSTPROCESS: u32 = 8;
const CALL_KEY: u32 = 9;
const CALL_FINISH: u32 = 10;
const CALL_COOKIE_CHECK: u32 = 11;

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn write_u8(address: u32, value: u8) {
    unsafe { (address as *mut u8).write(value) }
}

unsafe fn cookie_return(value: u32) -> u32 {
    let cookie = unsafe { lf_checker_rt::global::<u32>(COOKIE_WORD).read() };
    let _ = lf_checker_rt::callee_thiscall!(CALL_COOKIE_CHECK, u32, cookie);
    value
}

pub(crate) unsafe fn rewrite(this: u32, stack_word: u32, suppress_tail_store: bool) -> u32 {
    if unsafe { lf_checker_rt::global::<u32>(GATE_INIT).read() } == 1 {
        // This exit returns the caller's entry EAX, which the rewrite cannot read.
        return unsafe { cookie_return(0) };
    }

    let epoch = unsafe { lf_checker_rt::global::<u32>(GATE_EPOCH).read() };
    let epoch_copy = unsafe { lf_checker_rt::global::<u32>(GATE_EPOCH_COPY).read() };
    let mode = unsafe { lf_checker_rt::global::<u32>(GATE_MODE).read() };
    if epoch != epoch_copy || mode == GATE_MODE_EXIT {
        return unsafe { cookie_return(epoch) };
    }

    let output_slot = this.wrapping_add(OUTPUT_SLOT);
    let mut result = epoch;
    if unsafe { read_u32(output_slot) } == 0 {
        let mut request_block = [0u32; 32];
        let block = request_block.as_mut_ptr() as u32;
        let _ = lf_checker_rt::callee_thiscall!(CALL_BLOCK_INIT, u32, block);
        unsafe {
            write_u32(
                block.wrapping_add(3 * 4),
                read_u32(this.wrapping_add(THIS_SOURCE)).wrapping_add(BLOCK_PRESET),
            );
            write_u32(
                block.wrapping_add(8 * 4),
                read_u32(this.wrapping_add(THIS_TAG)),
            );
        }

        let setup_answer = lf_checker_rt::callee_thiscall!(
            CALL_SETUP,
            u32,
            this,
            lf_checker_rt::relocated(SETUP_NAME),
            output_slot,
            block,
            u32::MAX,
            0,
            0
        );
        let object = unsafe { read_u32(output_slot) };
        if object == 0 {
            result = setup_answer;
        } else {
            let level_bits = unsafe { read_u32(this.wrapping_add(THIS_LEVEL)) };
            let _ = lf_checker_rt::callee_thiscall!(
                CALL_CALLBACK,
                u32,
                object,
                lf_checker_rt::relocated(CALLBACK_NAME),
                level_bits
            );
            let _ = lf_checker_rt::callee_thiscall!(CALL_OBJECT_UPDATE, u32, object, 0);

            let mut frame_context = [0u32; 16];
            let frame_context_ptr = frame_context.as_mut_ptr() as u32;
            let mut parameter_words = [0u32, u32::MAX, 0x55];
            let parameters = parameter_words.as_mut_ptr() as u32;
            let _ = lf_checker_rt::callee_thiscall!(
                CALL_FRAME_INIT,
                u32,
                frame_context_ptr,
                0x55
            );

            unsafe {
                write_u32(block.wrapping_add(0x6C), level_bits);
                write_u8(block.wrapping_add(0x76), 2);
            }
            let created = lf_checker_rt::callee_cdecl!(
                CALL_CREATE,
                u32,
                lf_checker_rt::relocated(CREATE_NAME),
                0,
                0,
                1,
                0,
                block,
                parameters,
                unsafe { read_u32(this.wrapping_add(THIS_SOURCE)) },
                u32::MAX
            );
            let entity = lf_checker_rt::callee_cdecl!(CALL_RESOLVE, u32, created);
            let _ = lf_checker_rt::callee_cdecl!(
                CALL_POSTPROCESS,
                u32,
                entity,
                block.wrapping_add(0x48),
                unsafe { read_u32(this.wrapping_add(THIS_SOURCE)) }
            );
            let key = lf_checker_rt::callee_cdecl!(CALL_KEY, u32, entity);

            unsafe {
                write_u32(object.wrapping_add(0xA4), entity);
                write_u32(object.wrapping_add(0xA8), key);
                write_u32(object.wrapping_add(0xAC), 0);
            }
            result = lf_checker_rt::callee_thiscall!(
                CALL_FINISH,
                u32,
                object,
                0,
                0,
                0
            );
        }
    }

    if (stack_word as i32) > 0 {
        let addend = unsafe { lf_checker_rt::global::<u32>(TAIL_ADDEND).read() };
        let tail_value = addend.wrapping_add(stack_word);
        if !suppress_tail_store {
            unsafe { write_u32(this.wrapping_add(OUTPUT_VALUE), tail_value) };
        }
        result = tail_value;
    } else if !suppress_tail_store {
        unsafe { write_u32(this.wrapping_add(OUTPUT_VALUE), 0) };
    }

    unsafe { cookie_return(result) }
}

lf_checker_rt::export!(thiscall, rw_0097de90(this: u32, stack_word: u32) -> u32 {
    unsafe { rewrite(this, stack_word, false) }
});
