# ugc-server
Now developing...

## 環境変数
- `DATABASE_URL` - `mysql://<user>:<pass>@<host>:<port>/<database>`

## サーバの起動
```sh
cargo run -p server -r
```

## トークンの追加および削除
```sh
$ cargo run -p gentoken -- --help
Usage: gentoken <COMMAND>

Commands:
  gen   
  del   
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

サブコマンド
- `gen` - 作成
- `del` - 削除
