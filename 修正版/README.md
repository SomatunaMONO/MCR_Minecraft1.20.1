# MCR Minecraft セットアップガイド

## 必要なもの

### 1. Cloudflared
[Cloudflared](https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/install-and-setup/installation/)をインストールしてください。

未インストールの場合は、管理者権限で以下のコマンドを実行してください：
```sh
winget install --id Cloudflare.cloudflared
```

### 2. CurseForge
[CurseForge](https://www.curseforge.com/download/app)をダウンロード・インストールしてください。

### 3. MODファイル
`mekanism insane.zip`をダウンロードしてください。

---

## MCR-minecraft-launcher.exe の使用方法

### 起動
1. `MCR-minecraft-launcher.exe`をダブルクリックして起動してください。

### 初回設定
2. 初回起動時は以下の設定を行ってください：

   **ランチャーパス**
   - CurseForgeの実行ファイルのパスを指定してください
   - 例：`C:\Users\ユーザー名\AppData\Local\Programs\CurseForge Windows\CurseForge.exe`
   
   **ローカルアドレス**
   - 必ず `127.0.0.1` を指定してください
   - ⚠️ `localhost` では接続できないため注意してください
   
   **ホスト名**
   - デフォルト値のまま変更しないでください

> ⚠️ 注意：起動した場所に `config.json` ファイルが生成されます。

---

## ライセンス・改変について

- ✅ このソフトのソースコードは自由に改変してもOKです
- ❌ 配布は禁止です
- 🔧 改変する場合は[Rust](https://www.rust-lang.org/ja)をインストールしてください
- 💡 他の言語で参考実装することも可能です

---

## サーバーへの接続方法

1. **MODの導入**
   - CurseForgeに `mekanism insane.zip` を追加してください

2. **マルチプレイの開始**
   - Minecraftを起動し、「マルチプレイ」を選択してください

3. **サーバー追加**
   - 「サーバーを追加」を選択し、指定されたサーバーアドレスを入力してください

---

## トラブルシューティング

何か問題が発生した場合は、Discordまでご連絡ください。