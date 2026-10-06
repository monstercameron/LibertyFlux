// original: 0x008e5c80 menu_music_state_machine
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

#[inline(always)]
unsafe fn rd_u32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read() }
}

#[inline(always)]
unsafe fn rd_u8(addr: u32) -> u8 {
    unsafe { (addr as *const u8).read() }
}

#[inline(always)]
unsafe fn wr_u32(addr: u32, val: u32) {
    unsafe { (addr as *mut u32).write(val) }
}

#[inline(always)]
unsafe fn image_f32(file_va: u32) -> f32 {
    unsafe { global::<f32>(file_va).read() }
}

/// Chop-truncate a float to a 64-bit integer exactly like the original's
/// control-word dance does, returning the low 32 bits for the state slot.
/// Out-of-range values (including infinities and not-a-numbers) yield the
/// indefinite integer, whose low half is zero.
fn chop_low(value: f32) -> u32 {
    const LIMIT: f32 = 9223372036854775808.0;
    if value.is_nan() || value >= LIMIT || value < -LIMIT {
        0
    } else {
        (value as i64) as u32
    }
}

/// Advance the dual music/ambience state machine by one tick (stage 1a).
///
/// `handle` owns state word A at `+0x204` and state word B at `+0x200`.
/// In case 0 of either switch the machine queries its engine object,
/// acquires the shared voice handle on demand, and fans a stop call out to
/// the two registered channel handles. Between the switches the middle gate
/// scales a global rate by a fixed factor, truncates it toward zero, and
/// keeps the low word in `+0x214`. Returns the case-0 query answer, or the
/// state word on the invalid-state paths.
export!(
    thiscall,
    rw_008e5c80(handle: u32) -> u32 {
        unsafe {
            callee_thiscall!(1, u32, handle);
            let state_a = rd_u32(handle.wrapping_add(0x204));
            if state_a == 0 {
                // The stub answers in AL with scratch in the upper bytes;
                // only the low byte gates this branch (EAX is re-threaded
                // by later calls on every path through this stage).
                let ready: u32 = callee_thiscall!(2, u32, handle);
                if ready as u8 != 0 {
                    wr_u32(handle.wrapping_add(0x220), 0);
                    wr_u32(handle.wrapping_add(0x224), 0);
                    wr_u32(handle.wrapping_add(0x204), 1);
                    if rd_u32(handle.wrapping_add(0x218)) == 0 {
                        // The request block is laid down after the address
                        // push, so every slot sits one word lower than the
                        // encoding suggests; the two code addresses carry
                        // the image relocation delta.
                        let request = [
                            relocated(0x008E4480),
                            relocated(0x008E4050),
                            0,
                            5,
                            0,
                        ];
                        let obj: u32 = callee_cdecl!(3, u32, request.as_ptr() as u32);
                        wr_u32(handle.wrapping_add(0x218), obj);
                    }
                    let live = rd_u32(handle.wrapping_add(0x218));
                    if live != 0 && rd_u8(live.wrapping_add(0x28)) == 1 {
                        wr_u32(handle.wrapping_add(0x204), 2);
                    }
                    let first = rd_u32(handle.wrapping_add(0x34C));
                    if first != 0 {
                        callee_thiscall!(4, u32, first, 0);
                    }
                    let second = rd_u32(handle.wrapping_add(0x228));
                    if second != 0 {
                        callee_thiscall!(4, u32, second, 0);
                        wr_u32(handle.wrapping_add(0x234), 0);
                    }
                }
            } else if state_a <= 4 {
                panic!("fn 5c80 s1: state-A case {}", state_a);
            }
            let tick = image_f32(0x0115DBE8) * image_f32(0x00FE8C58);
            let chopped = chop_low(tick);
            if rd_u8(handle.wrapping_add(0x43D)) == 0 {
                panic!("fn 5c80 s1: table section");
            }
            let state_b = rd_u32(handle.wrapping_add(0x200));
            let answer: u32;
            if state_b == 0 {
                // Full EAX is threaded to the return: the stub's upper
                // scratch bytes are identical on both sides for the same
                // call sequence, so the whole word is compared.
                let live_b: u32 = callee_thiscall!(5, u32, handle);
                if live_b as u8 == 0 {
                    answer = live_b;
                } else if rd_u8(handle.wrapping_add(0x43D)) == 0 {
                    answer = live_b;
                } else {
                    wr_u32(handle.wrapping_add(0x200), 1);
                    answer = live_b;
                }
            } else if state_b <= 4 {
                panic!("fn 5c80 s1: state-B case {}", state_b);
            } else {
                answer = state_b;
            }
            wr_u32(handle.wrapping_add(0x214), chopped);
            // Stack-cookie check: the original passes cookie^ESP in ECX,
            // which a rewrite cannot reproduce (different frame), so the
            // contract excludes ECX from the comparison; the stub preserves
            // the registers and only the call itself is observed.
            let _: u32 = callee_thiscall!(6, u32, 0);
            answer
        }
    }
);
