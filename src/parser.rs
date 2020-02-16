use std::io::*;
use std::path::Path;

pub fn parse_life(file_name: &str) -> std::io::Result<()> {
    let file_extension = Path::new(file_name).extension();

    if let Some(ext) = file_extension {
        println!("{:?}", ext);
    } else {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "File type not recognized",
        ));
    }

    return Ok(());
}
