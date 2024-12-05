use algo_comp_exp_code2::*;

// アルゴリズム MODULO の単体テスト
#[test]
fn test_modulo_succeeds() {
    assert_eq!(0i128, modulo(1i128, 1i128));
    assert_eq!(1i128, modulo(1i128, 2i128));
    assert_eq!(0i128, modulo(6i128, 3i128));
    assert_eq!(3i128, modulo(13i128, 5i128));
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_modulo_input_x_zero_panic() {
    modulo(0i128, 3i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_modulo_input_y_zero_panic() {
    modulo(4i128, 0i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_modulo_input_xy_zero_panic() {
    modulo(0i128, 0i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_modulo_input_x_negative_panic() {
    modulo(-1i128, 1i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_modulo_input_y_negative_panic() {
    modulo(3i128, -2i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_modulo_input_xy_negative_panic() {
    modulo(-7i128, -5i128);
}

// アルゴリズム MODULOALT の単体テスト
#[test]
fn test_moduloalt_succeeds() {
    assert_eq!(0i128, moduloalt(1i128, 1i128));
    assert_eq!(1i128, moduloalt(1i128, 2i128));
    assert_eq!(0i128, moduloalt(6i128, 3i128));
    assert_eq!(3i128, moduloalt(13i128, 5i128));
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_moduloalt_input_x_zero_panic() {
    moduloalt(0i128, 3i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_moduloalt_input_y_zero_panic() {
    moduloalt(4i128, 0i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_moduloalt_input_xy_zero_panic() {
    moduloalt(0i128, 0i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_moduloalt_input_x_negative_panic() {
    moduloalt(-1i128, 1i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_moduloalt_input_y_negative_panic() {
    moduloalt(3i128, -2i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_moduloalt_input_xy_negative_panic() {
    moduloalt(-7i128, -5i128);
}