import matplotlib.pyplot as plt
import numpy as np
import linecache # ファイルの特定行を読み込むためのライブラリ

# 次のコマンドで日本語フォントをインストール（グラフにある日本語が文字化けしないように表示）
# pip install japanize-matplotlib
import japanize_matplotlib


# フォントサイズを設定
plt.rcParams['font.size'] = 15

# 出力ファイル名とアルゴリズム名、線の色の対応
def get_algorithm_name_and_line_color(file_name):
    if "alt" in file_name:
        return ['MODULOALT', 'green']
    else:
        return ['MODULO', 'red']

# ファイルから y の値を取得する
def get_y_value(file_name):
    line = linecache.getline(file_name, 3)
    linecache.clearcache() # キャッシュ削除

    if "y" not in line:
        return None

    num = int(line.split(":")[-1])
    return num

# 指定されたデータ（アルゴリズム）同士の処理時間を比較可能な図を作成する関数
def get_graph(*file_names):
    y_arr = []
    for file_name in file_names:
        data = np.loadtxt(file_name, skiprows=3, delimiter=',')
        algorithm_name, line_color = get_algorithm_name_and_line_color(file_name)
        plt.plot(data[:,0], data[:,1], label=algorithm_name, color=line_color)
        y = get_y_value(file_name)
        y_arr.append(y)

    # もし異なる y で比較しているなら作図しない
    if None in y_arr or len(set(y_arr)) != 1:
        return None

    y = y_arr[0]
    plt.title(f'y = {y} の場合')
    plt.xlabel('x の値') # x 軸のラベル
    plt.ylabel('処理時間 [μs]') # y 軸のラベル
    plt.legend() # 凡例を表示
    plt.tight_layout()
    plt.show()

get_graph('output0_modulo.txt', 'output0_moduloalt.txt')