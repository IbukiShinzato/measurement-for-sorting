#!/bin/zsh

# Rustプログラムを実行し、その出力をファイルに保存
cargo run

# Pythonスクリプトを実行してデータを可視化
python3 plot_data.py
