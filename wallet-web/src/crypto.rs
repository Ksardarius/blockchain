use base64::{Engine, engine::general_purpose};
use sha2::{Digest, Sha256};
use wasm_bindgen::JsValue;
use web_sys::window;

// Function to derive a simple fixed-size key from a password string
// In a real app, use PBKDF2 or Argon2 with a random salt
fn derive_key_from_password(password: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.finalize().into()
}

#[allow(dead_code)]
pub async fn encrypt_hex_data_aes_gcm(
    data_bytes_hex: String,
    password: &str,
) -> Result<String, JsValue> {
    let data_bytes = hex::decode(data_bytes_hex).map_err(|e| format!("Invalid hex: {}", e))?;
    encrypt_data_aes_gcm(data_bytes, password).await
}

pub async fn encrypt_data_aes_gcm(data_bytes: Vec<u8>, password: &str) -> Result<String, JsValue> {
    let window = window().ok_or("No window")?;
    let crypto = window.crypto().map_err(|_| "No crypto object")?;

    let subtle_crypto = crypto.subtle();

    let derived_key = derive_key_from_password(&password);

    // Import key for AES-GCM encryption
    let key_format = js_sys::Object::new();
    js_sys::Reflect::set(&key_format, &"name".into(), &"AES-GCM".into())?;

    let usages = js_sys::Array::new_with_length(1);
    usages.set(0, "encrypt".into());

    let imported_key_promise = subtle_crypto.import_key_with_object(
        "raw".into(),
        &js_sys::Uint8Array::from(derived_key.as_ref()),
        &key_format,
        false.into(), // extractable
        &usages,
    )?;

    let imported_key = wasm_bindgen_futures::JsFuture::from(imported_key_promise).await?;

    // Generate a random 12-byte nonce (IV)
    let mut iv_bytes = [0u8; 12];
    crypto
        .get_random_values_with_u8_array(&mut iv_bytes)
        .map_err(|_| "Failed to get random values for IV")?;
    let iv_js = js_sys::Uint8Array::from(iv_bytes.as_ref());

    // Define AES-GCM parameters
    let algo_params = js_sys::Object::new();
    js_sys::Reflect::set(&algo_params, &"name".into(), &"AES-GCM".into())?;
    js_sys::Reflect::set(&algo_params, &"iv".into(), &iv_js)?;

    // Encrypt the data
    let encrypted_buffer =
        wasm_bindgen_futures::JsFuture::from(subtle_crypto.encrypt_with_object_and_u8_array(
            &algo_params,
            &imported_key.into(),
            data_bytes.as_ref(),
        )?)
        .await?;

    let encrypted_data = js_sys::Uint8Array::new(&encrypted_buffer);
    let encrypted_data_vec = encrypted_data.to_vec();

    let encoded_data = general_purpose::STANDARD.encode(&encrypted_data_vec);
    let encoded_iv = general_purpose::STANDARD.encode(&iv_bytes);

    Ok(format!("{},{}", encoded_data, encoded_iv))
}

#[allow(dead_code)]
pub async fn decrypt_hex_data_aes_gcm(
    encrypted_payload: String,
    password: &str,
) -> Result<String, JsValue> {
    let decrypted_data_vec = decrypt_data_aes_gcm(encrypted_payload, password).await?;
    Ok(hex::encode(&decrypted_data_vec))
}

pub async fn decrypt_data_aes_gcm(
    encrypted_payload: String,
    password: &str,
) -> Result<Vec<u8>, JsValue> {
    let parts: Vec<&str> = encrypted_payload.split(',').collect();
    if parts.len() != 2 {
        return Err(JsValue::from_str("Invalid encrypted payload format."));
    }
    let encoded_data = parts[0];
    let encoded_iv = parts[1];

    let encrypted_data_bytes = general_purpose::STANDARD
        .decode(encoded_data)
        .map_err(|e| format!("Invalid base64 data: {}", e))?;
    let iv_bytes = general_purpose::STANDARD
        .decode(encoded_iv)
        .map_err(|e| format!("Invalid base64 IV: {}", e))?;

    let window = window().ok_or("No window")?;
    let crypto = window.crypto().map_err(|_| "No crypto object")?;
    let subtle_crypto = crypto.subtle();

    let derived_key = derive_key_from_password(password);

    // Import key for AES-GCM decryption
    let key_format = js_sys::Object::new();
    js_sys::Reflect::set(&key_format, &"name".into(), &"AES-GCM".into())?;

    let usages = js_sys::Array::new_with_length(1);
    usages.set(0, "decrypt".into());

    let imported_key = wasm_bindgen_futures::JsFuture::from(subtle_crypto.import_key_with_object(
        "raw".into(),
        &js_sys::Uint8Array::from(derived_key.as_ref()),
        &key_format,
        false.into(), // extractable
        &usages,
    )?)
    .await?;

    // Define AES-GCM parameters for decryption
    let algo_params = js_sys::Object::new();
    js_sys::Reflect::set(&algo_params, &"name".into(), &"AES-GCM".into())?;
    js_sys::Reflect::set(
        &algo_params,
        &"iv".into(),
        &js_sys::Uint8Array::from(iv_bytes.as_ref()),
    )?;

    // Decrypt the data
    let decrypted_buffer =
        wasm_bindgen_futures::JsFuture::from(subtle_crypto.decrypt_with_object_and_u8_array(
            &algo_params,
            &imported_key.into(),
            encrypted_data_bytes.as_ref(),
        )?)
        .await?;

    let decrypted_data = js_sys::Uint8Array::new(&decrypted_buffer);
    Ok(decrypted_data.to_vec())
}

#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod tests {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    use std::result;

    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn test_encrypt_data_aes_gcm() {
        let result = encrypt_data_aes_gcm("f9b4ca".to_string(), "aaaaaa".to_string()).await;
        assert!(result.is_ok(), "Expected success, got {:?}", result);
    }

    #[wasm_bindgen_test]
    async fn test_decrypt_data_aes_gcm() {
        let result = decrypt_data_aes_gcm(
            "Ff+nIxh4OaJmK6SujFqCBuIFBQ==,T+XzrpmkvbLD8/iY".to_string(),
            "aaaaaa".to_string(),
        )
        .await;
        assert!(result.is_ok(), "Expected success, got {:?}", result);
        assert_eq!(result.unwrap(), "f9b4ca");
    }

    #[wasm_bindgen_test]
    async fn test_encryption_with_password() {
        let data = "f9b4cbbce2".to_string();
        let result = encrypt_data_aes_gcm(data.clone(), "password".to_string())
            .await
            .unwrap();
        let decoded_data = decrypt_data_aes_gcm(result.into(), "password".to_string())
            .await
            .unwrap();
        assert_eq!(data, decoded_data);
    }
}
