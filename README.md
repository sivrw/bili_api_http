# bili_api_http

这是个用于解析Bilibili视频的API， 可用于部署在某些地区的服务器来**解锁当地的流媒体内容**

演示站点(香港): https://hongkongbiliapi.sivrw.me

## 调用方法与官方API相似
### 举个🌰: 
官方: https://api.bilibili.com/x/player/playurl?avid=373866981&cid=285903176&qn=112
bili_api_http: https://hongkongbiliapi.sivrw.me/playurl?avid=373866981&cid=285903176&qn=112


## av/BV/ss/ep只需要前缀带上ID
### 例如:

https://hongkongbiliapi.sivrw.me/video?id=ss37789

https://hongkongbiliapi.sivrw.me/video?id=av810872

https://hongkongbiliapi.sivrw.me/video?id=BV1Kr4y1A7RG

https://hongkongbiliapi.sivrw.me/video?id=ep380479