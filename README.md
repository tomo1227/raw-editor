# raw-editor

## raw-ppm

RAW画像をグレースケールのPPM形式に変換する。

```sh
cd raw-ppm
cargo run ../sample.ARW
```

sample.ARWと同じ階層にsample.ppmが生成される。

## raw-demosaic

RAW画像をデモザイク処理してカラー画像に変換する。

```sh
cd raw-demosaic
cargo run ../sample.ARW
```

sample.ARWと同じ階層にsample.ppmが生成される。
