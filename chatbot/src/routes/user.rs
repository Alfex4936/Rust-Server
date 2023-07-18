use crate::db::models::User;
use crate::db::query::{get_user, save_user, update_user_mode};
use crate::routes::DbPool;

use crate::routes::Kakao;
use actix_web::{post, web, Responder};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use hex;
use kakao_rs::prelude::*;

use serde_json::Value;

const SECRET_KEY: &str = env!("SECRET_KEY");
const NONCE: &str = env!("NONCE");

fn secret_key_from_hex(hex_str: &str) -> Result<[u8; 32], String> {
    if hex_str.len() != 64 {
        return Err("Invalid secret key length".to_string());
    }

    let mut key_array = [0u8; 32];
    for i in 0..32 {
        key_array[i] = u8::from_str_radix(&hex_str[i * 2..i * 2 + 2], 16)
            .map_err(|e| format!("Failed to parse hex string: {}", e))?;
    }
    Ok(key_array)
}

fn encrypt_key(key: &str, encryption_key: &[u8; 32]) -> Result<Vec<u8>, String> {
    let cipher = Aes256Gcm::new(encryption_key.into());
    let nonce = Nonce::from_slice(NONCE.as_bytes());
    let nonce_len = nonce.len(); // Get the length of the nonce

    cipher
        .encrypt(nonce, key.as_ref())
        .map(|ciphertext| {
            let mut encrypted_key = Vec::with_capacity(nonce_len + ciphertext.len());
            encrypted_key.extend_from_slice(nonce.as_ref());
            encrypted_key.extend_from_slice(&ciphertext);
            encrypted_key
        })
        .map_err(|e| format!("Failed to encrypt key: {}", e))
}

fn decrypt_key(encrypted_key: &[u8], encryption_key: &[u8; 32]) -> Result<String, String> {
    let cipher = Aes256Gcm::new(encryption_key.into());
    let nonce = Nonce::from_slice(NONCE.as_bytes());
    let nonce_len = nonce.len(); // Get the length of the nonce

    cipher
        .decrypt(nonce, &encrypted_key[nonce_len..])
        .map_err(|e| format!("Failed to decrypt key: {}", e))
        .and_then(|decrypted_bytes| {
            String::from_utf8(decrypted_bytes)
                .map_err(|e| format!("Failed to convert decrypted bytes to string: {}", e))
        })
}

#[post("/save")]
pub async fn save_me(kakao: web::Json<Value>, conn: DbPool) -> impl Responder {
    let id = kakao["userRequest"]["user"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    let mut openai_key = kakao["action"]["params"]["openai"]
        .as_str()
        .unwrap()
        .to_string();
    openai_key.retain(|c| !c.is_whitespace());

    let encryption_key = secret_key_from_hex(SECRET_KEY).unwrap();

    let hashed_key = match encrypt_key(&openai_key, &encryption_key) {
        Ok(hashed) => hex::encode(hashed),
        Err(err_msg) => {
            let mut result = Template::new();
            result.add_qr(QuickReply::new("내 정보 저장", "유저"));
            result.add_output(SimpleText::new(format!("API 키 해쉬 실패: {}", err_msg)).build());
            return Kakao { template: result };
        }
    };

    let mut result = Template::new();
    result.add_qr(QuickReply::new("내 정보 저장", "유저"));

    match save_user(
        &conn,
        &User {
            id,
            openai_key: hashed_key,
            mode: "gpt-3.5-turbo".to_string(),
        },
    )
    .await
    {
        Ok(saved_user) => {
            let decrypted_key = decrypt_key(
                &hex::decode(&saved_user.openai_key).unwrap(),
                &encryption_key,
            )
            .unwrap();

            result.add_output(
                SimpleText::new(format!(
                    "내 ID: {}\n\n내 API KEY{}\nMODE: {}",
                    saved_user.id, decrypted_key, saved_user.mode
                ))
                .build(),
            );
        }
        Err(_) => {
            result.add_output(SimpleText::new("유저 정보 저장 실패했습니다. :(").build());
        }
    };

    Kakao { template: result }
}

#[post("/setting")]
pub async fn update_me(kakao: web::Json<Value>, conn: DbPool) -> impl Responder {
    let id = kakao["userRequest"]["user"]["id"].as_str().unwrap();
    let mode = kakao["action"]["params"]["mode"]
        .as_str()
        .unwrap()
        .to_lowercase(); // Can be "GPT-3", "GPT-4", "GPT3", "GPT4" otherwise gpt3
    let mode = mode.replace(' ', "");
    let mode = match mode.as_str() {
        "gpt-3" | "gpt3" => "gpt-3.5-turbo",
        "gpt-4" | "gpt4" => "gpt-4",
        _ => "gpt-3.5-turbo",
    };

    let mut result = Template::new();
    result.add_qr(QuickReply::new("내 GPT MODE", "GPT"));

    match get_user(&conn, id).await {
        Ok(user) => {
            if let Some(mut user) = user {
                // Update the user's mode
                user.mode = mode.to_string();
                match update_user_mode(&conn, &user).await {
                    Ok(_) => result.add_output(
                        SimpleText::new(format!("GPT 모드가 {}로 변경되었습니다.", mode)).build(),
                    ),
                    Err(_) => {
                        result.add_output(SimpleText::new("GPT 모드 변경 실패했습니다. :(").build())
                    }
                }
            } else {
                result.add_qr(QuickReply::new("내 정보 저장", "유저"));
                result.add_output(SimpleText::new("정보를 먼저 저장해주세요.").build());
            }
        }
        Err(_) => {
            result.add_output(SimpleText::new("유저 정보 조회 실패했습니다. :(").build());
        }
    }

    Kakao { template: result }
}

pub fn init_user(cfg: &mut web::ServiceConfig) {
    cfg.service(save_me);
    cfg.service(update_me);
}
