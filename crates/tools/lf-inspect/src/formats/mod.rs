//! One module per format crate. Each exposes `COMMANDS` (its subcommands,
//! `summarize` first) and `run(command, args, io)`.

pub mod anim;
pub mod archive;
pub mod audio_bank;
pub mod audio_config;
pub mod collision;
pub mod cutscene;
pub mod effects;
pub mod entity_meta;
pub mod gamedata;
pub mod mapdata;
pub mod model;
pub mod nav;
pub mod resource;
pub mod save;
pub mod sco;
pub mod shaderpack;
pub mod text;
pub mod texture;

use crate::cli::{CliResult, Io};

/// A format's entry point: `run(command, args, io)`.
pub type RunFn = fn(&str, &[String], &mut Io) -> CliResult;

/// One row of the format table.
pub struct Format {
    /// Name used on the command line.
    pub name: &'static str,
    /// What the format covers, for `lf-inspect formats`.
    pub about: &'static str,
    /// Subcommands; the first is the default.
    pub commands: &'static [&'static str],
    /// Entry point.
    pub run: RunFn,
}

/// Every format, in the order `lf-inspect formats` lists them.
pub const FORMATS: &[Format] = &[
    Format {
        name: "archive",
        about: "RPF and IMG archives (.rpf, .img)",
        commands: archive::COMMANDS,
        run: archive::run,
    },
    Format {
        name: "resource",
        about: "RSC5 resource containers",
        commands: resource::COMMANDS,
        run: resource::run,
    },
    Format {
        name: "texture",
        about: "texture dictionaries (.wtd)",
        commands: texture::COMMANDS,
        run: texture::run,
    },
    Format {
        name: "model",
        about: "drawables, dictionaries, fragments (.wdr, .wdd, .wft)",
        commands: model::COMMANDS,
        run: model::run,
    },
    Format {
        name: "collision",
        about: "collision bounds (.wbn, .wbd)",
        commands: collision::COMMANDS,
        run: collision::run,
    },
    Format {
        name: "nav",
        about: "navigation meshes and path graphs (.wnv, .nod, paths.ipl)",
        commands: nav::COMMANDS,
        run: nav::run,
    },
    Format {
        name: "text",
        about: "GXT text, fonts and front-end files",
        commands: text::COMMANDS,
        run: text::run,
    },
    Format {
        name: "gamedata",
        about: "gameplay data files (handling, IDE, time cycle and more)",
        commands: gamedata::COMMANDS,
        run: gamedata::run,
    },
    Format {
        name: "mapdata",
        about: "map files (.ide, .ipl, .wpl, load lists)",
        commands: mapdata::COMMANDS,
        run: mapdata::run,
    },
    Format {
        name: "sco",
        about: "compiled scripts (.sco)",
        commands: sco::COMMANDS,
        run: sco::run,
    },
    Format {
        name: "shaderpack",
        about: "compiled shader containers (.fxc)",
        commands: shaderpack::COMMANDS,
        run: shaderpack::run,
    },
    Format {
        name: "audio-config",
        about: "versioned audio metadata and speech files",
        commands: audio_config::COMMANDS,
        run: audio_config::run,
    },
    Format {
        name: "save",
        about: "save game files",
        commands: save::COMMANDS,
        run: save::run,
    },
    Format {
        name: "anim",
        about: "animation dictionaries (.wad)",
        commands: anim::COMMANDS,
        run: anim::run,
    },
    Format {
        name: "audio-bank",
        about: "audio archives, banks and streamed sounds",
        commands: audio_bank::COMMANDS,
        run: audio_bank::run,
    },
    Format {
        name: "cutscene",
        about: "cutscene descriptions (.cut)",
        commands: cutscene::COMMANDS,
        run: cutscene::run,
    },
    Format {
        name: "effects",
        about: "effect packages, Fx tables and emitter XML",
        commands: effects::COMMANDS,
        run: effects::run,
    },
    Format {
        name: "entity-meta",
        about: "vehicle, weapon and ped data in models",
        commands: entity_meta::COMMANDS,
        run: entity_meta::run,
    },
];

/// Look a format up by its command-line name.
#[must_use]
pub fn find(name: &str) -> Option<&'static Format> {
    FORMATS.iter().find(|f| f.name == name)
}
