# booth-logger
ショップの公開情報を取得します

取得する情報
- 商品名
- 商品id
- URL
- スキ！数
- 価格

## Setting

### Selenium
```
docker run -d -p 4444:4444 --shm-size="2g" selenium/standalone-chrome:4.11.0-20230801
```

### envファイル
```
SHOP_URL=https://shopdomain.booth.pm
SHOP_NAME=shop-name
SELENIUM_URL=http://localhost:4444
```