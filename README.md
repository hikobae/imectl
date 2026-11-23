# imectl

Windows の IME（Input Method Editor）の状態をコマンドラインから制御するための軽量な CLI ツールである。

フォーカス中のウィンドウに対して IME の ON/OFF を切り替えることができる。スクリプトやショートカットからの IME 制御を想定している。

## Installation

GitHub Releases から最新版をダウンロードする。
<https://github.com/hikobae/imectl/releases/latest>

1. `imectl-x.y.z-windows-x86_64.zip` をダウンロードする。
1. ZIP ファイルを任意のディレクトリに展開する。
1. 展開したディレクトリに `PATH` を通す。

`PATH` を設定しない場合は、実行ファイルのパスを指定して実行することも可能である。

## Usage

```sh
imectl [0|1]
```

### Arguments

| 引数 | 説明 |
|-|-|
| 0 | IME を OFF にする |
| 1 | IME を ON にする |
| 省略 | IME を OFF にする |

### Examples

IME を OFF にする：

```sh
imectl 0
```

IME を ON にする：

```sh
imectl 1
```

引数を省略した場合も IME は OFF になる。

```sh
imectl
```

## Exit Codes

| 終了コード | 説明 |
|-|-|
| 0 | 正常終了 |
| 1 | IME の制御に失敗 |
| 2 | コマンドライン引数が不正 |

終了コードを確認することで、バッチファイルや PowerShell スクリプトから実行結果を判定できる。

## Limitations

- Windows 専用のツールである。
- フォーカス中のコントロールを優先し、存在しない場合はアクティブウィンドウを制御対象とする。
- 対象アプリケーションの権限レベルやウィンドウの応答状態によっては、IME の制御に失敗する場合がある。
- 対象ウィンドウが一定時間内に応答しない場合、処理はタイムアウトする。

## Building from Source

Rust の開発環境が必要である。

リポジトリを取得する：

```sh
git clone https://github.com/hikobae/imectl.git
cd imectl
```

ビルドする：

```sh
cargo build --release --locked
```

生成される実行ファイル：

```sh
target/release/imectl.exe
```

## Testing

### 自動テスト

```sh
cargo test --locked
```

### 手動テスト

Win32 実API に依存する部分（IME 制御など）は実機での手動検証が必要

1. PowerShell を起動し、日本語 IME を OFF にする。
1. `imectl 1` を実行し、IME が ON になることを確認する。
1. `imectl 0` を実行し、IME が OFF になることを確認する。

## License

This project is licensed under the MIT License.

Third-party dependency licenses are listed in
[LICENSE-THIRD-PARTY.md](LICENSE-THIRD-PARTY.md).

## Author

TAKAHASHI Satoshi <hikobae@gmail.com>
