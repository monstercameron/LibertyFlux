// original: 0x00a23720 ped_task_apply_matrix_and_notify_subobjects (proposed)

/// Apply a 3-row matrix transform to this object's blended position, then
/// notify flagged sub-objects in four stride-indexed arrays.
///
/// `this` points to the task object; `mat` points to a 15-word float block
/// with three columns at `+0x00/+0x04/+0x08`, `+0x10/+0x14/+0x18` and
/// `+0x20/+0x24/+0x28` plus three biases at `+0x30/+0x34/+0x38`. The three
/// current components at `this+0x430/0x434/0x438` weight the columns: row 0
/// is `M[0x10]*T[0x434] + M[0]*T[0x430] + M[0x20]*T[0x438] + M[0x30]`, row 1
/// uses the `0x14/0x04/0x24/0x34` column and row 2 the `0x18/0x08/0x28/0x38`
/// column, each accumulated strictly left to right in the original's
/// instruction order. The rows are stored back to `+0x430/+0x434/+0x438`;
/// `+0x43c` receives a word the original reads from its own uninitialised
/// stack scratch, which is zero under the checker's defined stack fill.
///
/// The object is then passed to an observer hook (callee 1, no stack
/// arguments), and four sub-object arrays are walked. Array k holds its
/// signed count at `+0x400+4k`, one flag byte per slot at `+0x410+k+i`, and
/// objects starting at a fixed base with a fixed stride (array 0:
/// `+0x90/0xb0`; array 1: `+0x140/0xc0`; array 2: `+0x200/0xe0`; array 3:
/// `+0x2e0/0x120`). Every slot whose flag is non-zero is notified through
/// that array's callee (ids 2..5) with the sub-object in `ecx` and `mat` as
/// the single stack argument; callee answers are ignored.
///
/// The return value is whatever the last loop left in `eax`: the array-0
/// count when only it ran, the one-past-end sub-object pointer when a later
/// array ran, or zero when no array ran.
///
/// Original: 0x00a23720 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a23720(this: u32, mat: u32) -> u32 {
    unsafe {
        const T_C0: u32 = 0x430;
        const T_C1: u32 = 0x434;
        const T_C2: u32 = 0x438;
        const T_PAD: u32 = 0x43c;
        const STACK_FILL_WORD: u32 = 0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        // Row accumulators, in the original's exact operation order.
        let w0 = rdf(this + T_C0);
        let w1 = rdf(this + T_C1);
        let w2 = rdf(this + T_C2);
        let mut x0 = mul(rdf(mat), w0);
        let mut row0 = mul(rdf(mat + 0x10), w1);
        let mut row1 = mul(rdf(mat + 0x14), w1);
        row0 = add(row0, x0);
        x0 = mul(rdf(mat + 0x20), w2);
        let mut row2 = mul(rdf(mat + 0x18), w1);
        row0 = add(row0, x0);
        x0 = mul(rdf(mat + 0x04), w0);
        row0 = add(row0, rdf(mat + 0x30));
        row1 = add(row1, x0);
        x0 = mul(rdf(mat + 0x24), w2);
        row1 = add(row1, x0);
        x0 = mul(rdf(mat + 0x08), w0);
        row1 = add(row1, rdf(mat + 0x34));
        row2 = add(row2, x0);
        x0 = mul(rdf(mat + 0x28), w2);
        row2 = add(row2, x0);
        x0 = f32::from_bits(STACK_FILL_WORD);
        row2 = add(row2, rdf(mat + 0x38));
        wrf(this + T_C0, row0);
        wrf(this + T_C1, row1);
        wrf(T_PAD + this, x0);
        wrf(this + T_C2, row2);

        // Observer hook; its answer is ignored.
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);

        // Four flag-gated notify loops. Counts are signed; a non-positive
        // count skips the array and leaves eax untouched.
        let mut eax: u32 = 0;
        let c0 = rd32(this + 0x400) as i32;
        if c0 > 0 {
            let mut obj = this.wrapping_add(0x90);
            let mut i = 0i32;
            while i < c0 {
                if rd8(this.wrapping_add((i as u32).wrapping_add(0x410))) != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, obj, mat);
                }
                i += 1;
                obj = obj.wrapping_add(0xb0);
            }
            eax = c0 as u32;
        }
        let c1 = rd32(this + 0x404) as i32;
        if c1 > 0 {
            let mut obj = this.wrapping_add(0x140);
            let mut i = 0i32;
            while i < c1 {
                if rd8(this.wrapping_add((i as u32).wrapping_add(0x411))) != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, obj, mat);
                }
                i += 1;
                obj = obj.wrapping_add(0xc0);
            }
            eax = obj;
        }
        let c2 = rd32(this + 0x408) as i32;
        if c2 > 0 {
            let mut obj = this.wrapping_add(0x200);
            let mut i = 0i32;
            while i < c2 {
                if rd8(this.wrapping_add((i as u32).wrapping_add(0x412))) != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, obj, mat);
                }
                i += 1;
                obj = obj.wrapping_add(0xe0);
            }
            eax = obj;
        }
        let c3 = rd32(this + 0x40c) as i32;
        if c3 > 0 {
            let mut obj = this.wrapping_add(0x2e0);
            let mut i = 0i32;
            while i < c3 {
                if rd8(this.wrapping_add((i as u32).wrapping_add(0x413))) != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, obj, mat);
                }
                i += 1;
                obj = obj.wrapping_add(0x120);
            }
            eax = obj;
        }
        eax
    }
});
