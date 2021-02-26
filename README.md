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

## 部署:

VPS： 设置好lib.rs里面make_header()函数的SESSDATA直接编译运行即可

阿里云函数: 新建一个HTTP函数 -> 设置好lib.rs里面make_header()函数的SESSDATA然后静态编译 -> 编译好之后在bootstrap添加一行`export ROCKET_PORT=9000`使其监听9000端口 -> 打包上传阿里云函数