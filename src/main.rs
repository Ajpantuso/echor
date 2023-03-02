// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author = clap::crate_authors!())]
#[command(version = clap::crate_version!())]
struct Options {
    #[arg()]
    text: Vec<String>,
    /// Removes newline from message.
    #[arg(short = 'n')]
    noline: bool,
}

fn main() {
    let opts = Options::parse();

    let mut joined = opts.text.join(" ");

    if !opts.noline {
        joined.push('\n');
    }

    print!("{joined}")
}
