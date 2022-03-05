use std::{io::{BufReader, BufRead, ErrorKind}, error::Error, process::Command};



pub fn run(cmd: duct::Expression) -> Result<(), Box<dyn Error>> {
    let stdout = cmd.reader()?;
    let lines = BufReader::new(stdout).lines();

    for line in lines {
        println!("{}", line?);
    }

    Ok(())
}

/// Detects if a binary executable is available on the path
pub fn has_binary(b: impl AsRef<str>) -> Result<bool, Box<dyn Error>> {
    match Command::new(b.as_ref()).output() {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(false),
        Err(e) => Err(Box::new(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_detect_binary() {
        assert!(has_binary("git").unwrap());
        assert!(!has_binary("sdknvaiecoicaosdvaiefw").unwrap());
    }
}
