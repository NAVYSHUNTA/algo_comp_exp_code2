use cpu_time::ProcessTime; // https://crates.io/crates/cpu-time
use std::time::Duration;
use algo_comp_exp_code2::*;

// エントリーポイント
fn main() {
    const INPUT_MAX_X: i128 = 100000; // 実行時間が長い場合は 100 などに変更すると良い

    const INPUT_Y: [i128; 4] = [
        2,
        INPUT_MAX_X / 2,
        INPUT_MAX_X,
        2 * INPUT_MAX_X
    ];

    // 以下のコメントアウトを外して、各アルゴリズムの実験結果を表示
    const INDEX: usize = 0;
    print_experiment_result(INPUT_MAX_X, INPUT_Y[INDEX], &Algorithm::MODULO);
    // print_experiment_result(INPUT_MAX_X, INPUT_Y[INDEX], &Algorithm::MODULOALT);
}

// 実験結果を出力する関数
pub fn print_experiment_result(max_x: i128, y: i128, select_algorithm: &Algorithm) {
    let algorithm_name: &str = get_algorithm_name(select_algorithm);
    println!("{} の実験結果", algorithm_name);
    println!("入力値 x の範囲: 1 ~ {}", max_x);
    println!("入力値 y の値: {}", y);

    let datas: Vec<(i128, u128)> = get_experiment_datas(max_x, select_algorithm);
    for (x, time) in datas {
        println!("{}, {}", x, time);
    }
}

// 実験データを返す関数
pub fn get_experiment_datas(max_x: i128, select_algorithm: &Algorithm) -> Vec<(i128, u128)> {
    let algorithm: fn(i128, i128) -> i128 = get_algorithm_function(select_algorithm);

    let mut datas: Vec<(i128, u128)> = Vec::new();
    let y = 10;
    for x in 1_i128..=max_x {
        let start_time: ProcessTime = ProcessTime::now(); // 計測開始
        let _res: i128 = algorithm(x, y);
        let duration: Duration = start_time.elapsed(); // 計測終了（計測結果）
        datas.push((x, duration.as_micros()));
    }

    datas
}