use std::io::Read;

use anyhow::Result;
use base64::{
    prelude::{BASE64_STANDARD, BASE64_URL_SAFE_NO_PAD},
    Engine,
};

use crate::Base64Format;

pub fn process_encode(reader: &mut dyn Read, format: Base64Format) -> Result<String> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let encoded = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(&buf),
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.encode(&buf),
    };
    Ok(encoded)
}

pub fn process_decode(reader: &mut dyn Read, format: Base64Format) -> Result<String> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(buf)?,
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.decode(buf)?,
    };
    Ok(String::from_utf8(decoded)?)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::{get_reader, process_encode, Base64Format};

    #[test]
    fn test_process_encode() -> Result<()> {
        let input = "Cargo.toml";
        let mut reader = get_reader(input)?;
        let format = Base64Format::Standard;
        assert!(process_encode(&mut reader, format).is_ok());
        Ok(())
    }

    // #[test]
    // fn test_process_decode() -> Result<()> {
    //     let input = "temp.txt";
    //     let mut reader = get_reader(input)?;
    //     let format = Base64Format::UrlSafe;
    //     process_decode(&mut reader, format)?;
    //     Ok(())
    // }
}
