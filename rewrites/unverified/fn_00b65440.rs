// original: 0x00B65440 veh_refresh_binding
/// Refresh the binding for `a1`, then re-resolve and stamp done.
///
/// Returns at once when `a1` is null. When the guard chain on `a0` is fully
/// present (`a0`, `[a0+0x6c]`, its ready byte) but the flag at `[this+0x109]`
/// is clear, returns without touching anything. Otherwise reads the next link
/// at `[a1+0x25c]`: when null, notifies (stubbed, thiscall/5) with
/// `(0x2e,1,1,0)`; else notifies with `([nx+0x18], word[nx+0x60], 1, 1, 0)`
/// and, when `[this+3*[this]*4+0x24]` still holds `[nx+0x18]`, converts
/// (stubbed, thiscall/1) and stores back (stubbed, thiscall/1). Reports
/// (stubbed, thiscall/3) with `(a0,-1,a1)`; when the live object (stubbed,
/// thiscall/0) is present twice with word 0x2e at +0x18, writes the saved
/// head to `[this+4]`. Stamps `[this+0xbf]`. Thiscall, two stack words.
export!(thiscall, rw_00b65440(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const GUARD: u32 = 0x6c;
        const READY: u32 = 0x0e;
        const FLAG: u32 = 0x109;
        const NEXT: u32 = 0x25c;
        const REF: u32 = 0x18;
        const WORD: u32 = 0x60;
        const IDX: u32 = 4;
        const PAIR: u32 = 0x24;
        const DONE: u32 = 0xbf;
        const HEAVY: u32 = 0x2e;
        if a1 == 0 {
            return 0;
        }
        if a0 != 0 {
            let n = ((a0 + GUARD) as *const u32).read_unaligned();
            if n != 0
                && ((n + READY) as *const u8).read() != 0
                && ((this + FLAG) as *const u8).read() == 0
            {
                return 0;
            }
        }
        let nx = ((a1 + NEXT) as *const u32).read_unaligned();
        let head = (this as *const u32).read_unaligned();
        if nx == 0 {
            let _: u32 = callee_thiscall!(1, u32, this, HEAVY, 1, 1, 0);
        } else {
            let w = ((nx + WORD) as *const u16).read_unaligned() as u32;
            let r = ((nx + REF) as *const u32).read_unaligned();
            let _: u32 = callee_thiscall!(1, u32, this, r, w, 1, 1, 0);
            let slot = this.wrapping_add((head.wrapping_mul(3)).wrapping_mul(4).wrapping_add(PAIR));
            if (slot as *const u32).read_unaligned() == r {
                let v: u32 = callee_thiscall!(2, u32, this, head);
                let _: u32 = callee_thiscall!(3, u32, nx, v);
            }
        }
        let _: u32 = callee_thiscall!(4, u32, this, a0, 0xffffffff, a1);
        let c1: u32 = callee_thiscall!(5, u32, this);
        if c1 != 0 {
            let c2: u32 = callee_thiscall!(5, u32, this);
            if ((c2 + REF) as *const u32).read_unaligned() == HEAVY {
                ((this + 4) as *mut u32).write_unaligned(head);
            }
        }
        ((this + DONE) as *mut u8).write(1);
        0
    }
});
