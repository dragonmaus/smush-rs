fn quality_to_codec(quality: crate::Quality) -> i32 {
    match quality {
        crate::Quality::Default => 0,
        crate::Quality::Level1 => 1,
        crate::Quality::Level2 => 2,
        crate::Quality::Level3 => 3,
        crate::Quality::Level4 => 4,
        crate::Quality::Level5 => 5,
        crate::Quality::Level6 => 6,
        crate::Quality::Level7 => 7,
        crate::Quality::Level8 => 8,
        crate::Quality::Level9 => 9,
        crate::Quality::Maximum => 11, // Goes to 21, but significantly more time (i.e. ~164ms vs 0.5ms at default)
    }
}

pub fn encode(data: &[u8], quality: crate::Quality) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut writer = std::io::Cursor::new(&mut buf);
    let mut encoder = zstd::stream::Encoder::new(&mut writer, quality_to_codec(quality))?;
    std::io::copy(&mut std::io::Cursor::new(data), &mut encoder)?;
    encoder.finish().map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("failed to encode with zstd - details: {:?}", err),
        )
    })?;
    Ok(buf)
}

pub fn decode(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut reader = std::io::Cursor::new(data);
    let mut writer = std::io::Cursor::new(&mut buf);
    zstd::stream::copy_decode(&mut reader, &mut writer).map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("failed to decode with zstd - details: {:?}", err),
        )
    })?;
    Ok(buf)
}
