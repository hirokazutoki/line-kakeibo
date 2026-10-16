# line-kakeibo

Software Design 2026年連載記事「Rustで作るイベント駆動型アーキテクチャ」を参考にして作成しているアプリ

## ディレクトリ構成

```
.
├── Cargo.lock              # workspace全体の定義
├── Cargo.toml              # 依存バージョンをworkspaceで共有
├── apps/
│   ├── message-receiver/
│   ├── receipt-analyzer/
│   └── response-service/
└── crates/                 # ライブラリクレート（共通部分）
    ├── aws/                # 共通の型定義・設定
    ├── external-api/       # AWS SDKのラッパー
    └── shared/             # 外部APIクライアント
```