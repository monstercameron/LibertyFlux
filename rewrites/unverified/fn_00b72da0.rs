// original: 0x00B72DA0 CTaskSimpleCreateCarAndGetIn::vf17

/// Create-car-and-get-in task step: probe a spawn query, refresh the
/// creation timer or place the car, then build and start the car task.
///
/// `this` is the task object: words at `+0x20` (address handed to the spawn
/// query), `+0x30` (an identifier), a byte at `+0x34`, a car handle at `+0x38`
/// (may be null), four floats at `+0x40`, timer words at `+0x50`/`+0x54` and
/// bytes at `+0x58`/`+0x59`. `ped` is the ped object: a pointer at `+0x20`
/// (three floats at `+0x30`/`+0x34`/`+0x38`) and a signed word index at
/// `+0x2e`. Game data used: a service object at file address `0x01177A80`, a
/// timer seed at `0x011735B4`, a self-indexed table at `0x0118D818`, a
/// pointer table at `0x01295CD8`, and words at `0x012B4138`, `0x012E22A4`
/// and `0x012BD0C4`.
///
/// Steps: ask callee 1 (thiscall on the service object: an answer buffer,
/// the task's `+0x20` address, `0x497423FE`, 1, 0, 0, 0, 0, 0) for a spawn
/// probe, then callee 2 (thiscall on the service object: buffer word 2's
/// address, buffer word 1, the flag byte's address) to settle it. When the
/// flag byte is clear, run the timer path: when `+0x58` is clear, store the
/// seed at `+0x50`, `0x7D0` at `+0x54` and set `+0x58` (the re-test of `+0x58`
/// just after is dead but kept); when `+0x59` is set, re-store the seed and
/// clear it; return 1 when `+0x54` plus `+0x50` (wrapping) does not exceed
/// the seed (signed), else 0. When the flag is set, clear `+0x58` and copy
/// buffer words 2 to 5 to the `+0x40` floats. When no car handle exists yet,
/// validate twice through callee 3 and callee 4 (thiscall, five arguments:
/// a table entry plus `0x10`, four float words, 0), returning 1 on either
/// nonzero answer, then ask callee 5 (cdecl, eight arguments) to commit,
/// returning 1 on a zero low byte. Then construct a car-task scratch object
/// through callee 6 (thiscall: 0, 0, 0, `8.0f`), attach the ped through
/// callee 7 (thiscall: the ped) and check the identifier through callee 8
/// (cdecl: the identifier, the `0x012B4138` word): on a zero low byte retry
/// through callee 9 (cdecl: the identifier, the word, `0x0c`) and finish
/// through the finalizer below with preset 0. Otherwise, when no car handle
/// exists, create it through callee 10 (cdecl: the identifier, the task's
/// `+0x40` address, 0, 1), store it at `+0x38`, and run the scratch
/// construct/use/teardown trio (callees 11, 12, 13) on a second buffer.
/// Then poll the handle through callee 14 (thiscall, no arguments): on a
/// nonzero low byte finish with preset 0, otherwise release through callee
/// 15 (cdecl: the handle), reconfigure through callee 16 (thiscall on the
/// `0x012E22A4` word: the handle, 1, 0, 2), hand over through callee 17
/// (thiscall on the `0x012BD0C4` word: callee 16's answer), clear the handle
/// and finish with preset 1. The finalizer presets the result byte, runs
/// callee 18 (thiscall on the first scratch buffer) and returns the byte.
///
/// The answer buffer and both scratch buffers live below the incoming stack
/// pointer on both sides, so their addresses are excluded from the call
/// comparison; their scripted contents are verified through every use. The
/// flag byte the second query writes is a single byte the checker can only
/// store word-wise: the contract's answer word preserves the three
/// neighbouring bytes, giving exact byte-store semantics on both sides, and
/// the finalizer's result byte sits below its buffer, addressed with a
/// wrapping negative offset. Float words are only moved, never computed.
/// Original: 0x00B72DA0 (thiscall, one stack word), returns an 8-bit boolean.
lf_checker_rt::export!(thiscall, rw_00B72DA0(this: u32, ped: u32) -> u32 {
    unsafe {
        const TASK_QUERY: u32 = 0x20;
        const TASK_ID: u32 = 0x30;
        const TASK_KIND: u32 = 0x34;
        const TASK_CAR: u32 = 0x38;
        const TASK_SPOT: u32 = 0x40;
        const TASK_SEED: u32 = 0x50;
        const TASK_DELAY: u32 = 0x54;
        const TASK_ARMED: u32 = 0x58;
        const TASK_RETRY: u32 = 0x59;
        const DELAY_INIT: u32 = 0x7d0;
        const PED_BODY: u32 = 0x20;
        const PED_INDEX: u32 = 0x2e;
        const BODY_X: u32 = 0x30;
        const BODY_Y: u32 = 0x34;
        const BODY_Z: u32 = 0x38;
        const SERVICE_FILE_VA: u32 = 0x01177a80;
        const SEED_FILE_VA: u32 = 0x011735b4;
        const TABLE1_FILE_VA: u32 = 0x0118d818;
        const TABLE2_FILE_VA: u32 = 0x01295cd8;
        const WORD2_FILE_VA: u32 = 0x012b4138;
        const WORD3_FILE_VA: u32 = 0x012e22a4;
        const WORD4_FILE_VA: u32 = 0x012bd0c4;
        const ENTRY_BIAS: u32 = 0x10;
        const ENTRY_PARAM: u32 = 0x1c;
        const QUERY_MAGIC: u32 = 0x497423fe;
        const THREE_BITS: u32 = 0x40400000;
        const EIGHT_BITS: u32 = 0x41000000;
        const RESULT_BACK: u32 = 0x36;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn seed() -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(SEED_FILE_VA) as *const u32).read() }
        }
        /// Run the finalizer (callee 18) over a result slot and return the
        /// result byte. The original passes a buffer whose result byte sits
        /// `RESULT_BACK` bytes below it; the rewrite passes the slot address
        /// raised by the same amount, so the scripted store lands on it.
        #[inline(always)]
        unsafe fn finalize(preset: u8) -> u32 {
            unsafe {
                let mut slot: u32 = preset as u32;
                let at = core::ptr::addr_of_mut!(slot) as u32;
                lf_checker_rt::callee_thiscall!(18, u32, at.wrapping_add(RESULT_BACK));
                (core::ptr::addr_of!(slot) as *const u8).read() as u32
            }
        }

        let service = lf_checker_rt::relocated(SERVICE_FILE_VA);
        let mut answer = [0u32; 8];
        let buf = core::ptr::addr_of_mut!(answer) as u32;
        lf_checker_rt::callee_thiscall!(
            1,
            u32,
            service,
            buf,
            this.wrapping_add(TASK_QUERY),
            QUERY_MAGIC,
            1,
            0,
            0,
            0,
            0,
            0
        );
        let flag_at = buf.wrapping_add(3);
        lf_checker_rt::callee_thiscall!(
            2,
            u32,
            service,
            buf.wrapping_add(8),
            rd32(buf.wrapping_add(4)),
            flag_at
        );
        if (flag_at as *const u8).read() == 0 {
            if rd8(this.wrapping_add(TASK_ARMED)) == 0 {
                wr32(this.wrapping_add(TASK_SEED), seed());
                wr32(this.wrapping_add(TASK_DELAY), DELAY_INIT);
                wr8(this.wrapping_add(TASK_ARMED), 1);
                if rd8(this.wrapping_add(TASK_ARMED)) == 0 {
                    return 0;
                }
            }
            if rd8(this.wrapping_add(TASK_RETRY)) != 0 {
                wr32(this.wrapping_add(TASK_SEED), seed());
                wr8(this.wrapping_add(TASK_RETRY), 0);
            }
            let total = rd32(this.wrapping_add(TASK_DELAY))
                .wrapping_add(rd32(this.wrapping_add(TASK_SEED)));
            if (total as i32) <= (seed() as i32) {
                return 1;
            }
            return 0;
        }
        wr8(this.wrapping_add(TASK_ARMED), 0);
        wr32(this.wrapping_add(TASK_SPOT), rd32(buf.wrapping_add(8)));
        wr32(this.wrapping_add(TASK_SPOT).wrapping_add(4), rd32(buf.wrapping_add(12)));
        wr32(this.wrapping_add(TASK_SPOT).wrapping_add(8), rd32(buf.wrapping_add(16)));
        wr32(this.wrapping_add(TASK_SPOT).wrapping_add(12), rd32(buf.wrapping_add(20)));
        if rd32(this.wrapping_add(TASK_CAR)) == 0 {
            let table1 = lf_checker_rt::relocated(TABLE1_FILE_VA);
            let index = rd32(table1);
            let entry = rd32(table1.wrapping_add(index.wrapping_mul(4)));
            let first: u32 = lf_checker_rt::callee_thiscall!(
                3,
                u32,
                entry.wrapping_add(ENTRY_BIAS),
                rd32(buf.wrapping_add(8)),
                rd32(buf.wrapping_add(12)),
                rd32(buf.wrapping_add(16)),
                THREE_BITS,
                0
            );
            if first != 0 {
                return 1;
            }
            let body = rd32(ped.wrapping_add(PED_BODY));
            let tindex = rd16(ped.wrapping_add(PED_INDEX)) as i16 as i32 as u32;
            let table2 = lf_checker_rt::relocated(TABLE2_FILE_VA);
            let picked = rd32(table2.wrapping_add(tindex.wrapping_mul(4)));
            let extra = rd32(picked.wrapping_add(ENTRY_PARAM));
            let index = rd32(table1);
            let entry = rd32(table1.wrapping_add(index.wrapping_mul(4)));
            let second: u32 = lf_checker_rt::callee_thiscall!(
                4,
                u32,
                entry.wrapping_add(ENTRY_BIAS),
                rd32(body.wrapping_add(BODY_X)),
                rd32(body.wrapping_add(BODY_Y)),
                rd32(body.wrapping_add(BODY_Z)),
                extra,
                0
            );
            if second != 0 {
                return 1;
            }
            let go: u32 = lf_checker_rt::callee_cdecl!(
                5,
                u32,
                body.wrapping_add(BODY_X),
                THREE_BITS,
                1,
                0xffff_ffff,
                0,
                1,
                1,
                1
            );
            if go & 0xff == 0 {
                return 1;
            }
        }
        let mut scratch = [0u32; 16];
        let work = core::ptr::addr_of_mut!(scratch) as u32;
        lf_checker_rt::callee_thiscall!(6, u32, work, 0, 0, 0, EIGHT_BITS);
        lf_checker_rt::callee_thiscall!(7, u32, work, ped);
        let word2: u32 =
            (lf_checker_rt::global::<u32>(WORD2_FILE_VA) as *const u32).read();
        let ident = rd32(this.wrapping_add(TASK_ID));
        let checked: u32 = lf_checker_rt::callee_cdecl!(8, u32, ident, word2);
        if checked & 0xff == 0 {
            lf_checker_rt::callee_cdecl!(9, u32, ident, word2, 0x0c);
            return finalize(0);
        }
        if rd32(this.wrapping_add(TASK_CAR)) == 0 {
            let made: u32 = lf_checker_rt::callee_cdecl!(
                10,
                u32,
                ident,
                this.wrapping_add(TASK_SPOT),
                0,
                1
            );
            wr32(this.wrapping_add(TASK_CAR), made);
            let kind = rd8(this.wrapping_add(TASK_KIND)) as u32;
            let mut build = [0u32; 8];
            let at = core::ptr::addr_of_mut!(build) as u32;
            lf_checker_rt::callee_thiscall!(11, u32, at, made, 0, 0, kind);
            lf_checker_rt::callee_thiscall!(12, u32, at, ped);
            lf_checker_rt::callee_thiscall!(13, u32, at);
        }
        let polled: u32 =
            lf_checker_rt::callee_thiscall!(14, u32, rd32(this.wrapping_add(TASK_CAR)));
        if polled & 0xff != 0 {
            return finalize(0);
        }
        let car = rd32(this.wrapping_add(TASK_CAR));
        lf_checker_rt::callee_cdecl!(15, u32, car);
        let word3: u32 =
            (lf_checker_rt::global::<u32>(WORD3_FILE_VA) as *const u32).read();
        let reconf: u32 =
            lf_checker_rt::callee_thiscall!(16, u32, word3, car, 1, 0, 2);
        let word4: u32 =
            (lf_checker_rt::global::<u32>(WORD4_FILE_VA) as *const u32).read();
        lf_checker_rt::callee_thiscall!(17, u32, word4, reconf);
        wr32(this.wrapping_add(TASK_CAR), 0);
        finalize(1)
    }
});
