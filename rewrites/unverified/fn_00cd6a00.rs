// original: 0x00CD6A00 CTaskSimpleGetUp::vf5
/// Event gate of the get-up task: decide whether an event finishes the task.
///
/// `this` is the task object (dword state at `+0x14`), `ped` the ped the task
/// runs on (flag word at `+0x26c`), `event_id` a small event kind and `event`
/// a pointer to the event object, null when the event carries nothing.
/// Thiscall with three stack words; returns 1 in `al` when the event is
/// accepted (the task finishes) and 0 when it is ignored.
///
/// Event 2 finishes at once: call the timer setter (callee 0) with -1000.0,
/// set state 3 and flag bit 0 of the ped word. Any id other than 1 or 2 is
/// ignored. Event 1 with a null event finishes with a -8.0 timer instead.
/// Otherwise the event is polled through its own vtable: slot `+8` must
/// report a kind of 0x54 or less, slot `+4` must not report 0x78, slot
/// `+0x18` (called through a register) must answer zero in its low byte, and
/// a second call to slot `+4` must report exactly 9, or the event is ignored.
/// Finally two unsigned millisecond stamps are compared as floats: the clock
/// global minus the event's stamp at `+0x1c` against the timeout callee's
/// answer (callee 4) scaled by the constant 3.0. The original converts both
/// with the unsigned-int path (double widening, table fixup, narrow to
/// float), which rounds exactly once, the same as a direct cast. When the
/// elapsed time is above the scaled timeout the event is ignored, otherwise
/// the task finishes as with a null event.
///
/// The float multiply keeps the original's operand order (converted value
/// times constant) with commuting blocked; the elapsed-above-timeout test is
/// the `comiss` above-test, false for NaN, matching `>` exactly.
lf_checker_rt::export!(thiscall, rw_00CD6A00(this: u32, ped: u32, event_id: u32, event: u32) -> u32 {
    unsafe {
        const STATE_OFF: u32 = 0x14;
        const STATE_FINISHED: u32 = 3;
        const PED_FLAG_OFF: u32 = 0x26c;
        const TIMER_DIRECT: u32 = 0xC47A0000; // -1000.0f
        const TIMER_FINISH: u32 = 0xC1000000; // -8.0f
        const EVENT_DIRECT: u32 = 2;
        const EVENT_POLL: u32 = 1;
        const KIND_LIMIT: u32 = 0x54;
        const KIND_BUSY: u32 = 0x78;
        const KIND_READY: u32 = 9;
        const VT_KIND: u32 = 8;
        const VT_STATE: u32 = 4;
        const VT_GUARD: u32 = 0x18;
        const EVENT_STAMP_OFF: u32 = 0x1c;
        const CLOCK: u32 = 0x011735B4;
        const TIME_SCALE: u32 = 0x00FE8A94; // 3.0f

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        /// Call the event's vtable slot `slot` with the event in ECX, loading
        /// the target through the object exactly like the original so both
        /// sides land on the same planted stub.
        #[inline(always)]
        unsafe fn vcall(slot: u32, obj: u32) -> u32 {
            unsafe {
                let vt = rd32(obj);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt + slot) as usize);
                f(obj)
            }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        if event_id == EVENT_DIRECT {
            lf_checker_rt::callee_thiscall!(0, u32, this, TIMER_DIRECT);
            wr32(this + STATE_OFF, STATE_FINISHED);
            wr32(ped + PED_FLAG_OFF, rd32(ped + PED_FLAG_OFF) | 1);
            return 1;
        }
        if event_id != EVENT_POLL {
            return 0;
        }
        if event != 0 {
            if vcall(VT_KIND, event) > KIND_LIMIT {
                // Falls through to the finish below.
            } else if vcall(VT_STATE, event) == KIND_BUSY {
                // Falls through to the finish below.
            } else if vcall(VT_GUARD, event) & 0xFF != 0 {
                // Falls through to the finish below.
            } else if vcall(VT_STATE, event) != KIND_READY {
                return 0;
            } else {
                let now = (lf_checker_rt::global::<u32>(CLOCK) as *const u32).read_unaligned();
                let elapsed = now.wrapping_sub(rd32(event + EVENT_STAMP_OFF));
                let f1 = elapsed as f32;
                let timeout: u32 = lf_checker_rt::callee_cdecl!(4, u32,);
                let scale = f32::from_bits(
                    (lf_checker_rt::global::<u32>(TIME_SCALE) as *const u32).read_unaligned(),
                );
                if f1 > mul(timeout as f32, scale) {
                    return 0;
                }
            }
        }
        lf_checker_rt::callee_thiscall!(0, u32, this, TIMER_FINISH);
        wr32(this + STATE_OFF, STATE_FINISHED);
        wr32(ped + PED_FLAG_OFF, rd32(ped + PED_FLAG_OFF) | 1);
        1
    }
});
