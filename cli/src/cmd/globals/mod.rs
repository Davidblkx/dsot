declare_arg_path!(
    ConfigArg,
    "config",
    "Use a custom configuration file, instead of the default ones",
    'c'
);
declare_arg_path!(
    LayerConfigArg,
    "layer-config",
    "Add a custom layer configuration file, on top of the default ones"
);

declare_arg_bool!(DebugArg, "debug", "Enable debug mode", 'd');
declare_arg_bool!(
    DebugFolderArg,
    "debug-folder",
    "Enable debug mode and write logs to a folder"
);

declare_arg_bool!(
    QuietModeArg,
    "quiet",
    "Don't print anything to console",
    'q'
);
declare_arg!(DisableAnsiArg("no-ansi") {
    help: "Do not print ansi codes",
    flag: SetFalse,
});

pub fn register(cmd: clap::Command) -> clap::Command {
    cmd.arg(ConfigArg::build())
        .arg(LayerConfigArg::build())
        .arg(DebugArg::build())
        .arg(DebugFolderArg::build())
        .arg(QuietModeArg::build())
        .arg(DisableAnsiArg::build())
}
