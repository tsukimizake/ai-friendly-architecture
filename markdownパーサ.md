[[AIフレンドリーアーキテクチャくん]] のプロンプト生成元として入力されるmarkdownを読むパーサ

## パースできるべきもの
- 見出し
- 箇条書き
- plaint text内の[[]]によるリンク Plain textデータコンストラクタに添えてList Linkのように持つ
## 出力形式
[types.rs](./src/markdown/types.rs)を参照
