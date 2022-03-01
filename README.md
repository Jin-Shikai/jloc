# jloc

一个简易的在命令行中以行为单位解析json格式数据的小工具。

## demo

1. 输入单行json数据，格式化输出，用于查看单条

![image](https://github.com/Shikai-Jin/jloc/blob/main/readme_gif/1.gif)



2. 输入多行json数据，以key为参数，输出每行该key的value。用于批量过滤。

![image](https://github.com/Shikai-Jin/jloc/blob/main/readme_gif/2.gif)

_多级之间用'.'分隔，列表索引使用'[ ]'_



3. 输入多行数据，以key为参数并用'='指定value值，输出每行该key的value为指定值的整行。

![image](https://github.com/Shikai-Jin/jloc/blob/main/readme_gif/3.gif)



4. 配合管道多次使用，用于按条件筛选指定数据

![image](https://github.com/Shikai-Jin/jloc/blob/main/readme_gif/4.gif)



## start

可以简单地将release版本的二进制文件添加进环境变量，或从源码编译



## build

cargo build --release



## 已知问题

1. 参数以'.'分隔多级key、以'[]'表示列表索引、以'='表示指定值，因此对key里有'.'、'['、']'、'='的数据不能解析
2. '='后指定值会将字符串解析为json的value再比较，如会将参数"=true"解析为布尔true，如果值为字符串"true"不能识别。



## 说明

1. 该项目的性质是玩具，主要目的是自娱自乐、学习rust基本语法与可能存在的工作效率提升；
2. 我的工作内容不覆盖已知问题场景，暂无计划修复；
3. 如果对解析json有正式的需求，推荐使用jq等工具
