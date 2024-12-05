// MODULO(x, y)
// 入力: 正の整数 x, y
// 出力: x % y
// 計算量: O(1)
pub fn modulo(x: i128, y: i128) -> i128 {
    if x <= 0i128 || y <= 0i128 {
        panic!("正の整数を入力してください.");
    }

    let z = x % y;

    z
}

// MODULOALT(x, y)
// 入力: 正の整数 x, y
// 出力: x % y
// 計算量: O(x / y)
pub fn moduloalt(x: i128, y: i128) -> i128 {
    if x <= 0i128 || y <= 0i128 {
        panic!("正の整数を入力してください.");
    }

    let mut t = x;
    for _i in 1i128..=x {
        if y <= t {
            t = t - y;
        } else {
            break;
        }
    }

    t
}

// 列挙型（例えば MODULO で実験する際は &Algorithm::MODULO で指定する）
// なぜこうしたか？ : 関数（アルゴリズム）と関数名（アルゴリズム名）を渡す代わりに列挙型を渡すだけで済むようにしている（ヒューマンエラーの防止）.
pub enum Algorithm {
    MODULO,
    MODULOALT,
}

// アルゴリズムのアルゴリズム名を返す関数
pub fn get_algorithm_name(select_algorithm: &Algorithm) -> &str {
    match &select_algorithm {
        Algorithm::MODULO => "MODULO",
        Algorithm::MODULOALT => "MODULOALT",
    }
}

// アルゴリズム（関数）を返す関数
pub fn get_algorithm_function(select_algorithm: &Algorithm) -> fn(i128, i128) -> i128 {
    match select_algorithm {
        Algorithm::MODULO => modulo,
        Algorithm::MODULOALT => moduloalt,
    }
}