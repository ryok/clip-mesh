# ClipMesh - AI Universal Clipboard

AIユニバーサル・クリップボード。コピー＆ペーストのたびにAIが仕事を済ませる。

## 現在の実装状況

### 完了した機能
- ✅ 基本的なプロジェクト構造（Rust + Cargo）
- ✅ クリップボード監視機能（macOS対応）
- ✅ データ永続化（JSONファイル保存）
- ✅ 基本的なスマート変換（数値フォーマット）
- ✅ コマンドラインインターフェース（CLI）

### 今後の実装予定
- ⏳ UI実装（システムトレイ、履歴表示）
- ⏳ Windows/Linux対応
- ⏳ AI翻訳機能の統合
- ⏳ クラウド同期機能

## ビルドと実行

```bash
# ビルド
cargo build --release

# クリップボード監視を開始
./target/release/clip-mesh monitor

# または単純に
./target/release/clip-mesh
```

## CLIコマンド

```bash
# ヘルプを表示
./target/release/clip-mesh --help

# 履歴を表示（最新10件）
./target/release/clip-mesh list

# 履歴を表示（最新20件）
./target/release/clip-mesh list --limit 20

# 検索
./target/release/clip-mesh search "keyword"

# 特定のアイテムの詳細表示
./target/release/clip-mesh show <item-id>
```

## 使い方

1. アプリケーションを起動すると、バックグラウンドでクリップボードを監視開始
2. テキストをコピーすると自動的に履歴に保存
3. 数値をコピーすると自動的にカンマ区切り形式に変換

## データ保存場所

- macOS: `~/Library/Application Support/clip-mesh/clipboard_history.json`
- Linux: `~/.local/share/clip-mesh/clipboard_history.json`
- Windows: `%APPDATA%\clip-mesh\clipboard_history.json`

## テスト用コピーサンプル

以下のテキストをコピーして動作を確認できます：

- 数値: `1234567.89`
- URL: `https://example.com`
- メールアドレス: `test@example.com`