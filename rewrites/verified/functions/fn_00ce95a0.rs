// original: 0x00ce95a0 CTaskSimpleNMHighFall::vf26 (symbols)
/// NaturalMotion "high fall" task update: copy the target position into the
/// task, then build and send one behaviour message from per-style tables.
///
/// `this` is the task object: the style index lives at `+0xE0`, and the
/// copied position lands at `+0x30` (one word plus three floats, 16 bytes).
/// `arg` is the owner object: `+0x20` points at the position source (same
/// 16-byte layout), `+0x219` is a flag byte selecting which of two table
/// floats seeds the first message parameter, and `+0x7B4` is the value
/// handed to the send call as its object.
///
/// Behaviour in order: log the style index together with its table entry
/// (cdecl helper); copy the 16-byte position; initialise a stack message
/// buffer; append one integer parameter (value 1); append fourteen float
/// parameters read from the style table at index `style * 64` (the first
/// from one of two neighbouring slots depending on the flag byte, the rest
/// from fixed slots); append three integer parameters from three table
/// bytes; send the buffer; reset the buffer. The message helpers take the
/// buffer in ECX and `(name, value)` on the stack; the send call takes its
/// object in ECX and `(name, buffer)` on the stack. No arithmetic is done
/// on the table values: every float is only moved, so any bit pattern
/// (including any NaN) passes through unchanged.
///
/// Original: 0x00CE95A0 (thiscall, one stack word, callee pops 4).
lf_checker_rt::export!(thiscall, rw_00ce95a0(this: u32, arg: u32) -> u32 {
    unsafe {
        const STYLE_OFF: u32 = 0xE0;
        const POS_OFF: u32 = 0x30;
        const ARG_POS: u32 = 0x20;
        const ARG_FLAG: u32 = 0x219;
        const ARG_NM: u32 = 0x7B4;
        const ENTRY_STRIDE: u32 = 64;
        const MSG_BUF_LEN: usize = 0xCC4;
        const STYLE_TABLE: u32 = 0x0171D000;
        const STYLE_NAMES: u32 = 0x01052218;
        const LOG_TAG: u32 = 0x00EDCE10;
        const MSG_OPEN: u32 = 0x01051CC8;
        const MSG_SEND_NAME: u32 = 0x01052098;
        /// (table slot, name global) for the thirteen fixed float parameters
        /// (a fourteenth, branch-selected one is emitted separately above).
        const FLOAT_PARAMS: [(u32, u32); 13] = [
            (0x30, 0x010520A0), (0x34, 0x010520A4), (0x3C, 0x010520B4),
            (0x38, 0x010520B0), (0x40, 0x010520B8), (0x44, 0x010520BC),
            (0x48, 0x010520C0), (0x4C, 0x010520C4), (0x50, 0x010520C8),
            (0x54, 0x010520D0), (0x58, 0x010520D4), (0x5C, 0x010520D8),
            (0x60, 0x010520DC),
        ];
        /// (table slot, name global) for the three trailing int parameters.
        const INT_PARAMS: [(u32, u32); 3] = [
            (0x64, 0x010520CC), (0x65, 0x010520E0), (0x66, 0x010520E4),
        ];
        const FIRST_NAME: u32 = 0x010520A8;
        const FIRST_SLOT_SET: u32 = 0x28;
        const FIRST_SLOT_CLEAR: u32 = 0x2C;
        const CAL_INIT: u32 = 1;
        const CAL_ADD_INT: u32 = 2;
        const CAL_ADD_FLOAT: u32 = 3;
        const CAL_SEND: u32 = 5;
        const CAL_RESET: u32 = 6;
        const CAL_LOG: u32 = 7;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn g32(file_va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(file_va).read() }
        }

        let style = rd32(this + STYLE_OFF);
        let style_name = g32(STYLE_NAMES + style.wrapping_mul(4));
        lf_checker_rt::callee_cdecl!(CAL_LOG, u32, lf_checker_rt::relocated(LOG_TAG), style, style_name);

        let pos = rd32(arg + ARG_POS);
        wr32(this + POS_OFF, rd32(pos + POS_OFF));
        wr32(this + POS_OFF + 4, rd32(pos + POS_OFF + 4));
        wr32(this + POS_OFF + 8, rd32(pos + POS_OFF + 8));
        wr32(this + POS_OFF + 12, rd32(pos + POS_OFF + 12));

        let mut buf = [0u8; MSG_BUF_LEN];
        let bp = buf.as_mut_ptr() as u32;
        lf_checker_rt::callee_thiscall!(CAL_INIT, u32, bp);
        lf_checker_rt::callee_thiscall!(CAL_ADD_INT, u32, bp, g32(MSG_OPEN), 1);

        let entry = STYLE_TABLE + style.wrapping_mul(ENTRY_STRIDE);
        let flag = ((arg + ARG_FLAG) as *const u8).read();
        let first_slot = if flag == 0 { FIRST_SLOT_CLEAR } else { FIRST_SLOT_SET };
        lf_checker_rt::callee_thiscall!(
            CAL_ADD_FLOAT, u32, bp, g32(FIRST_NAME),
            g32(entry + first_slot),
        );
        // The fixed parameters interleave: nine floats, one int, four
        // floats, two ints. The call order is part of the behaviour.
        let mut i = 0;
        while i < 9 {
            let (slot, name) = FLOAT_PARAMS[i];
            lf_checker_rt::callee_thiscall!(CAL_ADD_FLOAT, u32, bp, g32(name), g32(entry + slot));
            i += 1;
        }
        let mut j = 0;
        while j < 3 {
            if j == 1 {
                while i < 13 {
                    let (slot, name) = FLOAT_PARAMS[i];
                    lf_checker_rt::callee_thiscall!(CAL_ADD_FLOAT, u32, bp, g32(name), g32(entry + slot));
                    i += 1;
                }
            }
            let (slot, name) = INT_PARAMS[j];
            let b = (lf_checker_rt::relocated(STYLE_TABLE) + style.wrapping_mul(ENTRY_STRIDE) + slot) as *const u8;
            lf_checker_rt::callee_thiscall!(CAL_ADD_INT, u32, bp, g32(name), b.read() as u32);
            j += 1;
        }

        let nm = rd32(arg + ARG_NM);
        lf_checker_rt::callee_thiscall!(CAL_SEND, u32, nm, g32(MSG_SEND_NAME), bp);
        lf_checker_rt::callee_thiscall!(CAL_RESET, u32, bp);
        0
    }
});
