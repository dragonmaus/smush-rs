use std::{fmt, io, str};

mod codecs;

use crate::{Codec::*, Quality::*};

/// A value to represent an encoding
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Codec {
    /// The `base58` encoding.
    Base58,

    /// The `bincode` encoding.
    BinCode,

    /// The `brotli` encoding.
    Brotli,

    /// The `bzip2` encoding.
    Bzip2,

    /// The `deflate` encoding.
    Deflate,

    /// The `gzip` encoding.
    Gzip,

    /// The `lz4` encoding.
    Lz4,

    /// The `xz` encoding (also known as `lzma`).
    Xz,

    /// The `zlib` encoding.
    Zlib,

    /// The `zstd` encoding.
    Zstd,

    /// The `identity` encoding.
    Identity,

    #[doc(hidden)]
    // Silence "unreachable pattern" warnings when features are enabled.
    __Nonexhaustive,
}

impl fmt::Display for Codec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Base58 => "base58",
            BinCode => "bincode",
            Brotli => "brotli",
            Bzip2 => "bzip2",
            Deflate => "deflate",
            Gzip => "gzip",
            Lz4 => "lz4",
            Xz => "xz",
            Zlib => "zlib",
            Zstd => "zstd",
            Identity => "identity",
            __Nonexhaustive => unreachable!(),
        })
    }
}

impl str::FromStr for Codec {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let encoding = match s {
            "base58" => Base58,
            "bincode" => BinCode,
            "brotli" => Brotli,
            "bzip2" => Bzip2,
            "deflate" => Deflate,
            "gzip" => Gzip,
            "lz4" => Lz4,
            "xz" => Xz,
            "zlib" => Zlib,
            "zstd" => Zstd,
            "identity" => Identity,
            other => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("unknown encoding format: {}", other),
                ))
            }
        };

        Ok(encoding)
    }
}

/// A value to represent an encoding quality.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Quality {
    Default,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    Level8,
    Level9,
    Maximum,
}

impl fmt::Display for Quality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Default => "default",
            Level1 => "level1",
            Level2 => "level2",
            Level3 => "level3",
            Level4 => "level4",
            Level5 => "level5",
            Level6 => "level6",
            Level7 => "level7",
            Level8 => "level8",
            Level9 => "level9",
            Maximum => "maximum",
        })
    }
}

impl str::FromStr for Quality {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let quality = match s {
            "default" => Default,
            "level1" => Level1,
            "level2" => Level2,
            "level3" => Level3,
            "level4" => Level4,
            "level5" => Level5,
            "level6" => Level6,
            "level7" => Level7,
            "level8" => Level8,
            "level9" => Level9,
            "maximum" => Maximum,
            other => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("unknown quality level: {}", other),
                ))
            }
        };

        Ok(quality)
    }
}

pub fn encode(data: &[u8], codec: Codec, quality: Quality) -> io::Result<Vec<u8>> {
    match codec {
        Identity => Ok(data.to_vec()),

        #[cfg(feature = "base58_support")]
        Base58 => codecs::base58::encode(data, quality),

        #[cfg(feature = "bincode_support")]
        BinCode => codecs::bincode::encode(data, quality),

        #[cfg(feature = "brotli_support")]
        Brotli => codecs::brotli::encode(data, quality),

        #[cfg(feature = "bzip2_support")]
        Bzip2 => codecs::bzip2::encode(data, quality),

        #[cfg(feature = "deflate_support")]
        Deflate => codecs::deflate::encode(data, quality),

        #[cfg(feature = "gzip_support")]
        Gzip => codecs::gzip::encode(data, quality),

        #[cfg(feature = "lz4_support")]
        Lz4 => codecs::lz4::encode(data, quality),

        #[cfg(feature = "xz_support")]
        Xz => codecs::xz::encode(data, quality),

        #[cfg(feature = "zlib_support")]
        Zlib => codecs::zlib::encode(data, quality),

        #[cfg(feature = "zstd_support")]
        Zstd => codecs::zstd::encode(data, quality),

        disabled => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("encoding algorithm `{}` was not enabled", disabled),
        )),
    }
}

pub fn decode(data: &[u8], codec: Codec) -> io::Result<Vec<u8>> {
    match codec {
        Identity => Ok(data.to_vec()),

        #[cfg(feature = "base58_support")]
        Base58 => codecs::base58::decode(data),

        #[cfg(feature = "bincode_support")]
        BinCode => codecs::bincode::decode(data),

        #[cfg(feature = "brotli_support")]
        Brotli => codecs::brotli::decode(data),

        #[cfg(feature = "bzip2_support")]
        Bzip2 => codecs::bzip2::decode(data),

        #[cfg(feature = "deflate_support")]
        Deflate => codecs::deflate::decode(data),

        #[cfg(feature = "gzip_support")]
        Gzip => codecs::gzip::decode(data),

        #[cfg(feature = "lz4_support")]
        Lz4 => codecs::lz4::decode(data),

        #[cfg(feature = "xz_support")]
        Xz => codecs::xz::decode(data),

        #[cfg(feature = "zlib_support")]
        Zlib => codecs::zlib::decode(data),

        #[cfg(feature = "zstd_support")]
        Zstd => codecs::zstd::decode(data),

        disabled => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("encoding algorithm `{}` was not enabled", disabled),
        )),
    }
}

pub fn is_codec_enabled(codec: Codec) -> bool {
    match codec {
        Codec::Base58 => cfg!(feature = "base58_support"),
        Codec::BinCode => cfg!(feature = "bincode_support"),
        Codec::Brotli => cfg!(feature = "brotli_support"),
        Codec::Bzip2 => cfg!(feature = "bzip2_support"),
        Codec::Deflate => cfg!(feature = "deflate_support"),
        Codec::Gzip => cfg!(feature = "gzip_support"),
        Codec::Lz4 => cfg!(feature = "lz4_support"),
        Codec::Xz => cfg!(feature = "xz_support"),
        Codec::Zlib => cfg!(feature = "zlib_support"),
        Codec::Zstd => cfg!(feature = "zstd_support"),
        Codec::Identity => true,
        _disabled => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_DATA: &[u8] = include_bytes!("ipsum.txt");

    #[test]
    fn encode_identity() {
        let encoded = encode(&TEST_DATA, Codec::Identity, Quality::Default).unwrap();
        assert_eq!(&TEST_DATA, &encoded.as_slice());
    }

    #[cfg(feature = "base58_support")]
    #[test]
    fn encode_base58() {
        encode(&TEST_DATA, Codec::Base58, Quality::Default).unwrap();
    }

    #[cfg(feature = "bincode_support")]
    #[test]
    fn encode_bincode() {
        encode(&TEST_DATA, Codec::BinCode, Quality::Default).unwrap();
    }

    #[cfg(feature = "brotli_support")]
    #[test]
    fn encode_brotli() {
        encode(&TEST_DATA, Codec::Brotli, Quality::Default).unwrap();
    }

    #[cfg(feature = "bzip2_support")]
    #[test]
    fn encode_bzip2() {
        encode(&TEST_DATA, Codec::Bzip2, Quality::Default).unwrap();
    }

    #[cfg(feature = "deflate_support")]
    #[test]
    fn encode_deflate() {
        encode(&TEST_DATA, Codec::Deflate, Quality::Default).unwrap();
    }

    #[cfg(feature = "gzip_support")]
    #[test]
    fn encode_gzip() {
        encode(&TEST_DATA, Codec::Gzip, Quality::Default).unwrap();
    }

    #[cfg(feature = "lz4_support")]
    #[test]
    fn encode_lz4() {
        encode(&TEST_DATA, Codec::Lz4, Quality::Default).unwrap();
    }

    #[cfg(feature = "xz_support")]
    #[test]
    fn encode_xz() {
        encode(&TEST_DATA, Codec::Xz, Quality::Default).unwrap();
    }

    #[cfg(feature = "zlib_support")]
    #[test]
    fn encode_zlib() {
        encode(&TEST_DATA, Codec::Zlib, Quality::Default).unwrap();
    }

    #[cfg(feature = "zstd_support")]
    #[test]
    fn encode_zstd() {
        encode(&TEST_DATA, Codec::Zstd, Quality::Default).unwrap();
    }

    #[test]
    fn decode_identity() {
        let encoded = encode(&TEST_DATA, Codec::Identity, Quality::Default).unwrap();
        assert_eq!(&encoded, &TEST_DATA);
        let decoded = decode(&encoded, Codec::Identity).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "base58_support")]
    #[test]
    fn decode_base58() {
        let encoded = encode(&TEST_DATA, Codec::Base58, Quality::Default).unwrap();
        let decoded = decode(&encoded, Codec::Base58).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "bincode_support")]
    #[test]
    fn decode_bincode() {
        let encoded = encode(&TEST_DATA, Codec::BinCode, Quality::Default).unwrap();
        let decoded = decode(&encoded, Codec::BinCode).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "brotli_support")]
    #[test]
    fn decode_brotli() {
        let encoded = encode(&TEST_DATA, Codec::Brotli, Quality::Default).unwrap();
        let decoded = decode(&encoded, Codec::Brotli).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "bzip2_support")]
    #[test]
    fn decode_bzip2() {
        let encoded = encode(&TEST_DATA, Codec::Bzip2, Quality::Default).unwrap();
        let decoded = decode(&encoded, Codec::Bzip2).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "deflate_support")]
    #[test]
    fn decode_deflate() {
        let encoded = encode(&TEST_DATA, Codec::Deflate, Quality::Default).unwrap();
        let decoded = decode(&encoded, Codec::Deflate).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "gzip_support")]
    #[test]
    fn decode_gzip() {
        let encoded = encode(&TEST_DATA, Codec::Gzip, Quality::Default).unwrap();
        let decoded = decode(&encoded, Codec::Gzip).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "lz4_support")]
    #[test]
    fn decode_lz4() {
        let encoded = encode(&TEST_DATA, Codec::Lz4, Quality::Default).unwrap();
        let decoded = decode(&encoded, Codec::Lz4).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "xz_support")]
    #[test]
    fn decode_xz() {
        let encoded = encode(&TEST_DATA, Codec::Xz, Quality::Default).unwrap();
        let decoded = decode(&encoded, Codec::Xz).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "zlib_support")]
    #[test]
    fn decode_zlib() {
        let encoded = encode(&TEST_DATA, Codec::Zlib, Quality::Default).unwrap();
        let decoded = decode(&encoded, Codec::Zlib).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "zstd_support")]
    #[test]
    fn decode_zstd() {
        let encoded = encode(&TEST_DATA, Codec::Zstd, Quality::Default).unwrap();
        let decoded = decode(&encoded, Codec::Zstd).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }
}
