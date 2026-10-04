//! Per-file schemas: object type ids, names and field layouts.
//!
//! Every versioned file shares the container from [`crate::container`]; what
//! differs is the header every object carries and the meaning of each type
//! id. A [`Schema`] selected with [`Schema::detect`] from the file name
//! supplies both. Field layouts below were verified by decoding every object
//! of every shipped file and checking the schema consumes the object's bytes
//! exactly (see the crate-level documentation for the two known exceptions,
//! which surface as trailing bytes instead of errors).
//!
//! Array elements use one empty-named child for scalar elements
//! (for example a hash list) and named children for struct elements.

/// Width of a count prefix (array length or string length).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountWidth {
    /// One byte.
    U8,
    /// Two bytes, little-endian.
    U16,
    /// Four bytes, little-endian.
    U32,
}

/// Width of an enum's stored integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntWidth {
    /// Unsigned 8-bit.
    U8,
}

/// One field within a header, object body or array element.
#[derive(Debug, Clone, PartialEq)]
pub struct FieldDef {
    /// Field name; empty for scalar array elements.
    pub name: &'static str,
    /// Field encoding.
    pub kind: FieldKind,
}

/// How a field is stored.
#[derive(Debug, Clone, PartialEq)]
pub enum FieldKind {
    /// Unsigned 8-bit integer.
    U8,
    /// Unsigned 16-bit little-endian integer.
    U16,
    /// Unsigned 32-bit little-endian integer.
    U32,
    /// Signed 8-bit integer.
    I8,
    /// Signed 16-bit little-endian integer.
    I16,
    /// Signed 32-bit little-endian integer.
    I32,
    /// 32-bit little-endian float.
    F32,
    /// 32-bit name hash (see [`crate::hash`]).
    Hash,
    /// Bytes with a count prefix; decoded as text.
    Text(CountWidth),
    /// Count prefix followed by that many elements.
    Array {
        /// Width of the count prefix.
        count: CountWidth,
        /// Element layout: one empty-named child for scalars, named
        /// children for structs.
        element: Vec<FieldDef>,
    },
    /// Exactly `count` elements with no prefix.
    FixedArray {
        /// Element count.
        count: usize,
        /// Element layout, as for [`FieldKind::Array`].
        element: Vec<FieldDef>,
    },
    /// A u32 presence mask followed by the children whose bit is set, in
    /// schema order (bit `i` guards child `i`).
    OptionalBitfield(Vec<FieldDef>),
    /// Stored integer mapped to a name; unknown values keep their number.
    Enum {
        /// Stored integer width.
        base: IntWidth,
        /// (value, name) pairs.
        values: &'static [(i64, &'static str)],
    },
    /// Everything left in the object, of unknown layout.
    Rest,
}

fn f(name: &'static str, kind: FieldKind) -> FieldDef {
    FieldDef { name, kind }
}

fn u8f(name: &'static str) -> FieldDef {
    f(name, FieldKind::U8)
}
fn u16f(name: &'static str) -> FieldDef {
    f(name, FieldKind::U16)
}
fn u32f(name: &'static str) -> FieldDef {
    f(name, FieldKind::U32)
}
fn i8f(name: &'static str) -> FieldDef {
    f(name, FieldKind::I8)
}
fn i16f(name: &'static str) -> FieldDef {
    f(name, FieldKind::I16)
}
fn i32f(name: &'static str) -> FieldDef {
    f(name, FieldKind::I32)
}
fn f32f(name: &'static str) -> FieldDef {
    f(name, FieldKind::F32)
}
fn hash(name: &'static str) -> FieldDef {
    f(name, FieldKind::Hash)
}
fn arr(name: &'static str, count: CountWidth, element: Vec<FieldDef>) -> FieldDef {
    f(name, FieldKind::Array { count, element })
}
fn hash_list(name: &'static str, count: CountWidth) -> FieldDef {
    arr(name, count, vec![f("", FieldKind::Hash)])
}

/// Which versioned file layout to decode with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Schema {
    /// `categories.dat15`: mix categories with child-category links.
    Categories,
    /// `curves.dat12`: named response curves.
    Curves,
    /// `effects.dat11`: source/listener effects (filters, reverbs, delays).
    Effects,
    /// `sounds.dat15`: the sound graph (simple, looping, envelope,
    /// randomised, streaming and two dozen more node types).
    Sounds,
    /// `game.dat16`: game objects (collisions, emitters, radio, interiors,
    /// vehicles, zones, rules and more).
    Game,
}

impl Schema {
    /// Pick a schema from a file name (case-insensitive substring match on
    /// the bare name). Returns `None` for speech files and anything else.
    #[must_use]
    pub fn detect(file_name: &str) -> Option<Schema> {
        let base = file_name.rsplit(['/', '\\']).next().unwrap_or(file_name);
        let lower = base.to_ascii_lowercase();
        if lower.contains("categor") {
            Some(Schema::Categories)
        } else if lower.contains("curve") {
            Some(Schema::Curves)
        } else if lower.contains("effect") {
            Some(Schema::Effects)
        } else if lower.contains("sound") {
            Some(Schema::Sounds)
        } else if lower.contains("game") {
            Some(Schema::Game)
        } else {
            None
        }
    }

    /// Expected container version suffix for this schema.
    #[must_use]
    pub fn suffix(&self) -> u32 {
        match self {
            Schema::Categories | Schema::Sounds => 15,
            Schema::Curves => 12,
            Schema::Effects => 11,
            Schema::Game => 16,
        }
    }

    /// Header fields every object of this file carries (after the type id
    /// and name offset).
    #[must_use]
    pub fn header(&self) -> Vec<FieldDef> {
        match self {
            Schema::Categories => vec![],
            Schema::Curves => vec![
                u32f("flags"),
                u16f("unknown_09"),
                u16f("unknown_0b"),
                f32f("min_input"),
                f32f("max_input"),
            ],
            Schema::Effects => vec![
                u32f("flags"),
                u8f("unknown_08"),
                hash("child_effect"),
                u8f("unknown_0e"),
            ],
            Schema::Sounds => vec![
                u32f("flags"),
                u16f("unknown_09"),
                f(
                    "header",
                    FieldKind::OptionalBitfield(vec![
                        i16f("volume"),
                        u16f("volume_variance"),
                        i16f("pitch"),
                        u16f("pitch_variance"),
                        u16f("pan"),
                        u16f("pan_variance"),
                        i16f("pre_delay"),
                        u16f("pre_delay_variance"),
                        i32f("start_offset"),
                        i32f("start_offset_variance"),
                        u16f("attack_time"),
                        u16f("release_time"),
                        u16f("doppler_factor"),
                        hash("category"),
                        hash("volume_curve"),
                        u16f("volume_curve_scale"),
                        i8f("speaker_mask"),
                        i8f("effect_route"),
                        hash("volume_variable"),
                        hash("pitch_variable"),
                        hash("pan_variable"),
                        hash("unknown_variable_2"),
                        hash("unknown_variable_3"),
                        hash("cutoff_variable"),
                    ]),
                ),
            ],
            Schema::Game => vec![u32f("nametable_offset"), u8f("padding")],
        }
    }

    /// Object type name for a type id, or `None` when unknown.
    #[must_use]
    pub fn type_name(&self, id: u8) -> Option<&'static str> {
        Some(match self {
            Schema::Categories => match id {
                0 => "audCategory",
                _ => return None,
            },
            Schema::Curves => match id {
                1 => "audCurve_Constant",
                2 => "audCurve_Linear",
                3 => "audCurve_LinearDb",
                4 => "audCurve_PiecewiseLinear",
                5 => "audCurve_EqualPower",
                6 => "audCurve_ValueTable",
                7 => "audCurve_Exponential",
                8 => "audCurve_DecayingExponential",
                9 => "audCurve_DecayingSquaredExponential",
                10 => "audCurve_SineCurve",
                11 => "audCurve_OneOverX",
                12 => "audCurve_OneOverXSquared",
                13 => "audCurve_DefaultDistanceAttenuation",
                14 => "audCurve_DefaultDistanceAttenuationClamped",
                15 => "audCurve_DistanceAttenuationValueTable",
                _ => return None,
            },
            Schema::Effects => match id {
                1 => "audNullEffect",
                2 => "audReverbEffect",
                3 => "audBiquadFilterEffect",
                4 => "audConvolutionEffect",
                5 => "audCompressorEffect",
                6 => "audWaveshaperEffect",
                7 => "audDelayEffect",
                _ => return None,
            },
            Schema::Sounds => match id {
                1 => "audLoopingSound",
                2 => "audEnvelopeSound",
                3 => "audTwinLoopSound",
                4 => "audSpeechSound",
                5 => "audOnStopSound",
                6 => "audWrapperSound",
                7 => "audSequentialSound",
                8 => "audStreamingSound",
                9 => "audRetriggeredOverlappedSound",
                10 => "audCrossfadeSound",
                11 => "audCollapsingStereoSound",
                12 => "audSimpleSound",
                13 => "audMultitrackSound",
                14 => "audRandomizedSound",
                16 => "audSwitchSound",
                17 => "audVariableCurveSound",
                18 => "audVariablePrintValueSound",
                19 => "audAssertSound",
                20 => "audVariableSetTimeSound",
                21 => "audVariableBlockSound",
                22 => "audIfSound",
                23 => "audForLoopSound",
                24 => "audMathOperationSound",
                _ => return None,
            },
            Schema::Game => match id {
                0 => "gameAutomobile",
                1 => "gameCollision",
                2 => "gameAmbientEmitter",
                3 => "gameEmitterEntity",
                4 => "gameHeli",
                5 => "gameMeleeCombat",
                6 => "gameSpeechContexts",
                7 => "gameBoat",
                8 => "gameWeapon",
                9 => "gameFootsteps",
                10 => "gameRadioStationList",
                11 => "gameRadioStation",
                12 => "gameRadioStationTrackCategory",
                13 => "gameRadioStationCategoryWeights",
                14 => "gameCrime",
                15 => "gameClothing",
                16 => "gamePed",
                17 => "gameAmbientEmitterList",
                18 => "gameScriptedReport",
                19 => "gameAmbientZone",
                20 => "gameSoundRules",
                21 => "gameAmbientZoneList",
                22 => "gameTrainStation",
                23 => "gameCutscene",
                24 => "gameInterior",
                25 => "gameDoor",
                _ => return None,
            },
        })
    }

    /// Body fields for a type id, or `None` when the type is unknown.
    /// Types whose layout is unknown get a single [`FieldKind::Rest`] field.
    #[must_use]
    pub fn type_fields(&self, id: u8) -> Option<Vec<FieldDef>> {
        let rest = || vec![f("unknown_data", FieldKind::Rest)];
        Some(match self {
            Schema::Categories => match id {
                0 => vec![
                    u32f("flags"),
                    i16f("unknown_09"),
                    i16f("unknown_0b"),
                    i16f("unknown_0d"),
                    i16f("unknown_0f"),
                    i16f("unknown_11"),
                    i16f("unknown_13"),
                    i16f("unknown_15"),
                    i16f("unknown_17"),
                    u16f("unknown_19"),
                    u16f("unknown_1b"),
                    u16f("unknown_1d"),
                    u16f("unknown_1f"),
                    u16f("unknown_21"),
                    hash_list("child_categories", CountWidth::U8),
                ],
                _ => return None,
            },
            Schema::Curves => match id {
                1 => vec![f32f("value")],
                2 | 3 => vec![
                    f32f("left_x"),
                    f32f("left_y"),
                    f32f("right_x"),
                    f32f("right_y"),
                ],
                4 => vec![arr("points", CountWidth::U32, vec![f32f("x"), f32f("y")])],
                5 => vec![u8f("flip")],
                6 | 15 => vec![arr("values", CountWidth::U16, vec![f32f("y")])],
                7 => vec![u8f("flip"), f32f("exponent")],
                8 | 9 | 11 | 12 => vec![f32f("horizontal_scaling")],
                10 => vec![
                    f32f("start_phase"),
                    f32f("end_phase"),
                    f32f("frequency"),
                    f32f("vertical_scaling"),
                    f32f("vertical_offset"),
                ],
                13 => vec![],
                14 => vec![i16f("max_gain")],
                _ => return None,
            },
            Schema::Effects => match id {
                1 | 3 | 4 | 5 | 6 => vec![],
                2 => vec![
                    f32f("unknown_00"),
                    f32f("unknown_04"),
                    f32f("unknown_08"),
                    f32f("unknown_0c"),
                ],
                7 => rest(),
                _ => return None,
            },
            Schema::Sounds => sounds_type(id)?,
            Schema::Game => game_type(id, &rest)?,
        })
    }
}

/// Sound-graph node bodies (schema [`Schema::Sounds`]).
// Schema tables are data; splitting them would scatter one table per arm.
#[allow(clippy::too_many_lines)]
fn sounds_type(id: u8) -> Option<Vec<FieldDef>> {
    let sound_ref_list =
        |name: &'static str| arr(name, CountWidth::U8, vec![hash("sound"), u32f("unused")]);
    Some(match id {
        12 => vec![u32f("wave_slot_index"), hash("archive"), hash("sound")],
        13 => vec![arr(
            "tracks",
            CountWidth::U8,
            vec![hash("track"), u32f("unused")],
        )],
        1 => vec![
            u16f("loop_count"),
            u16f("loop_count_variance"),
            hash("sound"),
        ],
        2 => vec![
            u16f("attack"),
            u16f("decay"),
            u8f("sustain"),
            i32f("hold"),
            i32f("release"),
            hash("attack_curve"),
            hash("decay_curve"),
            hash("release_curve"),
            hash("attack_variable"),
            hash("decay_variable"),
            hash("sustain_variable"),
            hash("hold_variable"),
            hash("release_variable"),
            hash("sound"),
        ],
        3 => vec![
            i16f("min_swap_time"),
            i16f("max_swap_time"),
            i16f("min_crossfade_time"),
            i16f("max_crossfade_time"),
            hash("crossfade_curve"),
            hash("min_swap_time_variable"),
            hash("max_swap_time_variable"),
            hash("min_crossfade_time_variable"),
            hash("max_crossfade_time_variable"),
            sound_ref_list("sounds"),
        ],
        4 | 19 => vec![],
        5 => vec![
            hash("child_sound"),
            hash("on_pause_sound"),
            hash("on_end_sound"),
        ],
        6 => vec![hash("sound")],
        7 => vec![sound_ref_list("sounds")],
        8 => vec![u32f("duration"), sound_ref_list("sounds")],
        9 => vec![
            i16f("loop_count"),
            u16f("delay_time"),
            hash("loop_count_variable"),
            hash("delay_time_variable"),
            hash("sound"),
        ],
        10 => vec![
            hash("near_sound"),
            hash("far_sound"),
            u8f("mode"),
            f32f("min_distance"),
            f32f("max_distance"),
            i32f("hysteresis"),
            hash("crossfade_curve"),
            hash("distance_variable"),
            hash("min_distance_variable"),
            hash("max_distance_variable"),
            hash("crossfade_variable"),
        ],
        11 => vec![
            hash("left_sound"),
            hash("right_sound"),
            f32f("min_distance"),
            f32f("max_distance"),
            hash("min_distance_variable"),
            hash("max_distance_variable"),
            hash("crossfade_override_variable"),
            hash("frontend_left_pan_variable"),
            hash("frontend_right_pan_variable"),
            u8f("mode"),
        ],
        14 => vec![
            u32f("unused"),
            u8f("history_index"),
            arr("history_space", CountWidth::U8, vec![f("", FieldKind::U8)]),
            arr(
                "sounds",
                CountWidth::U8,
                vec![hash("sound"), f32f("weight")],
            ),
        ],
        16 => vec![
            hash("control_variable"),
            hash_list("sounds", CountWidth::U8),
        ],
        17 => vec![
            hash("sound"),
            hash("input_variable"),
            hash("output_variable"),
            hash("curve"),
        ],
        // A counted string, not a fixed array: one shipped object stores a
        // 6-letter label where the rest store 14-letter labels.
        18 => vec![
            hash("variable"),
            f("value", FieldKind::Text(CountWidth::U8)),
        ],
        20 => vec![hash("variable")],
        21 => vec![
            hash("sound"),
            arr(
                "variables",
                CountWidth::U8,
                vec![hash("variable"), f32f("data"), u8f("variable_type")],
            ),
        ],
        22 => vec![
            hash("true_sound"),
            hash("false_sound"),
            hash("variable_a"),
            f(
                "operator",
                FieldKind::Enum {
                    base: IntWidth::U8,
                    values: IF_CONDITIONS,
                },
            ),
            f32f("operand_b_static"),
            hash("operand_b_variable"),
        ],
        23 => vec![
            hash("sound"),
            f32f("counter_initial_static"),
            hash("counter_initial_variable"),
            f32f("counter_condition_static"),
            hash("counter_condition_variable"),
            f32f("counter_increment_static"),
            hash("counter_increment_variable"),
            hash("counter_variable"),
        ],
        24 => vec![
            hash("sound"),
            arr(
                "operations",
                CountWidth::U8,
                vec![
                    f(
                        "operation",
                        FieldKind::Enum {
                            base: IntWidth::U8,
                            values: MATH_OPERATIONS,
                        },
                    ),
                    f32f("operand_a_static"),
                    hash("operand_a_variable"),
                    f32f("operand_b_static"),
                    hash("operand_b_variable"),
                    f32f("operand_c_static"),
                    hash("operand_c_variable"),
                    hash("output_variable"),
                ],
            ),
        ],
        _ => return None,
    })
}

/// Game-object bodies (schema [`Schema::Game`]).
// Schema tables are data; splitting them would scatter one table per arm.
#[allow(clippy::too_many_lines)]
fn game_type(id: u8, rest: &dyn Fn() -> Vec<FieldDef>) -> Option<Vec<FieldDef>> {
    Some(match id {
        1 => vec![
            hash("hard_impact"),
            hash("scrape_sound"),
            hash("break_sound"),
            hash("bullet_impact_sound"),
            u16f("hardness"),
            u8f("max_impulse_magnitude"),
            u8f("max_scrape_speed"),
            u16f("impulse_magnitude_scalar"),
            u8f("bullet_collision_scaling"),
            hash("footstep_settings"),
            u8f("footstep_scaling"),
            u8f("scuffstep_scaling"),
            hash("impact_start_offset_curve"),
            hash("impact_volume_curve"),
            hash("scrape_pitch_curve"),
            hash("scrape_volume_curve"),
            hash("fast_tyre_roll"),
            hash("detail_tyre_roll"),
            hash("main_skid"),
            hash("side_skid"),
            hash("metal_shell_casing"),
            hash("plastic_shell_casing"),
            hash("roll_sound"),
            hash("rain_loop"),
            hash("tyre_bump"),
            hash("shockwave_sound"),
            hash("random_ambient"),
            u8f("material_type"),
            hash("door_material"),
        ],
        2 => vec![
            hash("child_sound"),
            hash("radio_station"),
            f32f("pos_x"),
            f32f("pos_y"),
            f32f("pos_z"),
            u32f("padding"),
            u32f("interior_room"),
            i32f("volume"),
            u16f("lpf_cutoff"),
            u16f("hpf_cutoff"),
            u16f("rolloff_factor"),
            hash("interior"),
            u32f("padding_2"),
        ],
        3 => vec![
            hash("child_sound"),
            f32f("max_distance"),
            f32f("business_hours_probability"),
            f32f("evening_probability"),
            f32f("night_probability"),
            f32f("cone_inner_angle"),
            f32f("cone_outer_angle"),
            f32f("cone_max_attenuation"),
            i32f("unknown_2"),
        ],
        4 | 5 | 7 | 9 | 15 | 18 | 22 => rest(),
        6 => vec![arr(
            "contexts",
            CountWidth::U16,
            vec![
                hash("context"),
                u32f("unknown_04"),
                i32f("unknown_08"),
                u32f("unknown_0c"),
                u8f("unknown_10"),
                hash("unknown_hash"),
                u32f("unknown_15"),
            ],
        )],
        8 => vec![
            hash("fire"),
            hash("echo"),
            hash("casing_bounce"),
            hash("swipe_sound"),
            hash("collision"),
            hash("melee_collision"),
            hash("heft"),
            hash("put_down"),
            hash("rattle_collision"),
            hash("pickup_sound"),
            u8f("unknown_00"),
            hash("safety_on_sound"),
            hash("safety_off_sound"),
            hash("slomo_swoosh_sound"),
            u32f("unknown_04"),
            hash("slomo_xfade"),
            hash("slomo_collision"),
        ],
        10 => vec![hash_list("stations", CountWidth::U8)],
        11 => vec![
            i32f("unused"),
            i32f("wheel_position"),
            u8f("genre"),
            u8f("padding_01"),
            i32f("padding_02"),
            i32f("padding_03"),
            u8f("ambient_radio_volume"),
            f("name", FieldKind::Text(CountWidth::U8)),
            hash_list("track_categories", CountWidth::U8),
        ],
        12 => vec![
            f(
                "category_type",
                FieldKind::Enum {
                    base: IntWidth::U8,
                    values: RADIO_TRACK_CATEGORIES,
                },
            ),
            u32f("padding_00"),
            u8f("padding_01"),
            hash_list("history_space", CountWidth::U8),
            u16f("padding_02"),
            u32f("padding_03"),
            arr(
                "tracks",
                CountWidth::U8,
                vec![hash("context"), hash("sound")],
            ),
        ],
        13 => vec![arr(
            "weights",
            CountWidth::U8,
            vec![
                f(
                    "category_type",
                    FieldKind::Enum {
                        base: IntWidth::U8,
                        values: RADIO_TRACK_CATEGORIES,
                    },
                ),
                i32f("value"),
            ],
        )],
        14 => vec![
            arr(
                "crime_instructions",
                CountWidth::U8,
                vec![hash("hash"), f32f("weight")],
            ),
            arr(
                "crime_descriptions",
                CountWidth::U8,
                vec![hash("hash"), f32f("weight")],
            ),
        ],
        16 => vec![
            arr(
                "voice_groups",
                CountWidth::U8,
                vec![hash("voice"), u32f("reference_count")],
            ),
            arr(
                "mini_voice_groups",
                CountWidth::U8,
                vec![hash("voice"), u32f("reference_count")],
            ),
            arr(
                "gang_voice_groups",
                CountWidth::U8,
                vec![hash("voice"), u32f("reference_count")],
            ),
        ],
        17 => vec![hash_list("ambient_emitters", CountWidth::U16)],
        19 => vec![
            f32f("min_x"),
            f32f("min_y"),
            f32f("min_z"),
            f32f("max_x"),
            f32f("max_y"),
            f32f("max_z"),
            u8f("rules_count"),
            hash_list("rules", CountWidth::U8),
        ],
        20 => vec![
            f32f("weight"),
            f32f("offset_x"),
            f32f("offset_y"),
            u8f("hours_start"),
            u8f("hours_end"),
            i16f("unknown_0f"),
            hash("sound"),
            hash("category"),
            u32f("unknown"),
        ],
        21 => vec![hash_list("zones", CountWidth::U8)],
        23 => vec![arr(
            "categories",
            CountWidth::U8,
            vec![hash("category"), u8f("intensity")],
        )],
        24 => vec![arr(
            "interior_rooms",
            CountWidth::U8,
            vec![
                hash("room_name"),
                i8f("padding"),
                f32f("reverb_large"),
                f32f("reverb_medium"),
                f32f("reverb_small"),
                hash("room_sound"),
                i8f("rain_type"),
                f32f("exterior_audibility"),
                f32f("room_occlusion_damping"),
                f32f("non_marked_portal_occlusion"),
                f32f("distance_from_portal_for_occlusion"),
                i8f("padding_2"),
                f32f("distance_from_portal_fade_distance"),
                hash("weapon_metrics"),
                hash("interior_walla_sound_set"),
            ],
        )],
        25 => vec![
            hash("brush"),
            hash("limit"),
            hash("open"),
            hash("close"),
            hash("unknown_00"),
        ],
        0 => automobile(),
        _ => return None,
    })
}

/// Vehicle engine/exhaust definition (game type 0, all fixed fields).
fn automobile() -> Vec<FieldDef> {
    vec![
        i32f("master_volume"),
        i32f("max_cone_attenuation"),
        hash("low_engine_loop"),
        hash("high_engine_loop"),
        hash("low_exhaust_loop"),
        hash("high_exhaust_loop"),
        hash("revs_off_loop"),
        f32f("engine_accel_volume"),
        f32f("exhaust_accel_volume"),
        i32f("engine_accel_min_pitch"),
        i32f("engine_accel_max_pitch"),
        f32f("engine_decel_volume"),
        f32f("exhaust_decel_volume"),
        i32f("engine_decel_min_pitch"),
        i32f("engine_decel_max_pitch"),
        f32f("engine_idle_volume"),
        f32f("exhaust_idle_volume"),
        i32f("engine_idle_min_pitch"),
        i32f("engine_idle_max_pitch"),
        f32f("engine_revs_volume"),
        f32f("exhaust_revs_volume"),
        i32f("engine_revs_min_pitch"),
        i32f("engine_revs_max_pitch"),
        i32f("throttle_resonance_volume"),
        f32f("filter_1_cutoff"),
        f32f("filter_2_cutoff"),
        i32f("resonance_min_pitch"),
        i32f("resonance_max_pitch"),
        hash("engine_synth_def"),
        i32f("engine_wave_shape_pitch"),
        hash("exhaust_synth_def"),
        i32f("exhaust_wave_shape_pitch"),
        i32f("min_pitch"),
        i32f("max_pitch"),
        hash("engine_idle_loop_sound"),
        hash("exhaust_idle_loop_sound"),
        i32f("idle_min_pitch"),
        i32f("idle_max_pitch"),
        hash("transmission_sound"),
        i32f("trans_whine_min_pitch"),
        i32f("trans_whine_max_pitch"),
        hash("induction_loop"),
        i32f("induction_min_pitch"),
        i32f("induction_max_pitch"),
        hash("exhaust_pop_sound"),
        hash("turbo_whine"),
        i32f("turbo_min_pitch"),
        i32f("turbo_max_pitch"),
        hash("dump_valve_sound"),
        hash("startup_revs"),
        hash("horn_sounds"),
        hash("door_open_sound"),
        hash("door_close_sound"),
        hash("boot_open_sound"),
        hash("boot_close_sound"),
        f32f("brake_squeek_factor"),
        hash("suspension_up_sound"),
        hash("suspension_down_sound"),
        f32f("min_susp_comp_thresh"),
        f32f("max_susp_comp_thresh"),
        hash("police_scanner_manufacturer_sound"),
        hash("police_scanner_model_sound"),
        hash("police_scanner_vehicle_category_sound"),
        hash("gear_transmission_sound"),
        i32f("gear_trans_min_pitch"),
        i32f("gear_trans_max_pitch"),
        i32f("gt_throttle_volume"),
        i32f("dump_valve_probability"),
        i32f("turbo_spin_up_speed"),
        i32f("volume_boost"),
        i32f("exhaust_boost"),
        i32f("transmission_boost"),
        hash("jump_land_sound"),
        i32f("jump_land_min_thresh"),
        i32f("jump_land_max_thresh"),
        hash("ignition_sound"),
        hash("engine_shut_down_sound"),
        i8f("volume_category"),
        i8f("gps_type"),
        i8f("radio_type"),
        i8f("radio_genre"),
        hash("indicator_on_sound"),
        hash("indicator_off_sound"),
        hash("cooling_fan_sound"),
        hash("handbrake_sound_2"),
        hash("null_sound_2"),
        hash("null_sound_3"),
        hash("handbrake_sound"),
        i16f("gps_voice"),
        i8f("radio_leakage"),
    ]
}

/// Track-category ids for radio scheduling (game types 12 and 13).
pub const RADIO_TRACK_CATEGORIES: &[(i64, &str)] = &[
    (0, "AD"),
    (1, "IDENT"),
    (2, "MUSIC"),
    (3, "NEWS"),
    (4, "WEATHER"),
    (5, "DJ_SOLO"),
    (6, "USER_INTRO"),
    (7, "USER_OUTRO"),
    (8, "USER_TO_AD"),
    (9, "USER_TO_NEWS"),
];

/// Condition operators for `audIfSound` (sounds type 22).
pub const IF_CONDITIONS: &[(i64, &str)] = &[
    (0, "LESS_THAN"),
    (1, "LESS_THAN_OR_EQUAL_TO"),
    (2, "GREATER_THAN"),
    (3, "GREATER_THAN_OR_EQUAL_TO"),
    (4, "EQUAL_TO"),
    (5, "NOT_EQUAL_TO"),
];

/// Operation ids for `audMathOperationSound` (sounds type 24).
pub const MATH_OPERATIONS: &[(i64, &str)] = &[
    (0, "ADD"),
    (1, "SUBTRACT"),
    (2, "MULTIPLY"),
    (3, "DIVIDE"),
    (4, "SET"),
    (5, "MOD"),
    (6, "MIN"),
    (7, "MAX"),
    (8, "ABS"),
    (9, "SIGN"),
    (10, "FLOOR"),
    (11, "CEIL"),
    (12, "RAND"),
    (13, "SIN"),
    (14, "COS"),
    (15, "SQRT"),
    (16, "DBTOLINEAR"),
    (17, "LINEARTODB"),
    (18, "PITCHTORATIO"),
    (19, "RATIOTOPITCH"),
    (20, "GETTIME"),
    (21, "FSEL"),
    (22, "VALUEINRANGE"),
    (23, "CLAMP"),
    (24, "POW"),
    (25, "ROUND"),
    (26, "SCALEDSIN"),
    (27, "SCALEDTRI"),
    (28, "SCALEDSAW"),
    (29, "SCALEDSQUARE"),
    (30, "SMOOTH"),
    (31, "GETSCALEDTIME"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_by_filename() {
        assert_eq!(Schema::detect("sounds.dat15"), Some(Schema::Sounds));
        assert_eq!(
            Schema::detect("EP1_RADIO_SOUNDS.DAT15"),
            Some(Schema::Sounds)
        );
        assert_eq!(Schema::detect("game.dat16"), Some(Schema::Game));
        assert_eq!(Schema::detect("curves.dat12"), Some(Schema::Curves));
        assert_eq!(Schema::detect("CATEGORIES.DAT15"), Some(Schema::Categories));
        assert_eq!(Schema::detect("effects.dat11"), Some(Schema::Effects));
        assert_eq!(Schema::detect("speech.dat"), None);
        assert_eq!(Schema::detect("nope.rpf"), None);
        assert_eq!(Schema::detect("a/b\\game.dat16"), Some(Schema::Game));
    }

    #[test]
    fn suffixes_match_files() {
        assert_eq!(Schema::Sounds.suffix(), 15);
        assert_eq!(Schema::Categories.suffix(), 15);
        assert_eq!(Schema::Curves.suffix(), 12);
        assert_eq!(Schema::Effects.suffix(), 11);
        assert_eq!(Schema::Game.suffix(), 16);
    }

    #[test]
    fn every_named_type_has_fields() {
        for schema in [
            Schema::Categories,
            Schema::Curves,
            Schema::Effects,
            Schema::Sounds,
            Schema::Game,
        ] {
            for id in 0..=25u8 {
                assert_eq!(
                    schema.type_name(id).is_some(),
                    schema.type_fields(id).is_some(),
                    "{schema:?} type {id}"
                );
            }
            assert!(schema.type_name(250).is_none());
            assert!(schema.type_fields(250).is_none());
        }
    }
}
