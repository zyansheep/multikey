#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AsymmType {
	//Asymmetric Keys
	Secp256k1,
	NistP256,
	NistP384,
	RsaPkcsv1_5,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SymmType {
	AesGcmSiv,
	AesGcm,
	AesSiv,
	CCM,
	ChaCha20Poly1305,
	Curve25519XSalsa20Poly1305,
	MGM,
	XSalsa20Poly1205,
}


impl AsymmType {
	pub fn byte_translation(&self) -> u8 {
		use AsymmType::*;
		match *self {
			Secp256k1 			=> 0,
			NistP256			=> 1,
			NistP384 			=> 2,
			RsaPkcsv1_5 		=> 3,
		}
	}
}
impl SymmType {
	pub fn byte_translation(&self) -> u8 {
		use SymmType::*;
		match *self {
			AesGcmSiv 			=> 0,
			AesGcm				=> 1,
			AesSiv				=> 2,
			CCM 				=> 3,
			ChaCha20Poly1305	=> 4,
			Curve25519XSalsa20Poly1305 => 5,
			MGM 				=> 6,
			XSalsa20Poly1205 	=> 7,
		}
	}
}

pub struct Multikey {
	key: Vec<u8>
}

pub fn multikey(key_type: SymmType) {

}
pub fn multikeypair(key_type: AsymmType) {

}


pub trait AsymmGenerator {
	fn generate() -> (Vec<u8>, Vec<u8>);
}
pub trait SymmGenerator {
	fn generate() -> Vec<u8>;
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
