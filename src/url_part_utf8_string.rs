use anyhow::Error;
use core::str::FromStr;
use percent_encoding::{percent_decode_str, AsciiSet, CONTROLS};
use std::string::ToString;

/// https://url.spec.whatwg.org/#fragment-percent-encode-set
#[allow(dead_code)]
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

/// the url must be utf 8. Only the 5 control characters are encoded.
/// url has parts or fragments or segments delimited mostly by slash /
/// every part must be encoded/decoded separately, 
/// to maintain the control character slash /
#[derive(Clone, Debug)]
pub struct UrlPartUtf8String {
    /// private inaccessible field - normal string - decoded
    s: String,
}

impl UrlPartUtf8String {
    /// constructor from decoded (normal) string
    #[allow(dead_code)]
    pub fn new_from_decoded_string(s: &str) -> Self {
        UrlPartUtf8String { 
            s: s.to_string()
        }
    }
    /// get encoded string
    #[allow(dead_code)]
    pub fn get_encoded_string(&self)->String{
        Self::encode_fragment(&self.s)
    }
    /// encode fragment / part - associated fn
    #[allow(dead_code)]
    pub fn encode_fragment(s: &str) -> String {
        percent_encoding::utf8_percent_encode(s, FRAGMENT).to_string()
    }
}

/// implementing FromStr because of path! in warp web server router
/// it assumes that the original string is encoded
impl FromStr for UrlPartUtf8String {
    type Err = Error;
    #[inline]
    /// constructor, decodes the string from encoded str. 
    /// It can error.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = percent_decode_str(s).decode_utf8()?.to_string();
        Ok(UrlPartUtf8String { 
            s 
        })
    }
}

impl ToString for UrlPartUtf8String {
    #[inline]
    /// returns decoded string (normal string)
    fn to_string(&self) -> String {
        // return
        self.s.clone()
    }
}