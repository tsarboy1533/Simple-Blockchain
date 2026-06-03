use crate::blockchain::block::{Block, Transaction};

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub mempool: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        // ジェネシスブロックも Vec<Transaction> を受け取るように修正
        // 最初は空のリスト vec![] を渡す
        let genesis_block = Block::new(
            0, 
            String::from("0"), 
            vec![] // 空のトランザクションリスト
        );
        
        Blockchain {
            chain: vec![genesis_block],
            mempool: Vec::new(),
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let mut new_block = Block::new(
            self.chain.len() as u64, 
            previous_hash, 
            transactions
        );
        
        new_block.mine(4); 
        self.chain.push(new_block);
    }

    pub fn add_transaction(&mut self, tx: Transaction, public_key_bytes: &[u8]) -> bool {
        // 前に作った verify メソッドで署名をチェック！
        if tx.verify(public_key_bytes) {
            println!("Transaction from {} is valid. Added to Mempool!", tx.sender);
            self.mempool.push(tx); // 箱にプッシュ！
            true
        } else {
            println!("Transaction invalid! Rejected.");
            false
        }
    }

    // Mempoolに溜まった取引をまとめてブロック化（マイニング）する
    pub fn mine_mempool(&mut self) {
        if self.mempool.is_empty() {
            println!("Mempool is empty. Nothing to mine.");
            return;
        }

        println!("Mining a new block with {} transactions...", self.mempool.len());

        // 1. 現在のMempoolの中身をまるごとコピーして、ブロック用のデータにする
        let transactions_to_mine = self.mempool.clone();

        // 2. 既存の add_block 関数を呼び出す（引数に取引の束を渡す）
        self.add_block(transactions_to_mine);

        // 3. 無事にブロックに入ったので、Mempool（待合室）を空っぽにする！
        self.mempool.clear();
        println!("Mempool cleared.");
    }
}