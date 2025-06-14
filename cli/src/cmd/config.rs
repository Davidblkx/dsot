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
                    .required(true)
            )
    }

    pub fn run(runtime: &dsot_runtime::Runtime, args: &clap::ArgMatches) -> anyhow::Result<()> {
        if let Some(key) = args.get_one::<String>("key") {
            let value = runtime.config.read_raw_config(key);
            if let Some(val) = value {
                println!("{:?}", val);
            } else {
                println!("No value found for '{}'", key);
            }
        } else {
            println!("No key provided");
        }

        Ok(())
    }
}
