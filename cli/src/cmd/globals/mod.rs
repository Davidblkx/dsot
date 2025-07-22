declare_arg_path!(ConfigArg, "config", short: 'c', "Use a custom configuration file, instead of the default ones");
declare_arg_path!(LayerConfigArg, "layer-config", short: 'l', "Add a custom layer configuration file, on top of the default ones");
declare_arg_bool!(DebugArg, "debug", short: 'd', "Enable debug mode");
declare_arg_bool!(
    DebugFolderArg,
    "debug-folder",
    "Enable debug mode and write logs to a folder"
);

pub fn register(cmd: clap::Command) -> clap::Command {
    cmd.arg(ConfigArg::build())
        .arg(LayerConfigArg::build())
        .arg(DebugArg::build())
        .arg(DebugFolderArg::build())
}
