use hyper::Request;
use hyper::header::{HeaderName, HeaderValue};
use ring::signature::RsaKeyPair; // {,UnparsedPublicKey, RSA_PKCS1_2048_8192_SHA256};
use ring::error::KeyRejected;
use crate::signatures::request::{self, SigningError};

const SIGNATURE_HEADER: &'static str = "x-bunq-client-signature";

pub struct SigningKey {
  key: RsaKeyPair,
}

pub enum SigningKeyError {
  KeyRejected(KeyRejected),
  ModulusRejected(usize),
  NotFound,
  NotReadable,
}

impl From<KeyRejected> for SigningKeyError {
  fn from(e: KeyRejected) -> Self { Self::KeyRejected(e) }
}

impl SigningKey {
  pub fn from_pkcs8(pkcs8: &[u8]) -> Result<Self, SigningKeyError> {
    let key = RsaKeyPair::from_pkcs8(pkcs8)?;
    let size = key.public_modulus_len();
    if size == 256 { // 2048 bits = 256 bytes
      Ok(Self { key })
    } else {
      Err(SigningKeyError::ModulusRejected(size))
    }
  }
  pub fn sign_request(&self, mut request: Request<String>) -> Result<Request<String>, SigningError> {
    let sig = request::sign(&request, &self.key)?;
    let name = HeaderName::from_static(SIGNATURE_HEADER);
    let val = HeaderValue::from_str(&sig)?;
    request.headers_mut().insert(name, val);
    Ok(request)
  }
}

// pub fn verify_response(&self, mut response: Response<String>) -> bool {
// }

