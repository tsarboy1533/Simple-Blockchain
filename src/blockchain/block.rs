use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use chrono::Utc;
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey, Signature};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub timestamp: i64,
    pub previous_hash: String,
    pub transactions: Vec<Transaction>, // ここを String から変更
    pub nonce: u64,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,    // 送信者（アドレスや名前）
    pub receiver: String,  // 受信者
    pub amount: f64,       // 金額
    pub signature: Option<Vec<u8>>, // 署名データ（最初は空）
}

impl Block {
    // 1. 引数を transactions: Vec<Transaction> に変更
    pub fn new(id: u64, previous_hash: String, transactions: Vec<Transaction>) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut block = Block {
            id,
            timestamp,
            previous_hash,
            transactions, // ここにそのまま入れる
            nonce: 0,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }
    
    pub fn calculate_hash(&self) -> String {
        let tx_data = format!("{:?}", self.transactions); 
        let data = format!("{}{}{}{}{}", self.id, self.timestamp, self.previous_hash, tx_data, self.nonce);
        
        // 省略されていたハッシュ計算の実体を書き戻します
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    pub fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);

        // 最初のハッシュが条件を満たしていない可能性があるので計算
        self.hash = self.calculate_hash();

        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Mined! Hash: {}", self.hash);
    }
}

impl Transaction {
    pub fn sign(&mut self, signing_key: &SigningKey) {
        // 送金内容（受信者と金額）を文字列にして署名の対象にする
        let message = format!("{}{}", self.receiver, self.amount);
        // 秘密鍵で署名を生成
        let signature = signing_key.sign(message.as_bytes());
        // 構造体のsignatureフィールドに保存
        self.signature = Some(signature.to_bytes().to_vec());
    }

    pub fn verify(&self, public_key_bytes: &[u8]) -> bool {
        if let Some(sig_bytes) = &self.signature {
            // 公開鍵を復元
            let public_key = VerifyingKey::from_bytes(
                public_key_bytes.try_into().expect("Invalid public key length")
            ).unwrap();
            
            // 署名を復元
            let signature = Signature::from_bytes(
                sig_bytes.as_slice().try_into().expect("Invalid signature length")
            );

            let message = format!("{}{}", self.receiver, self.amount);
            // 公開鍵で署名を検証！
            public_key.verify(message.as_bytes(), &signature).is_ok()
        } else {
            false // 署名がないものは偽物
        }
    }
}