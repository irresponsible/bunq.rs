use hyper::Request;
use hyper::header::{ToStrError, HeaderMap, HeaderName, InvalidHeaderValue, CACHE_CONTROL, USER_AGENT};
use im::ordmap::OrdMap;
use base64;
use inflector::cases::traincase::to_train_case;
use ring::signature::{RsaKeyPair, RSA_PKCS1_SHA256};
use ring::rand;
use std::convert::From;
use std::array::FixedSizeArray;

pub enum SigningError {
  BadKeyModulus(usize),
  InvalidHeaderValue(InvalidHeaderValue),
  HeaderValueInvalid(ToStrError),
  OutOfMemory,
}

impl From<InvalidHeaderValue> for SigningError {
  fn from(e: InvalidHeaderValue) -> Self { Self::InvalidHeaderValue(e) }
}

impl From<ToStrError> for SigningError {
  fn from(e: ToStrError)-> Self { Self::HeaderValueInvalid(e) }
}

struct SigningHeaders {
  headers: OrdMap<String, String>,
  size: usize,
}

impl SigningHeaders {
  fn is_header_signable(name: &HeaderName) -> bool {
    false // easily-optimised readability hack
      || (name == CACHE_CONTROL)
      || (name == USER_AGENT)
      || name.as_str().starts_with("x-bunq-")
  }

  /**
   * We use an ordmap to have them pop out in order while doing the
   * hard work ahead of time.
   */
  pub fn from_header_map(header_map: &HeaderMap) -> Result<SigningHeaders, SigningError> {
    let mut headers: OrdMap<String, String> = OrdMap::new();
    let mut size: usize = 1; // extra newline
    for (name, val) in header_map.iter() {
      if Self::is_header_signable(name) {
        let k = to_train_case(name.as_str());
        let v = val.to_str()?;
        let header_size: usize = k.len() + v.len() + 3; // colon, space, newline
        let mut header = String::with_capacity(header_size);
        header.push_str(&k);
        header.push_str(": ");
        header.push_str(&v);
        header.push('\n');
        size += header_size;
        headers.insert(k, header);
      }
    }
    Ok(SigningHeaders { headers, size })
  }
}


fn summarise(request: &Request<String>) -> Result<String, SigningError> {
  let method = request.method().as_str();
  let uri = request.uri();
  let path = uri
    .path_and_query()
    .map(|pq| pq.as_str())
    .unwrap_or(uri.path());
  let headers = SigningHeaders::from_header_map(request.headers())?;
  let size: usize =
    method.len()
    + path.len()
    + 2 // space, newline for first line
    + headers.size;
  let mut data = String::with_capacity(size);
  data.push_str(&method);
  data.push(' ');
  data.push_str(path);
  data.push('\n');
  for header in headers.headers.values() {
    data.push_str(&header);
  }
  data.push('\n');
  data.push_str(request.body());
  Ok(data)
}

/**
 * Sign a given piece of data with an RSA Private Key
 */
pub fn sign(request: &Request<String>, key: &RsaKeyPair) -> Result<String, SigningError> {
  let len = key.public_modulus_len();
  if len == 256 { // 2048 bits, the bunq-mandated size
    let summary = summarise(request)?;
    let mut buf: [u8; 256] = [0; 256];
    let rng = rand::SystemRandom::new();
    key.sign(&RSA_PKCS1_SHA256, &rng, summary.as_bytes(), &mut buf)
      .map_err(|_| SigningError::OutOfMemory)?;
    Ok(base64::encode(&buf.as_slice()))
  } else { Err(SigningError::BadKeyModulus(len)) }
}
