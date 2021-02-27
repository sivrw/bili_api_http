# bili_api_http

这是个用于解析Bilibili视频的API， 可用于部署在某些地区的服务器来**解锁当地的流媒体内容**

演示站点(香港): https://hongkongbiliapi.sivrw.me

## 调用方法与官方API相似
### 举个🌰: 
官方: https://api.bilibili.com/x/player/playurl?avid=373866981&cid=285903176&qn=112

bili_api_http: https://hongkongbiliapi.sivrw.me/playurl?avid=373866981&cid=285903176&qn=112

可添加一个`fourk=0`参数使其支持4K视频
(感谢[@SocialSisterYi](https://github.com/SocialSisterYi)整理的API)

## av/BV/ss/ep只需要前缀带上ID
### 例如:

https://hongkongbiliapi.sivrw.me/video?id=ss37789

https://hongkongbiliapi.sivrw.me/video?id=av810872

https://hongkongbiliapi.sivrw.me/video?id=BV1Kr4y1A7RG

https://hongkongbiliapi.sivrw.me/video?id=ep380479

## 部署:

VPS： 编译完之后设置好环境变量即可

阿里云函数: 新建一个HTTP函数 -> 静态编译 -> 编译好之后在bootstrap添加一行`export ROCKET_PORT=9000`使其监听9000端口以及设置一个`SESSDATA`环境变量用于登陆 -> 打包上传阿里云函数