# imectl

Windows 用のシンプルな CLI。アクティブウィンドウの IME を制御（ON/OFF）する。

## ビルド

```powershell
cargo build --release
```

## 実行

引数は 0（OFF）/　1（ON）。例:

```powershell
# IME を off にする
imectl.exe 0

# IME を ON にする
imectl.exe 1
```

## テスト

### 自動テスト

```powershell
cargo test
```

- Win32 実API に依存する部分（IME 制御など）は実機での手動検証が必要

### 手動テスト

1. PowerShell を起動し IME off にする
1. `imectl 1` を実行し IME ON になること
1. `imectl 0` を実行し IME off になること
