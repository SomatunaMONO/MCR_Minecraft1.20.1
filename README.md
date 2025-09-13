# Minecraft Launcher with Cloudflared セットアップガイド

## 必要なもの

### Cloudflared
- [Cloudflared](https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/install-and-setup/installation/)  
    インストールしていない場合は、以下のコマンドを管理者権限で実行してください。

    ```sh
    winget install --id Cloudflare.cloudflared
    ```

### modについて
- Releaseページにてダウンロードしてください

### mcr-minecraft.exe
- ダンロード先をReleaseページに変更しました。
- ダブルクリックで実行してください。
- [WindowsによってPCが保護されました]と表示される場合は、「詳細情報」を押してそのまま実行してください。
- 実行すると最初にMinecraftランチャーのパスを入力してください。
- 次にポートを入力してください（デフォルト: 20100）。
- ホスト名はminecraft.nitmcr.f5.si(mekanism_insane.zip)
-          nominecraft.nitmcr.f5.si(のんびりサーバー)
- .exeファイルを実行すると同じディレクトリに`config.json`ファイルが生成されます。
- 複数のホスト名ランチャーパスの指定が可能になりました。

### Minecraft Launcher  
パスの例: `C:\Program Files\Minecraft\launcher.exe`

### CurseForge 
modの導入をサポートするアプリケーション。今回の環境では必須。

### mekanism_insane.zip
modの詰め合わせパック。Curseforgeに入れる。

## のんびりサーバー
1. 1.20.1-forge-47.4.0使用
2. mod のんびりクラフト.zipこれを展開してmodフォルダの中に入れてください

## セットアップ手順

1. **CurseForgeでmodpack導入**
   - Home→Minecraft→import→ImportProfile .zipで同梱されている`mekanism_insane.zip`をインポート。
   - 今後modpack入りのminecraftバージョンを起動する場合、Curseforgeから起動する必要あり。

4. **設定の調整**
   - 起動して個人用に設定を調整

## サーバーへの接続

1. Minecraft内で「サーバーを追加（Add Server）」をクリック
2. サーバーアドレスに指定したポートを入力  
   例: `127.0.0.1:20100`

---
## サーバーについて
- 2つのサーバーが立つようになりました
1. 工業や農業modをいれたサーバーこちらはころころ構成を変えられればなと思います。
2. のんびり系のサーバー普通の農業modやバイオーム追加系、移動系(鉄道や車など)入れたサーバーです。雰囲気を崩さないものであればmod追加のリクエストをしても大丈夫です。⚠相性などの関係で入らない場合もあります。

## トラブルシューティング

何か問題が発生した場合は、Discordからご連絡ください。
