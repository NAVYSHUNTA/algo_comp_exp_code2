use algo_comp_exp_code2::*;

// アルゴリズム MODULO の単体テスト
#[test]
fn test_modulo_succeeds() {
    assert_eq!(0_i128, modulo(1_i128, 1_i128));
    assert_eq!(1_i128, modulo(1_i128, 2_i128));
    assert_eq!(0_i128, modulo(6_i128, 3_i128));
    assert_eq!(3_i128, modulo(13_i128, 5_i128));
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_modulo_input_x_zero_panic() {
    modulo(0_i128, 3_i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_modulo_input_y_zero_panic() {
    modulo(4_i128, 0_i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_modulo_input_xy_zero_panic() {
    modulo(0_i128, 0_i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_modulo_input_x_negative_panic() {
    modulo(-1_i128, 1_i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_modulo_input_y_negative_panic() {
    modulo(3_i128, -2_i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_modulo_input_xy_negative_panic() {
    modulo(-7_i128, -5_i128);
}

// アルゴリズム MODULOALT の単体テスト
#[test]
fn test_moduloalt_succeeds() {
    assert_eq!(0_i128, moduloalt(1_i128, 1_i128));
    assert_eq!(1_i128, moduloalt(1_i128, 2_i128));
    assert_eq!(0_i128, moduloalt(6_i128, 3_i128));
    assert_eq!(3_i128, moduloalt(13_i128, 5_i128));
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_moduloalt_input_x_zero_panic() {
    moduloalt(0_i128, 3_i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_moduloalt_input_y_zero_panic() {
    moduloalt(4_i128, 0_i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_moduloalt_input_xy_zero_panic() {
    moduloalt(0_i128, 0_i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_moduloalt_input_x_negative_panic() {
    moduloalt(-1_i128, 1_i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_moduloalt_input_y_negative_panic() {
    moduloalt(3_i128, -2_i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_moduloalt_input_xy_negative_panic() {
    moduloalt(-7_i128, -5_i128);
}