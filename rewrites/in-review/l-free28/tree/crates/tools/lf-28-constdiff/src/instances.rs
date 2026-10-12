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
pub const INSTANCES: [Named<ConstantWrites>; 26] = [
    Named {
        name: "net_init_small_rec",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x0, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x4, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x8, width: WriteWidth::Word, value: 0xffffffff }, ConstantWrite { offset: 0xc, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x10, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x14, width: WriteWidth::Word, value: 0x0 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "rage::ptxEventEffect::vf12",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x110, width: WriteWidth::Word, value: 0x0 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "blob21_zero",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x0, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x4, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x8, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0xc, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x10, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x14, width: WriteWidth::Word, value: 0x0 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "input_state_init",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x20, width: WriteWidth::Half, value: 0x0 }, ConstantWrite { offset: 0x24, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x8, width: WriteWidth::Byte, value: 0x1 }, ConstantWrite { offset: 0x30, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x34, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x38, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x3c, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x40, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x44, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x48, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x4c, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x54, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x58, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x5c, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x50, width: WriteWidth::Word, value: 0x3f800000 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "init_dual_fade",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x10, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x14, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x18, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x1c, width: WriteWidth::Word, value: 0x3f800000 }, ConstantWrite { offset: 0x20, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x24, width: WriteWidth::Word, value: 0x3f800000 }, ConstantWrite { offset: 0x28, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x2c, width: WriteWidth::Word, value: 0x0 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "audio_vec_zero16",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x0, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x1, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x2, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x3, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x4, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x5, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x6, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x7, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x8, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x9, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0xa, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0xb, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0xc, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0xd, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0xe, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0xf, width: WriteWidth::Byte, value: 0x0 }],
            returns: Returns::Zero,
        },
    },
    Named {
        name: "vehicle_initialize_attachment_record",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x0, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x4, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x8, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0xc, width: WriteWidth::Word, value: 0x1 }, ConstantWrite { offset: 0x10, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x12, width: WriteWidth::Half, value: 0x0 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "zero_wheel_field_block",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0xf8, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0xf4, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0xf0, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x108, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x104, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x100, width: WriteWidth::Word, value: 0x0 }],
            returns: Returns::Zero,
        },
    },
    Named {
        name: "init_param_triple",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x0, width: WriteWidth::Word, value: 0xefffffff }, ConstantWrite { offset: 0x4, width: WriteWidth::Word, value: 0x3f800000 }, ConstantWrite { offset: 0x8, width: WriteWidth::Word, value: 0x3f800000 }],
            returns: Returns::Zero,
        },
    },
    Named {
        name: "id_list_init",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x0, width: WriteWidth::Word, value: 0xffffffff }, ConstantWrite { offset: 0x4, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x8, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0xc, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x10, width: WriteWidth::Word, value: 0x3 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "zero_fields_4",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x8, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0xc, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x10, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x14, width: WriteWidth::Word, value: 0x0 }],
            returns: Returns::Zero,
        },
    },
    Named {
        name: "reset_record",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x0, width: WriteWidth::Word, value: 0xffffffff }, ConstantWrite { offset: 0x4, width: WriteWidth::Half, value: 0xffff }, ConstantWrite { offset: 0x8, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0xc, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x10, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x14, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x18, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x1c, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x20, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x24, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x2c, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x28, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x30, width: WriteWidth::Byte, value: 0x0 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "aud_placeable_tracker_state_init",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x0, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x4, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x8, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0xc, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x14, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x18, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x20, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x24, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x10, width: WriteWidth::Word, value: 0x3f800000 }, ConstantWrite { offset: 0x1c, width: WriteWidth::Word, value: 0x447a0000 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "CCamScript::vf6",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x148, width: WriteWidth::Half, value: 0x0 }, ConstantWrite { offset: 0x140, width: WriteWidth::Word, value: 0x3e8 }, ConstantWrite { offset: 0x144, width: WriteWidth::Word, value: 0x3e8 }],
            returns: Returns::One,
        },
    },
    Named {
        name: "CTaskSimpleMovePathfindProblem::vf7",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x0, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x4, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x8, width: WriteWidth::Word, value: 0x0 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "task_node_676_reset",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x280, width: WriteWidth::Word, value: 0x0 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "ccam_free_clear_vec",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x150, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x154, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x158, width: WriteWidth::Word, value: 0x0 }],
            returns: Returns::Zero,
        },
    },
    Named {
        name: "ccam_init_float_params",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x164, width: WriteWidth::Word, value: 0x3f800000 }, ConstantWrite { offset: 0x160, width: WriteWidth::Word, value: 0x447a0000 }, ConstantWrite { offset: 0x168, width: WriteWidth::Word, value: 0x41200000 }, ConstantWrite { offset: 0x16c, width: WriteWidth::Word, value: 0x4788b800 }, ConstantWrite { offset: 0x170, width: WriteWidth::Word, value: 0x41200000 }, ConstantWrite { offset: 0x174, width: WriteWidth::Word, value: 0x41200000 }, ConstantWrite { offset: 0x178, width: WriteWidth::Word, value: 0x4788b800 }, ConstantWrite { offset: 0x17c, width: WriteWidth::Word, value: 0x3f800000 }, ConstantWrite { offset: 0x180, width: WriteWidth::Word, value: 0x3f800000 }, ConstantWrite { offset: 0x184, width: WriteWidth::Word, value: 0x3f000000 }, ConstantWrite { offset: 0x14c, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x18c, width: WriteWidth::Byte, value: 0x1 }],
            returns: Returns::Zero,
        },
    },
    Named {
        name: "ccam_cinematic_veh_offset_init",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x140, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x144, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x148, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x150, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x154, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x158, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x160, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x164, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x168, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x170, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x174, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x178, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x188, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x180, width: WriteWidth::Word, value: 0xffffffff }, ConstantWrite { offset: 0x184, width: WriteWidth::Word, value: 0xffffffff }, ConstantWrite { offset: 0x18c, width: WriteWidth::Byte, value: 0x0 }],
            returns: Returns::One,
        },
    },
    Named {
        name: "ccam_market_init",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x140, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x144, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x150, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x154, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x14c, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x158, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x148, width: WriteWidth::Word, value: 0x40a00000 }, ConstantWrite { offset: 0x15c, width: WriteWidth::Word, value: 0x42340000 }, ConstantWrite { offset: 0x160, width: WriteWidth::Half, value: 0x0 }, ConstantWrite { offset: 0x162, width: WriteWidth::Byte, value: 0x0 }],
            returns: Returns::One,
        },
    },
    Named {
        name: "audio_state_reset_small",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x4, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x8, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0xc, width: WriteWidth::Byte, value: 0x0 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "audio_matrix_identity_reset",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x8, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x4, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x0, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x10, width: WriteWidth::Word, value: 0x3f800000 }, ConstantWrite { offset: 0x14, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x18, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x20, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x24, width: WriteWidth::Word, value: 0x3f800000 }, ConstantWrite { offset: 0x28, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x30, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x34, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x38, width: WriteWidth::Word, value: 0x3f800000 }, ConstantWrite { offset: 0x48, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x44, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x40, width: WriteWidth::Word, value: 0x0 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "audio_block_zero_wide",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x0, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x4, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x8, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x20, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x24, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x28, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x2c, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x3c, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x38, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x34, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x30, width: WriteWidth::Word, value: 0x0 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "audio_struct_zero22",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x0, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x1, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x2, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x3, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x4, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x5, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x6, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x7, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x8, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x9, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0xa, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0xb, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0xc, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0xd, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0xe, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0xf, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x10, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x11, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x12, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x13, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x14, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x15, width: WriteWidth::Byte, value: 0x0 }],
            returns: Returns::This,
        },
    },
    Named {
        name: "uimouse_clear_pair",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x200, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x204, width: WriteWidth::Word, value: 0x0 }],
            returns: Returns::Zero,
        },
    },
    Named {
        name: "StatsRecord_InitBasic",
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x0, width: WriteWidth::Byte, value: 0x1 }, ConstantWrite { offset: 0x4, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x8, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0xc, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x10, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x14, width: WriteWidth::Half, value: 0x0 }, ConstantWrite { offset: 0x1c, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x18, width: WriteWidth::Word, value: 0x0 }],
            returns: Returns::Zero,
        },
    },
];
