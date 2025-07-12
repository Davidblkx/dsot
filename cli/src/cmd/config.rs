use anyhow::Ok;

static NAME: &str = "config";

pub struct ConfigCmd {}

impl ConfigCmd {
    pub fn get_name() -> &'static str {
        NAME
    }

    pub fn build() -> clap::Command {
        clap::Command::new(NAME)
            .about("Read configuration values")
            .arg(
                clap::Arg::new("key")
                    .help("The configuration key to read")
                    .required(true),
            )
    }

    pub fn run(runtime: &dsot_runtime::Runtime, args: &clap::ArgMatches) -> anyhow::Result<()> {
        // TODO: Implement the logic to write configuration values
        if let Some(key) = args.get_one::<String>("key") {
            let value = runtime.config.get_config_value(key);

            match value {
                bakunin_config::Value::None => println!("No value found for '{}'", key),
                _ => println!("{:?}", value),
            }
        } else {
            println!("No key provided");
        }

        Ok(())
    }
}
