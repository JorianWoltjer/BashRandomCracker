use clap::{Parser, Subcommand, ValueEnum};

/// Bash $RANDOM Cracker
#[derive(Parser, Debug)]
#[command(name = "bashrand")]
pub struct Args {
    #[command(subcommand)]
    pub command: SubCommands,

    /// Which bash version to use for generation (check with `bash --version`)
    #[arg(value_enum, global = true, short, long, default_value = "both")]
    pub version: Version,

    /// Number of values to generate
    #[arg(global = true, short, long, default_value = "10")]
    pub number: usize,
}

#[derive(Subcommand, Debug)]
pub enum SubCommands {
    /// Provide random numbers to brute-force the seed
    Crack {
        /// 2-3 $RANDOM numbers as input for brute-forcing the seed
        ///
        /// 2 => multiple possible seeds, 3 => single seed
        #[clap(num_args = 2..=3)]
        numbers: Vec<String>,
    },

    /// Get random numbers from a seed
    Get {
        /// Seed to use for generating random numbers
        seed: u32,

        /// Skip the first n numbers
        #[arg(short, long, default_value = "0")]
        skip: usize,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Version {
    /// Bash versions 5.0 and older
    Old,

    /// Bash versions 5.1 and newer
    New,

    /// Try both old and new versions if unsure
    Both,
}
