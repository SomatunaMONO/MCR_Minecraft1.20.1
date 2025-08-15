# Minecraft Launcher with Cloudflared セットアップガイド

## 必要なもの

### Cloudflared
- [Cloudflared](https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/install-and-setup/installation/)  
    インストールしていない場合は、以下のコマンドを管理者権限で実行してください。

    ```sh
    winget install --id Cloudflare.cloudflared
    ```

### MCR-minecraft-launcher.exe
- ダブルクリックで実行してください。
- [WindowsによってPCが保護されました]と表示される場合は、「詳細情報」を押してそのまま実行してください。
- 実行すると最初にMinecraftランチャーのパスを入力してください。
- 次にポートを入力してください（デフォルト: 20100）。
- ホスト名はデフォルトのままで大丈夫です(デフォルト: minecraft.nitmcr.f5.si) 。
- .exeファイルを実行すると同じディレクトリに`config.json`ファイルが生成されます。

### Minecraft Launcher  
パスの例: `C:\Program Files\Minecraft\launcher.exe`

### MineCRaft ModClient
1. ~~[GitHub リポジトリ](https://github.com/mcr-2024b/MineCRaft_ModCliant)にアクセス~~ mod_settingfiles.zipをクリック
2. ダウンロードマークをクリックしてダウンロード 
3. .zipファイルを展開（解凍）

### Java 8
- インストールされていない場合は`JavaSetup8u421.exe`を実行してインストール

### Forge 1.12.2
- `forge-1.12.2-14.23.5.2860-installer.jar`を実行してインストール

## セットアップ手順

1. **Forgeプロファイルの作成**
   - Minecraft LauncherでForgeを一度起動し、すぐ終了してください
   - ⚠️ パスを正しく指定しないと無意味なので注意してください

2. **Modファイルの配置**
   - 生成されたModsフォルダに、ダウンロードしたZIPファイル内のすべてのmodファイルを配置

3. **設定の調整**
   - もう一度起動して個人用に設定を調整

## サーバーへの接続

1. Minecraft内で「サーバーを追加（Add Server）」をクリック
2. サーバーアドレスに指定したポートを入力  
   例: `localhost:20100`

---

## トラブルシューティング

何か問題が発生した場合は、Discordからご連絡ください。
