//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified 32-bit method of the thirteen lifted channel
//! classes. `Proven` means the method is restated on its channel type and
//! the differential test crate ran it against its verified rewrite on the
//! same generated inputs, comparing results and every effect, with a
//! deliberately wrong lift caught alongside. The base channel class has no
//! lifted type (its methods forward to one slot each, which carries no
//! behaviour) and no rows here. Counts below come from this table.

/// Lift state of one verified method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on its channel type and proven against its rewrite.
    Proven,
    /// Restated but not proven (no row should stay here: everything
    /// lifted in this module is proven).
    Lifted,
    /// Not lifted, for the stated reason.
    Missing,
}

/// One verified method's row.
#[derive(Debug, Clone, Copy)]
pub struct Row {
    /// Channel class, e.g. `"StaticFloat"`.
    pub channel: &'static str,
    /// Slot or method name in the 32-bit form, e.g. `"vf4"`.
    pub method: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away, or why the method is missing. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every verified method of the lifted classes, in class order.
pub const ROWS: &[Row] = &[
    // Static float: all six verified methods.
    Row {
        channel: "StaticFloat",
        method: "eval_indexed",
        state: State::Proven,
        narrows: &[
            "float-stack return travels as f32; its signalling-NaN quieting is reproduced as bit operations",
        ],
    },
    Row {
        channel: "StaticFloat",
        method: "vf9",
        state: State::Proven,
        narrows: &["out-pointer answer narrows to the value"],
    },
    Row {
        channel: "StaticFloat",
        method: "vf4",
        state: State::Proven,
        narrows: &["out-pointer answer narrows to the value"],
    },
    Row {
        channel: "StaticFloat",
        method: "vf14",
        state: State::Proven,
        narrows: &[
            "samples arrive as a slice: count and stride must stay in it",
            "1/0 answer narrows to bool",
        ],
    },
    Row {
        channel: "StaticFloat",
        method: "vf1",
        state: State::Missing,
        narrows: &[
            "copy constructor through the thread allocator: Clone covers it, no behaviour to restate",
        ],
    },
    Row {
        channel: "StaticFloat",
        method: "vf20",
        state: State::Missing,
        narrows: &[
            "serializer through stream-helper callees: needs the stream trait, not modelled yet",
        ],
    },
    // Static int: both verified methods.
    Row {
        channel: "StaticInt",
        method: "vf17",
        state: State::Proven,
        narrows: &[
            "samples arrive as a slice: count must stay in it",
            "1/0 answer narrows to bool",
        ],
    },
    Row {
        channel: "StaticInt",
        method: "vf1",
        state: State::Missing,
        narrows: &["copy constructor through the thread allocator: Clone covers it"],
    },
    // Raw float: the lerp pair and the sampler; the build and lifecycle need callees.
    Row {
        channel: "RawFloat",
        method: "vf9",
        state: State::Proven,
        narrows: &[
            "out-pointer answer narrows to the value",
            "idx + 1 must stay in the keys",
        ],
    },
    Row {
        channel: "RawFloat",
        method: "vf13",
        state: State::Proven,
        narrows: &[
            "x87 double return travels as f64: same bits",
            "idx + 1 must stay in the keys",
        ],
    },
    Row {
        channel: "RawFloat",
        method: "vf4",
        state: State::Proven,
        narrows: &[
            "out-pointer answer narrows to the value",
            "channel must hold a key (clamp to key -1 is meaningless)",
        ],
    },
    Row {
        channel: "RawFloat",
        method: "vf14",
        state: State::Missing,
        narrows: &["build-from-samples through allocator and copier callees"],
    },
    Row {
        channel: "RawFloat",
        method: "vf1",
        state: State::Missing,
        narrows: &["copy constructor through the thread allocator: Clone covers it"],
    },
    Row {
        channel: "RawFloat",
        method: "vf0",
        state: State::Missing,
        narrows: &["deleting destructor through the thread allocator: Drop covers it"],
    },
    // Raw int: key copy, sampler, size.
    Row {
        channel: "RawInt",
        method: "vf10",
        state: State::Proven,
        narrows: &[
            "out-pointer answer narrows to the value",
            "idx must stay in the keys",
        ],
    },
    Row {
        channel: "RawInt",
        method: "vf5",
        state: State::Proven,
        narrows: &[
            "out-pointer answer narrows to the value",
            "channel must hold a key",
        ],
    },
    Row {
        channel: "RawInt",
        method: "vf19",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        channel: "RawInt",
        method: "vf20",
        state: State::Missing,
        narrows: &["serializer through stream-helper callees"],
    },
    Row {
        channel: "RawInt",
        method: "vf17",
        state: State::Missing,
        narrows: &["build-from-source through allocator and sizer callees"],
    },
    Row {
        channel: "RawInt",
        method: "vf1",
        state: State::Missing,
        narrows: &["copy constructor through the thread allocator: Clone covers it"],
    },
    Row {
        channel: "RawInt",
        method: "vf0",
        state: State::Missing,
        narrows: &["deleting destructor through the thread allocator: Drop covers it"],
    },
    // Raw bool: sampler and size.
    Row {
        channel: "RawBool",
        method: "vf6",
        state: State::Proven,
        narrows: &[
            "out-pointer answer narrows to the byte",
            "channel must hold a byte",
            "frames selecting past the last byte panic: the original over-reads one byte of allocator slack",
        ],
    },
    Row {
        channel: "RawBool",
        method: "storage_size",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        channel: "RawBool",
        method: "try_build_from_samples",
        state: State::Proven,
        narrows: &[
            "the installed buffer travels as bytes: its address is not compared",
            "the pair words stay zeroed on the rewrite side and are not modelled",
            "inputs longer than 2,147,483,647 samples panic: the original reads out of bounds there",
            "inputs packing past 65,535 bytes panic: the count word is 16 bits",
            "the old buffer's release goes through a stub: Drop covers it",
        ],
    },
    Row {
        channel: "RawBool",
        method: "vf20",
        state: State::Missing,
        narrows: &["serializer through stream-helper callees"],
    },
    Row {
        channel: "RawBool",
        method: "vf1",
        state: State::Missing,
        narrows: &["copy constructor through the thread allocator: Clone covers it"],
    },
    Row {
        channel: "RawBool",
        method: "vf0",
        state: State::Missing,
        narrows: &["deleting destructor through the thread allocator: Drop covers it"],
    },
    // Raw vector: indexed lerp, sampler, size.
    Row {
        channel: "RawVec3",
        method: "sample_indexed",
        state: State::Proven,
        narrows: &["idx + 1 must stay in the keys"],
    },
    Row {
        channel: "RawVec3",
        method: "vf2",
        state: State::Proven,
        narrows: &[
            "integer answer (out on lerp, padding on snap) is not modelled: the padding is in the sample on the snap path",
            "channel must hold a key",
        ],
    },
    Row {
        channel: "RawVec3",
        method: "storage_size",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        channel: "RawVec3",
        method: "vf15",
        state: State::Missing,
        narrows: &["build-from-source through allocator and sizer callees"],
    },
    Row {
        channel: "RawVec3",
        method: "vf20",
        state: State::Missing,
        narrows: &["serializer through stream-helper callees"],
    },
    Row {
        channel: "RawVec3",
        method: "clone",
        state: State::Missing,
        narrows: &["clone through the thread allocator: Clone covers it"],
    },
    Row {
        channel: "RawVec3",
        method: "dtor",
        state: State::Missing,
        narrows: &["deleting destructor through the thread allocator: Drop covers it"],
    },
    // Static vector: copy and uniformity check.
    Row {
        channel: "StaticVec3",
        method: "vf7",
        state: State::Proven,
        narrows: &[
            "out-pointer answer and padding-word answer narrow to the value, which carries the word",
        ],
    },
    Row {
        channel: "StaticVec3",
        method: "compress",
        state: State::Proven,
        narrows: &[
            "records arrive as a slice: count must stay in it",
            "1/0 answer narrows to bool",
        ],
    },
    Row {
        channel: "StaticVec3",
        method: "vf1",
        state: State::Missing,
        narrows: &["copy constructor through the thread allocator: Clone covers it"],
    },
    Row {
        channel: "StaticVec3",
        method: "ctor",
        state: State::Missing,
        narrows: &["in-place constructor allocating the value block: new() covers it"],
    },
    Row {
        channel: "StaticVec3",
        method: "deleting",
        state: State::Missing,
        narrows: &["deleting destructor through the thread allocator: Drop covers it"],
    },
    Row {
        channel: "StaticVec3",
        method: "copy_from",
        state: State::Missing,
        narrows: &["copy through session-relocation and allocator callees"],
    },
    Row {
        channel: "StaticVec3",
        method: "map",
        state: State::Missing,
        narrows: &["session slot remap through relocation callees: no portable meaning yet"],
    },
    Row {
        channel: "StaticVec3",
        method: "vf20",
        state: State::Missing,
        narrows: &["single call through a helper: no behaviour in it"],
    },
    // Static quaternion: copy only.
    Row {
        channel: "StaticQuat",
        method: "vf8",
        state: State::Proven,
        narrows: &["out-pointer answer narrows to the value"],
    },
    Row {
        channel: "StaticQuat",
        method: "vf1",
        state: State::Missing,
        narrows: &["copy constructor through the thread allocator: Clone covers it"],
    },
    Row {
        channel: "StaticQuat",
        method: "vf0",
        state: State::Missing,
        narrows: &["deleting destructor through the thread allocator: Drop covers it"],
    },
    Row {
        channel: "StaticQuat",
        method: "copy_from",
        state: State::Missing,
        narrows: &["copy through session-relocation and allocator callees"],
    },
    Row {
        channel: "StaticQuat",
        method: "map",
        state: State::Missing,
        narrows: &["session slot remap through relocation callees: no portable meaning yet"],
    },
    Row {
        channel: "StaticQuat",
        method: "vf3",
        state: State::Proven,
        narrows: &["out-pointer answer narrows to the value"],
    },
    Row {
        channel: "StaticQuat",
        method: "vf16",
        state: State::Proven,
        narrows: &[
            "records arrive as a slice: count must stay in it",
            "1/0 answer narrows to bool",
        ],
    },
    Row {
        channel: "StaticQuat",
        method: "vf20",
        state: State::Missing,
        narrows: &["single call through a helper: no behaviour in it"],
    },
    // Quantized float: the evaluator alone; the rest needs the extractor callee.
    Row {
        channel: "QuantizeFloat",
        method: "vf13",
        state: State::Proven,
        narrows: &["x87 double return travels as f64: same bits"],
    },
    Row {
        channel: "QuantizeFloat",
        method: "vf9",
        state: State::Missing,
        narrows: &["sampler through the extractor callee"],
    },
    Row {
        channel: "QuantizeFloat",
        method: "vf12",
        state: State::Missing,
        narrows: &["pair fetch through the extractor callee"],
    },
    Row {
        channel: "QuantizeFloat",
        method: "vf4",
        state: State::Missing,
        narrows: &["sampler through the extractor callee"],
    },
    Row {
        channel: "QuantizeFloat",
        method: "vf19",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        channel: "QuantizeFloat",
        method: "vf1",
        state: State::Missing,
        narrows: &["copy constructor through the thread allocator: Clone covers it"],
    },
    Row {
        channel: "QuantizeFloat",
        method: "vf0",
        state: State::Missing,
        narrows: &["deleting destructor through the thread allocator: Drop covers it"],
    },
    // Raw quaternion: the indexed blend alone; the rest needs callees.
    Row {
        channel: "RawQuat",
        method: "sample_indexed",
        state: State::Proven,
        narrows: &[
            "out-pointer answer narrows to the value",
            "idx + 1 must stay in the keys",
            "the square root is shared with the rewrite's stub: its behaviour is not proven here",
        ],
    },
    Row {
        channel: "RawQuat",
        method: "clone",
        state: State::Missing,
        narrows: &["clone through the thread allocator: Clone covers it"],
    },
    Row {
        channel: "RawQuat",
        method: "dtor",
        state: State::Missing,
        narrows: &["deleting destructor through the thread allocator: Drop covers it"],
    },
    Row {
        channel: "RawQuat",
        method: "vf3",
        state: State::Missing,
        narrows: &["sampler through the normalize callee"],
    },
    Row {
        channel: "RawQuat",
        method: "vf16",
        state: State::Missing,
        narrows: &["build-from-source through the allocator callee, with sign alignment"],
    },
    Row {
        channel: "RawQuat",
        method: "vf20",
        state: State::Missing,
        narrows: &["serializer through stream-helper callees"],
    },
    // Delta float: the size alone; the rest needs sub-object callees.
    Row {
        channel: "DeltaFloat",
        method: "vf19",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        channel: "DeltaFloat",
        method: "vf1",
        state: State::Missing,
        narrows: &["copy constructor through the thread allocator: Clone covers it"],
    },
    Row {
        channel: "DeltaFloat",
        method: "vf0",
        state: State::Missing,
        narrows: &["deleting destructor through the thread allocator: Drop covers it"],
    },
    Row {
        channel: "DeltaFloat",
        method: "vf4",
        state: State::Missing,
        narrows: &["sampler through sub-object callees"],
    },
    Row {
        channel: "DeltaFloat",
        method: "vf14",
        state: State::Missing,
        narrows: &["compressor through allocator and bit-stream callees"],
    },
    Row {
        channel: "DeltaFloat",
        method: "vf20",
        state: State::Missing,
        narrows: &["serializer through stream-helper callees"],
    },
    // Run-length int: the size alone; the decoders need the bit callee.
    Row {
        channel: "RleInt",
        method: "get_alloc_size",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        channel: "RleInt",
        method: "clone",
        state: State::Missing,
        narrows: &["clone through the thread allocator: Clone covers it"],
    },
    Row {
        channel: "RleInt",
        method: "create",
        state: State::Missing,
        narrows: &["factory through the thread allocator: new() covers it"],
    },
    Row {
        channel: "RleInt",
        method: "init_guarded",
        state: State::Missing,
        narrows: &["in-place init relocating pointers: new() covers it"],
    },
    Row {
        channel: "RleInt",
        method: "copy_from",
        state: State::Missing,
        narrows: &["copy through part-copier callees"],
    },
    Row {
        channel: "RleInt",
        method: "deleting",
        state: State::Missing,
        narrows: &["deleting destructor through the thread allocator: Drop covers it"],
    },
    Row {
        channel: "RleInt",
        method: "sample_indexed",
        state: State::Missing,
        narrows: &["decode through the bit-decoder callee"],
    },
    Row {
        channel: "RleInt",
        method: "decode_at",
        state: State::Missing,
        narrows: &["decode through the bit-decoder callee"],
    },
    Row {
        channel: "RleInt",
        method: "vf5",
        state: State::Missing,
        narrows: &["sampler through the integer-decoder callee"],
    },
    Row {
        channel: "RleInt",
        method: "compress",
        state: State::Missing,
        narrows: &["compressor through array and allocator callees"],
    },
    Row {
        channel: "RleInt",
        method: "serialize",
        state: State::Missing,
        narrows: &["serializer through stream-helper callees"],
    },
    // Curve float: size, segment evaluator and sampler; the key-list
    // management needs allocator and stream callees.
    Row {
        channel: "CurveFloat",
        method: "get_alloc_size",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        channel: "CurveFloat",
        method: "eval_segment",
        state: State::Proven,
        narrows: &[
            "out-pointer answer narrows to the value",
            "order + 1 coefficients must stay in the slice",
        ],
    },
    Row {
        channel: "CurveFloat",
        method: "sample",
        state: State::Proven,
        narrows: &[
            "out-pointer answer narrows to the value",
            "channel must hold a key (the original reads before its array when empty)",
            "order + 1 coefficients must stay in each segment",
            "the found-path segment call is the lifted eval_segment on both sides, proven separately",
        ],
    },
    Row {
        channel: "CurveFloat",
        method: "clone",
        state: State::Missing,
        narrows: &["clone through the thread allocator: Clone covers it"],
    },
    Row {
        channel: "CurveFloat",
        method: "create",
        state: State::Missing,
        narrows: &["factory through the thread allocator: new() covers it"],
    },
    Row {
        channel: "CurveFloat",
        method: "init_guarded",
        state: State::Missing,
        narrows: &["single call through a helper: no behaviour in it"],
    },
    Row {
        channel: "CurveFloat",
        method: "deleting",
        state: State::Missing,
        narrows: &["deleting destructor through key and thread allocators: Drop covers it"],
    },
    Row {
        channel: "CurveFloat",
        method: "copy_from",
        state: State::Missing,
        narrows: &["copy through the key-copier callee"],
    },
    Row {
        channel: "CurveFloat",
        method: "map",
        state: State::Missing,
        narrows: &["session slot remap through relocation callees: no portable meaning yet"],
    },
    Row {
        channel: "CurveFloat",
        method: "serialize",
        state: State::Missing,
        narrows: &["serializer through stream-helper callees"],
    },
    Row {
        channel: "CurveFloat",
        method: "copy_segment",
        state: State::Missing,
        narrows: &["segment copy through the thread allocator"],
    },
    Row {
        channel: "CurveFloat",
        method: "serialize_keys",
        state: State::Missing,
        narrows: &["serializer through stream-helper and allocator callees"],
    },
    Row {
        channel: "CurveFloat",
        method: "alloc_keys",
        state: State::Missing,
        narrows: &["zeroed array through the thread allocator: Vec covers it"],
    },
    Row {
        channel: "CurveFloat",
        method: "copy_keys",
        state: State::Missing,
        narrows: &["segment-list copy through allocator and copier callees"],
    },
    Row {
        channel: "CurveFloat",
        method: "free_keys",
        state: State::Missing,
        narrows: &["release through the thread allocator: Drop covers it"],
    },
    Row {
        channel: "CurveFloat",
        method: "purge_list",
        state: State::Missing,
        narrows: &["list drain through the thread allocator: Drop covers it"],
    },
];

/// Number of rows in each state.
#[must_use]
pub const fn counts() -> (usize, usize, usize) {
    let mut proven = 0;
    let mut lifted = 0;
    let mut missing = 0;
    let mut i = 0;
    while i < ROWS.len() {
        match ROWS[i].state {
            State::Proven => proven += 1,
            State::Lifted => lifted += 1,
            State::Missing => missing += 1,
        }
        i += 1;
    }
    (proven, lifted, missing)
}
