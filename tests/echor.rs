// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod echor {
    use anyhow::Result;
    use assert_cmd::Command;
    use predicates::prelude::*;
    use test_case::test_case;

    #[test_case(&["hello"], "hello\n" ; "single argument/no options")]
    #[test_case(&["hello"], "hello" ; "single argument/'-n' option")]
    #[test_case(&["one", "two", "three"], "one two three\n" ; "multiple argument/no options")]
    #[test_case(&["one", "two", "three", "-n"], "one two three" ; "multiple argument/'-n' option")]
    fn valid(args: &[&str], output: &str) -> Result<()> {
        Command::cargo_bin(env!("CARGO_PKG_NAME"))?
            .args(args)
            .assert()
            .stdout(predicate::str::contains(output))
            .success();

        Ok(())
    }
    #[test_case(&["-x"], "Usage:" ; "unknown short optional argument")]
    #[test_case(&["--unknown"], "Usage:" ; "unknown long argument")]
    fn invalid(args: &[&str], error: &str) -> Result<()> {
        Command::cargo_bin(env!("CARGO_PKG_NAME"))?
            .args(args)
            .assert()
            .stderr(predicate::str::contains(error))
            .failure();

        Ok(())
    }
}
