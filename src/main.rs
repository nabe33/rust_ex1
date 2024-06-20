use std::io;

fn main() {
    loop {
        // 華氏->摂氏の変換
        println!("華氏に変換したい摂氏の温度を入力してね！（Qで終了）");
        let mut temp_c = String::new();

        io::stdin()
            .read_line(&mut temp_c)
            .expect("読み込みに失敗しました");

        // 入力が "Q" かどうかを確認
        if temp_c.trim().eq_ignore_ascii_case("Q") {
            println!("終了します");
            break;
        }

        // 入力された値を f32 にパース
        let temp_c: f32 = match temp_c.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("数字を入力してね！");
                continue;
            }
        };

        fun_1(temp_c);
    }

    loop {
        // フィボナッチ数列
        println!("何番目のフィボナッチ数列を計算してほしいですか？（Qで終了）");
        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("読み込みに失敗しました");

        // 入力が "Q" かどうかを確認
        if n.trim().eq_ignore_ascii_case("Q") {
            println!("終了します");
            break;
        }

        // 入力された値を f64 にパース
        let n: u64 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("数字を入力してね！");
                continue;
            }
        };

        fun_2(n);
    }
}

// 摂氏->華氏の変換
fn fun_1(temp_c: f32) {
    let temp_f = temp_c * 9.0 / 5.0 + 32.0;
    println!("{}°C は {}°F だよ", temp_c, temp_f);
}

// フィボナッチ数列
fn fun_2(n: u64) {
    let mut a: i64 = 0;
    let mut b: i64 = 1;
    let mut golden_ratio: f64 = 0.0;
    for _ in 0..n {
        let c: i64 = a + b;
        a = b;
        b = c;
        golden_ratio = b as f64 / a as f64;
    }
    println!(
        "{}番目のFibonacci数列は{}．黄金比は{}．",
        n, a, golden_ratio
    );
}
