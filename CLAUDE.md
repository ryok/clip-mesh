# ClipMesh - AIユニバーサル・クリップボード

## プロダクトコンセプト
コピー＆ペーストのたびに AI が仕事を済ませる。  
OS 付属の「当たり前機能」を AI 時代にアップグレードする、"誰でも・どこでも" 使う新インフラ。

## 1. 背景 - なぜクリップボードなのか

| 課題 | 現状 | ClipMesh での変化 |
|------|------|-------------------|
| コピー後の手作業 | 貼り付け前に整形・翻訳・抽出などを毎回手動 | コピーした瞬間に AI が用途を推測して自動変換 |
| 情報散逸 | 転職情報・住所・リンクがバラバラに埋もれる | 全文検索・時系列ビューで「いつ何をコピーしたか」を瞬時に再利用 |
| デバイス間の断絶 | PC→スマホ間ペーストにメールやチャットを使用 | クラウド同期で OS を超え即ペースト、オフライン時はローカル保持 |

## 2. プロダクト概要

| 項目 | 内容 |
|------|------|
| 常駐方式 | Windows/macOS/Linux/iOS/Android すべてにネイティブ常駐（OS API Hook） |
| 起動 | 通常コピー：Ctrl/Cmd+C<br>**＋**コピー連打：ClipMesh パネル表示 |
| AI コア | エッジ実行 2-3 B LLM + Whisper Tiny（音声コピー用） + クラウド拡張 |
| UI | 最小化された リスト & タブ：<br>• 直近 30 件<br>• 自動カテゴリ（住所 / URL / コード / 日付）<br>• "スマート変換" トグル |
| プライバシー | オンデバイス推論＆E2E 暗号化同期。企業版はオフライン専用モード |

## 3. コア機能

### スマート変換
- 数値列 ⇒ 自動でカンマ整形 or 合計計算
- 英文 ⇒ 日本語訳＋ふりがな
- CSV ⇒ Markdown 表 or JSON

### 用途推測 (Intent Detection)
- URL → "コピーで即ブラウザを開く？"を提案
- 住所 → 地図リンク生成

### 高速検索 & サジェスト
- ⌥⌘Shift+F（モバイルは2本指スワイプ）で全文ベクトル検索
- よく貼るフレーズは "ピン留め"＋自動予測入力

### 多段ペースト
- １つのコピーを **形式違い（TXT／RTF／HTML）**でペースト選択

### クイックAI
- コピー後に「//sum」と打つと 要約結果 がペーストバッファへ上書き
- 「//tone polite」に続ければ 敬語化 などテンプレート変換

### オーディオ・画像対応
- 音声クリップは即テキスト化、画像は OCR → 文章 or 表に整形

## 4. 代表ユースケース

| シーン | 旧フロー | ClipMesh |
|--------|----------|----------|
| 資料作成 | Web → コピー → ブラウザタブ切替 → Excel へ貼付 → 手作業整形 | コピーした瞬間、自動で表整形＋合計列。ペースト一回で完成 |
| 外国語メール | 英文コピー → 翻訳サイト → 貼付 | コピー→ペーストで 日本語訳済み |
| ソースコード共有 | コード → チャット → フォーマット崩れ | コピー時に シンタックスハイライト付き & 行番号保持 |
| 日程調整 | 候補日リンクを複数ペースト→重複・ミス | 時刻列コピー→自動でカレンダー招待を生成 |

## 5. 普及戦略 - "当たり前" になるための条件

- **ゼロ学習曲線**：既存操作（Ctrl+C / Ctrl+V）に "+α" だけ
- **OSレベル組み込み** API を OSS で提供 → デバイスメーカーや Linux ディストリへ波及
- **フリーミアム**：
  - 個人版＝無料・広告なし
  - Pro＝組織LoRA共有・監査ログ機能（月$4.99）
- **エコシステム**：ClipMesh Actions Marketplace（翻訳エンジン／CMS投稿／会計ソフト連携）

## 開発メモ

### 技術スタック（想定）
- **クロスプラットフォーム**: Rust + Tauri または Flutter
- **AI推論**: ONNX Runtime + quantized models
- **同期**: WebRTC DataChannel + E2E暗号化
- **検索**: Tantivy (Rust) または SQLite FTS5

### MVP開発優先順位
1. ✅ macOS版の基本クリップボード監視
2. ✅ スマート変換（数値整形）
3. ✅ 履歴管理・検索機能
4. ✅ CLI インターフェース
5. ⏳ システムトレイUI
6. ⏳ Windows/Linux対応
7. ⏳ AI翻訳機能統合
8. ⏳ クラウド同期基盤
9. ⏳ モバイル版展開

### 現在のアーキテクチャ
- **コア**: `src/core/` - クリップボード監視、データ型定義
- **ストレージ**: `src/storage/` - JSONファイルベースの永続化
- **変換**: `src/transforms/` - スマート変換エンジン
- **CLI**: `src/cli.rs` - コマンドライン操作
- **UI**: `src/ui/` - 将来のGUI実装用

### 実装済み機能
- クリップボード内容の自動検出・分類
- 数値の自動フォーマット（カンマ区切り）
- 履歴の永続化（JSON）
- 検索機能
- CLIインターフェース

### コマンド
開発時に使用する主要コマンド：
```bash
# ビルド
cargo build --release

# 実行（監視モード）
./target/release/clip-mesh monitor

# 履歴表示
./target/release/clip-mesh list

# 検索
./target/release/clip-mesh search "query"

# テスト実行
cargo test

# 型チェック
cargo check

# リント
cargo clippy
```