// original: 0x006A5AE0 input_window_proc

// File VAs of the globals and import slots this window procedure uses.
// The helpers below read and write them through lf_checker_rt::global.
const HOOK_SLOT: u32 = 0x018B_8038; // stdcall hook pointer, called when non-null
const IAT_GET_CLIENT_RECT: u32 = 0x00E7_33F4;
const IAT_IS_WINDOW_UNICODE: u32 = 0x00E7_336C;
const IAT_DEF_WINDOW_PROC_W: u32 = 0x00E7_3374;
const IAT_DEF_WINDOW_PROC_A: u32 = 0x00E7_3390;
const IAT_SEND_MESSAGE_A: u32 = 0x00E7_3428;
const IAT_RELEASE_CAPTURE: u32 = 0x00E7_3370;
const DISPATCH_GATE: u32 = 0x018B_7DAC; // dword: key-array writes happen while it is zero
const KEY_ARRAY: u32 = 0x018B_7C90; // one byte per scan code (256 bytes)
const QUEUE_ENABLED: u32 = 0x018B_7DA4; // byte: the character queue is in use
const QUEUE_COUNT: u32 = 0x018B_7DA8; // dword, signed: entries in the queue
const QUEUE_BYTES: u32 = 0x018B_7D90; // queue entries, one byte each
const SHIFT_HELD: u32 = 0x018B_7A7B; // byte: set by the shift scan codes
const MOUSE_GATE: u32 = 0x018B_7A5E; // byte: when non-zero the mouse position is not rescaled
const BUTTON_BITS: u32 = 0x018B_7A74; // dword: one bit per button and extra button
const WHEEL_ACCUM: u32 = 0x018B_7A7C; // dword: accumulated wheel steps
const MOUSE_X: u32 = 0x018B_7A80; // dword: rescaled x position
const MOUSE_Y: u32 = 0x018B_7A8C; // dword: rescaled y position
const SCALE_X: u32 = 0x017A_CCEC; // dword: x scale factor
const SCALE_Y: u32 = 0x017A_CCDC; // dword: y scale factor
const CAPTURE_COUNT: u32 = 0x01BB_394C; // dword: outstanding button captures
const SUB_COUNT: u32 = 0x01BB_3948; // dword, signed: subscriber rows
const SUB_MSG: u32 = 0x01BB_3950; // dword per subscriber row: message it takes
const SUB_OBJ: u32 = 0x01BB_3960; // dword per subscriber row: subscriber object

// Callee ids of the checker contract (direct patches, import slots and the subscriber slot).
const ID_ACTIVATE: u32 = 2;
const ID_SCAN: u32 = 3;
const ID_CAPTURE: u32 = 4;

#[inline(always)]
unsafe fn rd32(va: u32) -> u32 {
    *lf_checker_rt::global::<u32>(va)
}

#[inline(always)]
unsafe fn wr32(va: u32, v: u32) {
    *lf_checker_rt::global::<u32>(va) = v;
}

#[inline(always)]
unsafe fn rd8(va: u32) -> u8 {
    *lf_checker_rt::global::<u8>(va)
}

#[inline(always)]
unsafe fn wr8(va: u32, v: u8) {
    *lf_checker_rt::global::<u8>(va) = v;
}

// Calls through an import slot: the slot holds the address the worker planted.
#[inline(always)]
unsafe fn iat_call_0(slot: u32) -> u32 {
    let f: extern "stdcall" fn() -> u32 = core::mem::transmute(rd32(slot) as usize);
    f()
}

#[inline(always)]
unsafe fn iat_call_1(slot: u32, a: u32) -> u32 {
    let f: extern "stdcall" fn(u32) -> u32 = core::mem::transmute(rd32(slot) as usize);
    f(a)
}

#[inline(always)]
unsafe fn iat_call_2(slot: u32, a: u32, b: u32) -> u32 {
    let f: extern "stdcall" fn(u32, u32) -> u32 = core::mem::transmute(rd32(slot) as usize);
    f(a, b)
}

#[inline(always)]
unsafe fn iat_call_4(slot: u32, a: u32, b: u32, c: u32, d: u32) -> u32 {
    let f: extern "stdcall" fn(u32, u32, u32, u32) -> u32 =
        core::mem::transmute(rd32(slot) as usize);
    f(a, b, c, d)
}

// Direct callee `id` with the cdecl convention (two arguments, caller cleans).
#[inline(always)]
unsafe fn callee_cdecl_2(id: u32, a: u32, b: u32) -> u32 {
    let f: extern "cdecl" fn(u32, u32) -> u32 =
        core::mem::transmute(lf_checker_rt::callee_addr(id) as usize);
    f(a, b)
}

// Direct callee `id` with the thiscall convention and no stack arguments.
#[inline(always)]
unsafe fn callee_thiscall_0(id: u32, this: u32) -> u32 {
    let f: extern "thiscall" fn(u32) -> u32 =
        core::mem::transmute(lf_checker_rt::callee_addr(id) as usize);
    f(this)
}

// Default path: the window is asked whether it is Unicode, then the message goes to
// DefWindowProcW or DefWindowProcA with the same four arguments.
unsafe fn forward(hwnd: u32, msg: u32, wparam: u32, lparam: u32) -> u32 {
    if iat_call_1(IAT_IS_WINDOW_UNICODE, hwnd) == 0 {
        iat_call_4(IAT_DEF_WINDOW_PROC_A, hwnd, msg, wparam, lparam)
    } else {
        iat_call_4(IAT_DEF_WINDOW_PROC_W, hwnd, msg, wparam, lparam)
    }
}

// Scan code of a key message: the extended-key bit (24) routes lparam through the
// helper, otherwise the scan code is bits 16..23 of lparam.
unsafe fn scan_code(lparam: u32) -> u32 {
    if lparam & 0x0100_0000 != 0 {
        callee_thiscall_0(ID_SCAN, lparam)
    } else {
        (lparam >> 16) & 0xFF
    }
}

// Key down: the key array takes 0x80 while the dispatch gate is zero, the character
// queue takes the scan code while it is enabled and has room (count below 16, signed),
// and the shift byte is set for scan codes 0x2A and 0x36.
unsafe fn key_down(scan: u32) {
    if rd32(DISPATCH_GATE) == 0 {
        wr8(KEY_ARRAY.wrapping_add(scan), 0x80);
    }
    if rd8(QUEUE_ENABLED) != 0 {
        let count = rd32(QUEUE_COUNT) as i32;
        if count < 0x10 {
            wr8(QUEUE_BYTES.wrapping_add(count as u32), scan as u8);
            let next = count.wrapping_add(1);
            wr32(QUEUE_COUNT, next as u32);
            wr8(QUEUE_BYTES.wrapping_add(next as u32), 0);
        }
    }
    if scan == 0x2A || scan == 0x36 {
        wr8(SHIFT_HELD, 1);
    }
}

// Key up: the key array byte is cleared while the dispatch gate is zero, and the shift
// byte is cleared for scan codes 0x2A and 0x36. The queue is not touched.
unsafe fn key_up(scan: u32) {
    if rd32(DISPATCH_GATE) == 0 {
        wr8(KEY_ARRAY.wrapping_add(scan), 0);
    }
    if scan == 0x2A || scan == 0x36 {
        wr8(SHIFT_HELD, 0);
    }
}

// Mouse rescale: x is the signed low word of lparam times the x scale, divided by the
// client width; y is the signed high word times the y scale, divided by the client
// height. Both divisors come from the client rectangle, which the caller keeps nonzero.
unsafe fn rescale_mouse(lparam: u32, rect: &[u32; 4]) {
    let x = (lparam as u16 as i16 as i32).wrapping_mul(rd32(SCALE_X) as i32);
    wr32(MOUSE_X, (x / (rect[2] as i32)) as u32);
    let y = ((lparam >> 16) as u16 as i16 as i32).wrapping_mul(rd32(SCALE_Y) as i32);
    wr32(MOUSE_Y, (y / (rect[3] as i32)) as u32);
}

// Rescale unless the mouse gate byte is set.
unsafe fn rescale_mouse_if_free(lparam: u32, rect: &[u32; 4]) {
    if rd8(MOUSE_GATE) == 0 {
        rescale_mouse(lparam, rect);
    }
}

// Decrement the capture count and report whether it reached zero.
unsafe fn dec_capture_count() -> bool {
    let next = rd32(CAPTURE_COUNT).wrapping_sub(1);
    wr32(CAPTURE_COUNT, next);
    next == 0
}

// Wheel: the signed wheel delta (high word of wparam) is divided by 120 with the
// multiply-high sequence the original uses (magic 0x88888889, signed; add the delta
// back; arithmetic shift by 6; add the sign bit), and the result is added to the
// accumulator.
unsafe fn wheel_accumulate(wparam: u32) {
    let delta = (wparam >> 16) as u16 as i16 as i32;
    let product = (0x8888_8889u32 as i32 as i64).wrapping_mul(delta as i64);
    let mut q = ((product >> 32) as i32).wrapping_add(delta);
    q >>= 6;
    let steps = q.wrapping_add(((q as u32) >> 31) as i32);
    wr32(WHEEL_ACCUM, rd32(WHEEL_ACCUM).wrapping_add(steps as u32));
}

// Subscriber loop: every row whose message equals this one is called through the
// object's vtable slot 1 (thiscall, four arguments). The row count is read again
// after each row, and the default forward follows the loop.
unsafe fn dispatch_subscribers(hwnd: u32, msg: u32, wparam: u32, lparam: u32) -> u32 {
    let mut i: u32 = 0;
    while (i as i32) < (rd32(SUB_COUNT) as i32) {
        if rd32(SUB_MSG.wrapping_add(i.wrapping_mul(4))) == msg {
            let obj = rd32(SUB_OBJ.wrapping_add(i.wrapping_mul(4)));
            let vtable = *(obj as *const u32);
            let slot = *(vtable.wrapping_add(4) as *const u32);
            let f: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            f(obj, hwnd, msg, wparam, lparam);
        }
        i = i.wrapping_add(1);
    }
    forward(hwnd, msg, wparam, lparam)
}

// Button press: set the bit, rescale unless the gate is set, then capture.
unsafe fn button_down(bit: u32, hwnd: u32, lparam: u32, rect: &[u32; 4]) -> u32 {
    wr32(BUTTON_BITS, rd32(BUTTON_BITS) | bit);
    rescale_mouse_if_free(lparam, rect);
    callee_thiscall_0(ID_CAPTURE, hwnd);
    0
}

// Button release: clear the bit, rescale unless the gate is set, then release the
// capture when the count reaches zero.
unsafe fn button_up(bit: u32, lparam: u32, rect: &[u32; 4]) -> u32 {
    wr32(BUTTON_BITS, rd32(BUTTON_BITS) & !bit);
    rescale_mouse_if_free(lparam, rect);
    if dec_capture_count() {
        iat_call_0(IAT_RELEASE_CAPTURE);
    }
    0
}

/// Input window procedure (stdcall, four arguments: window, message, wparam, lparam;
/// the original pops sixteen bytes, and the result is returned whole).
///
/// Every call first gives a non-null hook the message with an out flag held in the
/// lparam slot; a nonzero flag returns the hook's answer at once. Otherwise the client
/// rectangle is fetched and the message is dispatched: activation (calls the activation
/// helper when wparam is nonzero, then the default path); key down and up (scan code from
/// lparam, or from the extended-key helper; key array, character queue and shift state);
/// system key down (wparam 0x73 posts a close first, wparam 0x10 takes the default path);
/// mouse move and the buttons (position rescaled by the client size unless the mouse gate
/// is set; capture counted and released at zero); the wheel (delta divided by 120 with the
/// original's multiply-high sequence, accumulated); extra buttons 1 and 2; reset (0x215);
/// and the forward with lparam zero (0x281). Every other message, and any message in the
/// default range, goes to the subscriber rows (called while the row count is positive) and
/// then to the default path: Unicode test, then DefWindowProcW or DefWindowProcA.
lf_checker_rt::export!(stdcall, rw_fn1(hwnd: u32, msg: u32, wparam: u32, lparam: u32) -> u32 {
    unsafe {
        // The original keeps a word in its own incoming lparam slot: that word holds lparam
        // until a non-null hook is called. The hook is then given the slot as its out flag
        // (cleared first); a nonzero flag ends the call with the hook's answer. Otherwise
        // the slot holds the flag (zero) and is reused by the activation helper below.
        let mut slot: u32 = lparam;
        let hook = rd32(HOOK_SLOT);
        if hook != 0 {
            slot = 0;
            let f: extern "stdcall" fn(u32, u32, u32, u32, *mut u32) -> u32 =
                core::mem::transmute(hook as usize);
            let answer = f(hwnd, msg, wparam, lparam, &mut slot as *mut u32);
            if slot != 0 {
                return answer;
            }
        }

        // The client rectangle is fetched for every message that reaches this point.
        let mut rect: [u32; 4] = [0; 4];
        iat_call_2(IAT_GET_CLIENT_RECT, hwnd, rect.as_mut_ptr() as u32);

        match msg {
            0x1C => {
                // Activate: a nonzero wparam calls the activation helper with a pointer to
                // the lparam slot and a zero; the message then goes to the default path.
                if wparam != 0 {
                    callee_cdecl_2(ID_ACTIVATE, &mut slot as *mut u32 as u32, 0);
                }
                forward(hwnd, msg, wparam, lparam)
            }
            0x100 => {
                key_down(scan_code(lparam));
                0
            }
            0x101 | 0x105 => {
                key_up(scan_code(lparam));
                0
            }
            0x102 => 0,
            0x104 => {
                // System key down: wparam 0x73 posts a close first; wparam 0x10 goes to the
                // default path; everything else is a key down.
                if wparam == 0x73 {
                    iat_call_4(IAT_SEND_MESSAGE_A, hwnd, 0x10, 0, 0);
                } else if wparam == 0x10 {
                    return forward(hwnd, msg, wparam, lparam);
                }
                key_down(scan_code(lparam));
                0
            }
            0x200 => {
                rescale_mouse_if_free(lparam, &rect);
                0
            }
            0x201 => button_down(0x1, hwnd, lparam, &rect),
            0x202 => button_up(0x1, lparam, &rect),
            0x204 => button_down(0x2, hwnd, lparam, &rect),
            0x205 => button_up(0x2, lparam, &rect),
            0x207 => button_down(0x4, hwnd, lparam, &rect),
            0x208 => button_up(0x4, lparam, &rect),
            0x20A => {
                wheel_accumulate(wparam);
                0
            }
            0x20B => {
                // Extra button down: buttons 1 and 2 set their bit and rescale; any other
                // button only captures.
                let xbutton = (wparam >> 16) & 0xFFFF;
                if xbutton == 1 {
                    wr32(BUTTON_BITS, rd32(BUTTON_BITS) | 0x8);
                } else if xbutton == 2 {
                    wr32(BUTTON_BITS, rd32(BUTTON_BITS) | 0x10);
                }
                if xbutton == 1 || xbutton == 2 {
                    rescale_mouse_if_free(lparam, &rect);
                }
                callee_thiscall_0(ID_CAPTURE, hwnd);
                1
            }
            0x20C => {
                // Extra button up: buttons 1 and 2 clear their bit and rescale; the
                // capture count is decremented for every button.
                let xbutton = (wparam >> 16) & 0xFFFF;
                if xbutton == 1 {
                    wr32(BUTTON_BITS, rd32(BUTTON_BITS) & !0x8);
                } else if xbutton == 2 {
                    wr32(BUTTON_BITS, rd32(BUTTON_BITS) & !0x10);
                }
                if xbutton == 1 || xbutton == 2 {
                    rescale_mouse_if_free(lparam, &rect);
                }
                if dec_capture_count() {
                    iat_call_0(IAT_RELEASE_CAPTURE);
                }
                1
            }
            0x215 => {
                wr32(BUTTON_BITS, 0);
                wr32(CAPTURE_COUNT, 0);
                0
            }
            0x281 => forward(hwnd, msg, wparam, 0),
            _ => dispatch_subscribers(hwnd, msg, wparam, lparam),
        }
    }
});
