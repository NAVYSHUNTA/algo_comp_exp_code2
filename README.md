# algo_comp_exp_code2
アルゴリズムと計算量特論の実験レポートで用いるプログラムのコード.

Rust 言語のプログラムを実行するためには, 事前に `cargo` を用意する必要がある.
これ以降では, Python 言語および Rust 言語の環境構築が済んでいることを前提に述べる.

# ディレクトリ構成
ディレクトリ構成の中から重要なもののみを抜粋すると以下の通りである.
<pre>
.
├── src
│   ├── lib.rs
│   └── main.rs
├── tests
│   └── lib_test.rs
├── graph.py
├── output0_modulo.txt
├── output0_moduloalt.txt
├── output1_modulo.txt
├── output1_moduloalt.txt
├── output2_modulo.txt
├── output2_moduloalt.txt
├── output3_modulo.txt
└── output3_moduloalt.txt
</pre>

- `./src/lib.rs` : Rust 言語で $2$ つのアルゴリズム MODULO, MODULOALT を関数として実装したもの.
- `./src/main.rs` : アルゴリズムの処理時間の計測（実験）を行うコード. 実験結果は `output0_modulo.txt` 等である（ `cargo run > output0_modulo.txt` で実行）.
- `./tests/lib_test.rs` : `lib.rs` で実装した関数が正しいかのテストを行うテストコード（ `cargo test` でテスト実行）.
- `./graph.py` : Rust 言語で実装した各アルゴリズムの処理時間の計測結果をグラフ化するための Python 言語のコード（`python graph.py` で実行）.
- `./output0_modulo.txt` : MODULO の処理時間 $($ 入力値 $x$ の範囲 : $1 \le x \le 100000$ , $y$ の値 : $y = 2$ $)$
- `./output0_moduloalt.txt` : MODULOALT の処理時間 $($ 入力値 $x$ の範囲 : $1 \le x \le 100000$ , $y$ の値 : $y = 2$ $)$
- `./output1_modulo.txt` : MODULO の処理時間 $($ 入力値 $x$ の範囲 : $1 \le x \le 100000$ , $y$ の値 : $y = 50000$ $)$
- `./output1_moduloalt.txt` : MODULOALT の処理時間 $($ 入力値 $x$ の範囲 : $1 \le x \le 100000$ , $y$ の値 : $y = 50000$ $)$
- `./output2_modulo.txt` : MODULO の処理時間 $($ 入力値 $x$ の範囲 : $1 \le x \le 100000$ , $y$ の値 : $y = 100000$ $)$
- `./output2_moduloalt.txt` : MODULOALT の処理時間 $($ 入力値 $x$ の範囲 : $1 \le x \le 100000$ , $y$ の値 : $y = 100000$ $)$
- `./output3_modulo.txt` : MODULO の処理時間 $($ 入力値 $x$ の範囲 : $1 \le x \le 100000$ , $y$ の値 : $y = 200000$ $)$
- `./output3_moduloalt.txt` : MODULOALT の処理時間 $($ 入力値 $x$ の範囲 : $1 \le x \le 100000$ , $y$ の値 : $y = 200000$ $)$