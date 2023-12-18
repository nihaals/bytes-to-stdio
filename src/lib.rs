use std::{
    io::{self, Seek, SeekFrom, Write},
    process::Stdio,
};

pub fn bytes_to_stdio(bytes: &[u8]) -> io::Result<Stdio> {
    let mut temp_file = tempfile::NamedTempFile::new()?;
    temp_file.write_all(bytes)?;
    let mut file = temp_file.into_file();
    file.seek(SeekFrom::Start(0))?;
    Ok(Stdio::from(file))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process;

    #[test]
    fn test_bytes_to_stdio() {
        let bytes = "Hello, world!".as_bytes();
        let stdio = bytes_to_stdio(bytes).unwrap();
        let output = process::Command::new("cat").stdin(stdio).output().unwrap();
        assert!(output.status.success());
        assert_eq!(output.stdout, bytes);
    }
}
