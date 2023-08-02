use serde_encrypt::{
    shared_key::SharedKey, traits::SerdeEncryptSharedKey, EncryptedMessage, Error,
};

use crate::database::model::EncodedObject;

pub fn encrypt(data: EncodedObject) -> Result<Vec<u8>, Error> {
    let encrypted_message = data.encrypt(&SharedKey::new([0u8; 32]))?;
    let serialized_encrypted_message: Vec<u8> = encrypted_message.serialize();
    println!("serialized = {:?}", serialized_encrypted_message);
    Ok(serialized_encrypted_message)
    // Ok(String::from_utf8(serialized_encrypted_message).unwrap())
}

pub fn decrypt(serialized_encrypted_message: Vec<u8>) -> Result<EncodedObject, Error> {
    let encrypted_message = EncryptedMessage::deserialize(serialized_encrypted_message)?;
    Ok(EncodedObject::decrypt_owned(
        &encrypted_message,
        &&SharedKey::new([0u8; 32]),
    )?)
}
