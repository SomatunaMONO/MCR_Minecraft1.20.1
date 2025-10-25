# Minecraft Launcher with Cloudflared セットアップガイド

## 必要なもの

### Cloudflared

- [Cloudflared](https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/install-and-setup/installation/)  
   インストールしていない場合は、以下のコマンドを管理者権限で実行してください。

  ```sh
  winget install --id Cloudflare.cloudflared
  ```

### mod について

- Release ページの Modfile にてダウンロードしてください

### mcr-minecraft.exe

- [ダンロード先](https://github.com/SomatunaMONO/MCR_Minecraft1.20.1/releases/tag/4.0.exe)
- ダブルクリックで実行してください。
- [Windows によって PC が保護されました]と表示される場合は、「詳細情報」を押してそのまま実行してください。
- 実行すると最初に Minecraft ランチャーのパスを入力してください。
- 次にポートを入力してください（デフォルト: 20100）。
- ホスト名は
- 1 mekanism_insane, 2 のんびりサーバー

1.       minecraft.nitmcr.f5.si
2.       nobiminecraft.nitmcr.f5.si

- .exe ファイルを実行すると同じディレクトリに`config.json`ファイルが生成されます。
- 複数のホスト名ランチャーパスの指定が可能になりました。

### Minecraft Launcher

パスの例: `C:\Program Files\Minecraft\launcher.exe`

### CurseForge

mod の導入をサポートするアプリケーション。今回の環境では必須。

### 工業 Mod PvP サーバー

- PvP の開催は Discord にてお知らせします。
- 禁止事項

1. 意図的にサーバーに異常な負荷をかけて落とす行為
2. 禁止アイテムの所持・使用・装備・それを使用したクラフトの一切を禁じます。
   - ProjectE のデフォルトのレシピでダークマターを使用するアイテム。
   - ProjectE の指輪系アイテム全般。
   - ProjectE のノヴァ TNT、スーパーノヴァ TNT。
   - Sophisticated Backpacks の防護機能。

## セットアップ手順(PvP サーバー)

1. **CurseForge で modpack 導入**

   - Home→Minecraft→import→ImportProfile .zip で同梱されている`PvP.zip`をインポート。
   - 今後 modpack 入りの minecraft バージョンを起動する場合、Curseforge から起動する必要あり。

2. **設定の調整**
   - 起動して個人用に設定を調整

## セットアップ手順(のんびりサーバー版)

1. 1.20.1-forge-47.4.0 をインストールしてください。
2. 一回マインクラフトで 1.20.1-forge-47.4.0 を実行してください。⚠ 一度実行しないと必要なフォルダが生成されません。
3. [default_ver6.zip](https://github.com/SomatunaMONO/MCR_Minecraft1.20.1/releases/tag/mod_nobi_change_ver6)これを展開して mod フォルダの中に入れてください。
4. マインクラフトを実行してください。

## mod 変更お知らせ(のんびりサーバー版)

1. ** 削除 mod **

- 9/19
- MSD-forge-1.20.1-4.0.1-1.4.1
- MTR-forge-4.0.1+1.20.1
- 9/28
- Oh The Biomes We've Gone
- BA_BT
- 10/6
- car-forge-1.20.1-1.0.34.jar

2. ** 追加 mod **

- 9/19
- create-1.20.1-0.5.1.j.jar
- Steam_Rails-1.6.7+forge-mc1.20.1.jar
- urushi-1.20.1-6.5.4.jar
- 9/25
- Angel Block Renewed
- 9/28
- CBMultipart-1.20.1-3.3.0.146-universal
- CodeChickenLib-1.20.1-4.4.0.516-universal
- ProjectRed-1.20.1-4.21.0-core
- ProjectRed-1.20.1-4.21.0-expansion
- ProjectRed-1.20.1-4.21.0-exploration
- ProjectRed-1.20.1-4.21.0-fabrication
- ProjectRed-1.20.1-4.21.0-illumination
- ProjectRed-1.20.1-4.21.0-integration
- ProjectRed-1.20.1-4.21.0-transmission
- radium-mc1.20.1-0.12.4+git.26c9d8e
- 10/6
- ChickenChunks-1.20.1-2.10.0.100-universal.jar
- chisels-and-bits-forge-1.4.148.jar
- createdieselgenerators-1.20.1-1.2i.jar
- Expanded UNUverse Pack [MTS] 1.20.1-22.18.0-2.1.3.jar
- framework-forge-1.20.1-0.7.15.jar
- gtcraft-2.0.1(1.20.1).jar
- Immersive Vehicles-1.20.1-22.18.0-pre.jar
- refurbished_furniture-forge-1.20.1-1.0.14.jar
- UNU Civilian Pack [MTS] 1.20.1-22.18.0-6.7.1.jar
- UNU Military Pack [MTS] 1.20.1-22.18.0-6.0.3.jar
- UNU Parts Pack [MTS] 1.20.1-22.18.0-6.7.3.jar
- 10/7
- potions_backport-1.0.0-forge-1.20.1.jar
- 10/14
- architectury-9.2.14-forge.jar
- CraftTweaker-forge-1.20.1-14.0.59.jar
- mcjtylib-1.20-8.0.7.jar
- notenoughwands-1.20-6.0.2.jar
- rhino-forge-2001.2.3-build.10.jar

3. ** バージョン変更 mod **

- 10/14
- urushi-1.20.1-6.5.4.jar → urushi-1.20.1-6.6.1.jar

4. ** easy_recipe.zs について **

- recipe の一部簡易化のための scripts です。
- Scripts フォルダの設定

1. Minecraft 終了後、ワークスペース内に `scripts` フォルダが生成されます
2. 生成された `scripts` フォルダに `easy_recipe.zs` ファイルを配置してください
3. 再度 Minecraft を起動して、スクリプトが正しく読み込まれることを確認してください

- [easy_recipe.zs](https://github.com/SomatunaMONO/MCR_Minecraft1.20.1/blob/main/%E8%BF%BD%E5%8A%A0mod/10%E6%9C%8814%E6%97%A5/easy_recipe.zs)は zip ファイルの中にあります。

#### 追加 mod は追加 mod フォルダの中に入っています。

### のんびりサーバー 一括破壊の config について

- [config](https://github.com/SomatunaMONO/MCR_Minecraft1.20.1/blob/main/config%E8%A8%AD%E5%AE%9A%E3%81%AB%E3%81%A4%E3%81%84%E3%81%A6/nobiminecraft.md)
- 上記リンク先にてご確認ください

## サーバーへの接続

1. Minecraft 内で「サーバーを追加（Add Server）」をクリック
2. サーバーアドレスに指定したポートを入力  
   例: `127.0.0.1:20100`

---

## サーバーについて

- 2 つのサーバーが立つようになりました

1. 工業や農業 mod をいれたサーバーこちらはころころ構成を変えられればなと思います。
2. のんびり系のサーバー普通の農業 mod やバイオーム追加系、移動系(鉄道や車など)入れたサーバーです。雰囲気を崩さないものであれば mod 追加のリクエストをしても大丈夫です。⚠ 相性などの関係で入らない場合もあります。

## トラブルシューティング

何か問題が発生した場合は、Discord からご連絡ください。
