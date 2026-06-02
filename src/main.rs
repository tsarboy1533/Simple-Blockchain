mod block;
mod blockchain;

use ed25519_dalek::SigningKey;
use rand::RngCore;

use block::Transaction; // Transactionを使えるようにする
use blockchain::Blockchain;

fn main() {
    // --- 準備：鍵ペアの生成 ---
    let mut entropy = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut entropy);
    let signing_key = SigningKey::from_bytes(&entropy);
    let public_key = signing_key.verifying_key();
    let wallet_address = hex::encode(public_key.to_bytes());

    // --- 1. ブロックチェーンの初期化 ---
    let mut my_blockchain = Blockchain::new();

    // ==========================================
    // --- 2. 1つ目のトランザクション（Bob宛て） ---
    // ==========================================
    println!("Creating transaction 1...");
    let mut tx1 = Transaction {
        sender: wallet_address.clone(),
        receiver: String::from("Bob_Address_Sample"),
        amount: 50.0,
        signature: None,
    };
    //署名
    tx1.sign(&signing_key);

    // 【復活！】1つ目の署名検証
    if tx1.verify(&public_key.to_bytes()) {
        println!("Tx1: Signature verified! Sending to Mempool...");
        my_blockchain.add_transaction(tx1, &public_key.to_bytes());
    } else {
        println!("Tx1: Invalid signature! Transaction rejected.");
    }


    // ==========================================
    // --- 3. 2つ目のトランザクション（Charlie宛て） ---
    // ==========================================
    println!("\nCreating transaction 2...");
    let mut tx2 = Transaction {
        sender: wallet_address.clone(),
        receiver: String::from("Charlie_Address_Sample"),
        amount: 20.0,
        signature: None,
    };
    //署名
    tx2.sign(&signing_key);

    // 2つ目の署名検証
    if tx2.verify(&public_key.to_bytes()) {
        println!("Tx2: Signature verified! Sending to Mempool...");
        my_blockchain.add_transaction(tx2, &public_key.to_bytes());
    } else {
        println!("Tx2: Invalid signature! Transaction rejected.");
    }


    // ==========================================
    // --- 4. Mempoolに溜まった取引をまとめてマイニング ---
    // ==========================================
    println!("\n--- Start Mining Mempool ---");
    my_blockchain.mine_mempool();
    println!("----------------------------\n");


    // 最終的なチェーンの表示
    for block in my_blockchain.chain {
        println!("{:#?}", block);
    }
}