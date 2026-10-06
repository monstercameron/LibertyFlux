// original: 0x009A6E50 audio_activate_slot (proposed)

/// Activates a pending slot: binds the request, resolves the voice, marks live.
///
/// thiscall, one stack word `req` pointing at the request object. When
/// the state word at `STATE` (+0x3AC8) of `this` is not 2, returns at
/// once (the return register keeps caller leftovers on this path, so the
/// contract compares no return channel). Otherwise stores `req` at
/// `BOUND` (+0x3ACC), binds it through callee 1 (thiscall on `req` with
/// the bound-slot address as its word),
/// then resolves the voice from the info object at `INFO` (+0x3AC4):
/// id bytes `kind` (+4) and `bank` (+0x40); a `kind` of `NO_VOICE`
/// (0xFF, equality compare) selects the null voice, otherwise the
/// address is `STRIDE_G * kind + table` with the stride global at
/// `STRIDE`, the base global at `TABLE`, `TABLE_OFF` 0x6F14 and
/// `BANK_STRIDE` 0x6F40 (all arithmetic wraps mod 2^32). Callee 2
/// (thiscall on `req`) produces a token that callee 3 (thiscall on the
/// voice) consumes; then the request's link dword at +0x20 selects a
/// position (null link: `req + 0x10`, else `link + 0x30`) that callee 4
/// (thiscall on the info object) takes, callee 5 (thiscall on the info
/// object, no words) finalises, and the slot is marked live: `STATE` to
/// 3, `PREV` (+0x3AD4) to the old `NEXT` (+0x3AD0), `NEXT` to -1, the
/// flag byte at `LIVE` (+0x3AE0) to 0.
lf_checker_rt::export!(thiscall, rw_009a6e50(this: u32, req: u32) -> u32 {
    unsafe {
        const BINDER: u32 = 1;
        const TOKEN: u32 = 2;
        const PLAYER: u32 = 3;
        const PLACER: u32 = 4;
        const FINISHER: u32 = 5;
        const INFO: u32 = 0x3AC4;
        const STATE: u32 = 0x3AC8;
        const BOUND: u32 = 0x3ACC;
        const NEXT: u32 = 0x3AD0;
        const PREV: u32 = 0x3AD4;
        const LIVE: u32 = 0x3AE0;
        const ID_KIND: u32 = 4;
        const ID_BANK: u32 = 0x40;
        const NO_VOICE: u8 = 0xFF;
        const STRIDE: u32 = 0x00115D968;
        const TABLE: u32 = 0x00115D988;
        const TABLE_OFF: u32 = 0x6F14;
        const BANK_STRIDE: u32 = 0x6F40;
        if (this.wrapping_add(STATE) as *const u32).read_unaligned() != 2 {
            return 0;
        }
        let bound = this.wrapping_add(BOUND);
        (bound as *mut u32).write_unaligned(req);
        let _: u32 = lf_checker_rt::callee_thiscall!(BINDER, u32, req, bound);
        let info = (this.wrapping_add(INFO) as *const u32).read_unaligned();
        let kind = (info.wrapping_add(ID_KIND) as *const u8).read();
        let voice = if kind == NO_VOICE {
            0
        } else {
            let bank = (info.wrapping_add(ID_BANK) as *const u8).read() as u32;
            let stride = lf_checker_rt::global::<u32>(STRIDE).read_unaligned();
            let base = lf_checker_rt::global::<u32>(TABLE).read_unaligned();
            let slot = base
                .wrapping_add(bank.wrapping_mul(BANK_STRIDE))
                .wrapping_add(TABLE_OFF);
            stride
                .wrapping_mul(kind as u32)
                .wrapping_add((slot as *const u32).read_unaligned())
        };
        let token: u32 = lf_checker_rt::callee_thiscall!(TOKEN, u32, req);
        let _: u32 = lf_checker_rt::callee_thiscall!(PLAYER, u32, voice, token);
        let back = (this.wrapping_add(BOUND) as *const u32).read_unaligned();
        let link = (back.wrapping_add(0x20) as *const u32).read_unaligned();
        let pos = if link == 0 {
            back.wrapping_add(0x10)
        } else {
            link.wrapping_add(0x30)
        };
        let _: u32 = lf_checker_rt::callee_thiscall!(PLACER, u32, info, pos);
        let _: u32 = lf_checker_rt::callee_thiscall!(FINISHER, u32, info);
        let prev = (this.wrapping_add(NEXT) as *const u32).read_unaligned();
        (this.wrapping_add(STATE) as *mut u32).write_unaligned(3);
        (this.wrapping_add(PREV) as *mut u32).write_unaligned(prev);
        (this.wrapping_add(NEXT) as *mut u32).write_unaligned(0xFFFF_FFFF);
        (this.wrapping_add(LIVE) as *mut u8).write(0);
    }
    0
});
