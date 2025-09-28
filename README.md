# Minecraft Launcher with Cloudflared セットアップガイド

## 必要なもの

### Cloudflared
- [Cloudflared](https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/install-and-setup/installation/)  
    インストールしていない場合は、以下のコマンドを管理者権限で実行してください。

    ```sh
    winget install --id Cloudflare.cloudflared
    ```

### modについて
- ReleaseページのModfileにてダウンロードしてください

### mcr-minecraft.exe
- ダンロード先をReleaseページに変更しました。
- ダブルクリックで実行してください。
- [WindowsによってPCが保護されました]と表示される場合は、「詳細情報」を押してそのまま実行してください。
- 実行すると最初にMinecraftランチャーのパスを入力してください。
- 次にポートを入力してください（デフォルト: 20100）。
- ホスト名は
- 1 mekanism_insane, 2のんびりサーバー
1.     minecraft.nitmcr.f5.si
2.     nobiminecraft.nitmcr.f5.si
- .exeファイルを実行すると同じディレクトリに`config.json`ファイルが生成されます。
- 複数のホスト名ランチャーパスの指定が可能になりました。

### Minecraft Launcher  
パスの例: `C:\Program Files\Minecraft\launcher.exe`

### CurseForge 
modの導入をサポートするアプリケーション。今回の環境では必須。

### mekanism_insane.zip
modの詰め合わせパック。Curseforgeに入れる。

## セットアップ手順(mekanism_insane.zip版)

1. **CurseForgeでmodpack導入**
   - Home→Minecraft→import→ImportProfile .zipで同梱されている`mekanism_insane.zip`をインポート。
   - 今後modpack入りのminecraftバージョンを起動する場合、Curseforgeから起動する必要あり。

2. **設定の調整**
   - 起動して個人用に設定を調整
  
## セットアップ手順(のんびりサーバー版)
1. 1.20.1-forge-47.4.0をインストールしてください。
2. 一回マインクラフトで1.20.1-forge-47.4.0を実行してください。⚠一度実行しないと必要なフォルダが生成されません。
3. default_ver3.zipこれを展開してmodフォルダの中に入れてください
4. マインクラフトを実行してください。

## mod変更お知らせ(のんびりサーバー版)
1. ** 削除mod **
- 9/19
- MSD-forge-1.20.1-4.0.1-1.4.1
- MTR-forge-4.0.1+1.20.1
- 9/28
- Oh The Biomes We've Gone
- BA_BT
2. ** 追加mod **
- 9/19
- create-1.20.1-0.5.1.j.jar
- Steam_Rails-1.6.7+forge-mc1.20.1.jar
- urushi-1.20.1-6.5.4.jar
- 9/25
- Angel Block Renewed
- 9/28
#### 追加modは追加modフォルダの中に入っています。

### のんびりサーバー 一括破壊のconfigについて
1. 現状はデフォルトのものしか一括破壊に対応させていません。(ID確認して打つのがめんどくさい)

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
