# Snapshot

Snapshot 是一个用 Rust 编写的网络视频截图程序，在 config.toml 中配置后，可以按指定的频率调用 ffmpeg 截取网络视频中的一帧保存到本地文件。

## 使用说明
**Linux 查找 ffmpeg 路径**  

命令：
```shell
which ffmpeg
```
输出结果:
```shell
 /usr/bin/ffmpeg
 ```

 **资源ID**

资源唯一 id，会作为 storage.local 的子目录存在，截取的图片保存在这里。资源 id 需要保持唯一性，否则重复 id 的图片将会保存到同一个目录下造成冲突。资源 id 的名字可以是数字、字母、下划线等操作系统允许的字符，建议使用数字或字母来作为 id。

**频率控制**  

配置文件中的 src.frequency 表示当前资源截图的频率（单位：秒），如果资源较多，系统配置较低，应避免频率在同一时刻，以减少系统开销。

**资源网络地址**

配置文件中的 src.url 表示截图资源的网络地址，支持 http、rtsp、rtmp 等。

**存储**

目前支持本地存储，未来会支持更多。 storage.local 用以配置本地存储路径。

**新增资源**

添加新的截图资源，只要在 config.toml 中参照 src 的内容新增 src 节点即可。
