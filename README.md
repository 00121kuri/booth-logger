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
docker run --rm -d -p 4444:4444 -p 5900:5900 --name selenium-server -v /dev/shm:/dev/shm selenium/standalone-chrome:4.1.0-20211123
```

### envファイル
```
SHOP_URL=https://shopdomain.booth.pm
SHOP_NAME=shop-name
SELENIUM_URL=http://localhost:4444
```