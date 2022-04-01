# jloc (json locator)

以行为单位读取json格式数据，并按指定key展示value，或指定key和value输出符合条件的数据行。

## demo

1. 输入单行json数据，0个参数，格式化输出，用于查看单条

![image](https://github.com/Shikai-Jin/jloc/blob/main/readme_gif/1.gif)



2. 输入多行json数据，以key为参数，输出每行该key的value。用于批量过滤。

![image](https://github.com/Shikai-Jin/jloc/blob/main/readme_gif/2.gif)

_多级之间用'.'分隔，列表索引使用'[ ]'_



3. 输入多行数据，在2的基础上用'='指定value值，输出每行该key的value为指定值的整行。

![image](https://github.com/Shikai-Jin/jloc/blob/main/readme_gif/3.gif)



4. 配合管道多次使用，用于按条件筛选指定数据

![image](https://github.com/Shikai-Jin/jloc/blob/main/readme_gif/4.gif)



## start

可以简单地将release的二进制文件添加进环境变量，或从源码编译



## build

cargo build --release



## notice

1. 参数以'.'分隔多级key、以'[]'表示列表索引、以'='表示指定值，因此key里不能出现'.'、'['、']'、'='
2. '='后指定值会将字符串解析为json的value再比较，如会将参数"=true"解析为布尔true
3. 如果'='后的值是字符串，需要用转义符\\"和\\"包起来。eg: "country=\\"China\\""
