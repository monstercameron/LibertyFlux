//! The constant-write instances the verified rewrites were checked against.
//!
//! Every value below is a parameter read from the verified rewrite's own text, not typed in by hand. The differential test runs the
//! rewrite at index `i` and the lifted routine on the same generated inputs.

use lf_core::constant_writes::{ConstantWrite, ConstantWrites, Returns, WriteWidth};

/// One instance: the symbol name of its rewrite and the family parameters it was extracted to.
#[derive(Clone, Copy, Debug)]
pub struct Named<T> {
    /// The rewrite's symbol name.
    pub name: &'static str,
    /// The parameters.
    pub spec: T,
}

/// The instances, in the order of the call table.
pub const INSTANCES: [Named<ConstantWrites>; 18] = [
    Named {
        name: "audio_header_clear",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x14, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x10, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x1b, width: WriteWidth::Byte, value: 0x0 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "reset_state_a",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x4, width: WriteWidth::Half, value: 0x0 }, ConstantWrite { offset: 0x84, width: WriteWidth::Half, value: 0x0 }, ConstantWrite { offset: 0x0, width: WriteWidth::Word, value: 0x0 }],
            returns: Returns::Zero,
        },
    },
    Named {
        name: "init_input_vec_zero",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x0, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x4, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x8, width: WriteWidth::Word, value: 0xc0c00000 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "clear_task_queue_entry",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x24, width: WriteWidth::Word, value: 0x0 }],
            returns: Returns::One,
        },
    },
    Named {
        name: "stream_hdr_reset",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x0, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x4, width: WriteWidth::Half, value: 0x0 }, ConstantWrite { offset: 0x6, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x8, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0xc, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x13, width: WriteWidth::Byte, value: 0x0 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "veh_state_zero_tail",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x54, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x55, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x56, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x57, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x58, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x59, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x5a, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x5b, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x5c, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x5d, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x5e, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x5f, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x60, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x61, width: WriteWidth::Byte, value: 0x0 }],
            returns: Returns::Zero,
        },
    },
    Named {
        name: "object_set_state_3",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x22a, width: WriteWidth::Byte, value: 0x3 }, ConstantWrite { offset: 0x224, width: WriteWidth::Word, value: 0x0 }],
            returns: Returns::Zero,
        },
    },
    Named {
        name: "clear_arg1_field10",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x10, width: WriteWidth::Word, value: 0x0 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "task_effect_reset_slot_a",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x18, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x24, width: WriteWidth::Byte, value: 0x1 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "task_effect_reset_slot_b",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x14, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x20, width: WriteWidth::Byte, value: 0x1 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "task_effect_clear_handle_c",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x1c, width: WriteWidth::Word, value: 0x0 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "task_effect_reset_slot_d",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x60, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x74, width: WriteWidth::Byte, value: 0x1 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "slot_zero4_ret_this",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x0, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x4, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x8, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0xc, width: WriteWidth::Word, value: 0x0 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "vehicle_heading_clear_state",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x0, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x4, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x8, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0xc, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x10, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x14, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x18, width: WriteWidth::Word, value: 0x0 }],
            returns: Returns::Zero,
        },
    },
    Named {
        name: "ped_task_output_clear_status_word",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x8, width: WriteWidth::Word, value: 0x0 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "task_clear_flag_slot",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x1c, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x20, width: WriteWidth::Byte, value: 0x1 }],
            returns: Returns::Zero,
        },
    },
    Named {
        name: "clear_task_link_and_set_flag",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x1c, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x2c, width: WriteWidth::Byte, value: 0x1 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "audio_struct_zero16",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x0, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x1, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x2, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x3, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x4, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x5, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x6, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x7, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x8, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x9, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0xa, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0xb, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0xc, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0xd, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0xe, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0xf, width: WriteWidth::Byte, value: 0x0 }],
            returns: Returns::This,
        },
    },
];
