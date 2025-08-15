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

### CurseForge 
modの導入をサポートするアプリケーション。今回の環境では必須。

### mekanism_insane.zip
modの詰め合わせパック。Curseforgeに入れる。

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

## トラブルシューティング

何か問題が発生した場合は、Discordからご連絡ください。
