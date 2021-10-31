# Processing_speed_measurement_by_Rust

Rustで処理速度を測るリポジトリ

単純な計算やデータベースアクセスの速度を測る

環境

Diesel



コンテナ作成

```
$ docker-compose up -d --build
```

コンテナとイメージ破棄

```
$ docker-compose down --rmi all --volumes --remove-orphans
```

各種コンテナに入る

```
rustコンテナ：ここでRustを動かしている
$ docker-compose exec rust bash
```

```
dbコンテナ：ここでmysqlを動かしている
$ docker-compose exec db bash
```

構築

envファイル作成
```
$ echo DATABASE_URL=mysql://root:password@db/test > .env
```

```
$ docker-compose build

$ docker-compose run --rm rust bash 

bash# diesel setup

bas# diesel migration run

bash# cargo run
```