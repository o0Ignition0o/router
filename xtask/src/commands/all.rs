use anyhow::Result;
use structopt::StructOpt;

use super::{Compliance, Lint, Test};

#[derive(Debug, StructOpt)]
pub struct All {
    #[structopt(flatten)]
    compliance: Compliance,
    #[structopt(flatten)]
    lint: Lint,
    #[structopt(flatten)]
    test: Test,
}

impl All {
    pub fn run(&self) -> Result<()> {
        eprintln!("Checking format and clippy...");
        self.lint.run()?;
        eprintln!("Checking licenses...");
        self.compliance.run()?;
        eprintln!("Running tests...");
        self.test.run()
    }
}