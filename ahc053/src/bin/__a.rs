use proconio::{input, source::line::LineSource};
use std::io::{stdin, stdout, BufReader, Write};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    
    // 入力読み込み
    input! {
        from &mut source,
        n: usize,
        m: usize,
        l: i64,
        u: i64,
    }
    
    // 戦略1: 多様なサイズのカードを作成
    // 大きな値、中程度の値、小さな値を混合
    let mut cards = Vec::new();
    
    // 大きな値のカード（目標値に近い）
    let large_base = (l + u) / 2;
    for i in 0..n/3 {
        let offset = (i as i64 % 1000) * 1000000; // バリエーションを追加
        cards.push((large_base + offset).min(u));
    }
    
    // 中程度の値のカード（調整用）
    let medium_base = large_base / 10;
    for i in 0..(n/3) {
        let offset = (i as i64 % 100) * 10000;
        cards.push((medium_base + offset).min(u).max(1));
    }
    
    // 小さな値のカード（微調整用）
    for i in 0..(n - 2*(n/3)) {
        let small_val = 1 + (i as i64 % 10000);
        cards.push(small_val.min(u));
    }
    
    // カードの値を出力
    for i in 0..n {
        print!("{}", cards[i]);
        if i < n - 1 { print!(" "); }
    }
    println!();
    stdout().flush().unwrap();
    
    // 目標値を読み込み
    let mut targets = Vec::new();
    input! {
        from &mut source,
        targets_input: [i64; m],
    }
    targets = targets_input;
    
    // 各山の現在の合計を管理
    let mut pile_sums = vec![0i64; m];
    let mut assignments = vec![0; n];  // 0は破棄を意味
    
    // 贪心算法：各カードを最も効果的な山に配置
    for card_idx in 0..n {
        let card_value = cards[card_idx];
        
        let mut best_pile = 0;
        let mut best_improvement = i64::MIN;
        
        // 各山に配置した場合の改善度を計算
        for pile in 0..m {
            let current_error = (pile_sums[pile] - targets[pile]).abs();
            let new_sum = pile_sums[pile] + card_value;
            let new_error = (new_sum - targets[pile]).abs();
            
            let improvement = current_error - new_error;
            
            if improvement > best_improvement {
                best_improvement = improvement;
                best_pile = pile;
            }
        }
        
        // 改善がある場合のみカードを配置（そうでなければ破棄）
        if best_improvement > 0 {
            assignments[card_idx] = best_pile + 1;  // 1-indexed
            pile_sums[best_pile] += card_value;
        } else {
            assignments[card_idx] = 0;  // 破棄
        }
    }
    
    // 結果を出力
    for i in 0..n {
        print!("{}", assignments[i]);
        if i < n - 1 { print!(" "); }
    }
    println!();
    stdout().flush().unwrap();
}