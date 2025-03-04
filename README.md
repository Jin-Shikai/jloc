# jloc (json locator)

Read JSON format data in lines and display the value according to the specified key, or specify the key and value to output the data rows that meet the conditions.

## demo

1. Input a single line of JSON data, 0 parameters, formatted output, used to view a single line

![image](https://github.com/Shikai-Jin/jloc/blob/main/readme_gif/1.gif)



2. Input multiple lines of JSON data, take key as parameter, and output the value of each key. Used for batch filtering.

![image](https://github.com/Shikai-Jin/jloc/blob/main/readme_gif/2.gif)

_Multiple levels are separated by '.' and list indexes use '[ ]'_



3. Input multiple lines of JSON data, take key as parameter, and output the value of each key. Used for batch filtering.

![image](https://github.com/Shikai-Jin/jloc/blob/main/readme_gif/3.gif)



4. Used with pipelines multiple times to filter specified data by conditions

![image](https://github.com/Shikai-Jin/jloc/blob/main/readme_gif/4.gif)



## start

You can simply add the release binary file (https://github.com/Shikai-Jin/jloc/releases) to your environment variables, or compile from source


## build from source

cargo build --release



## notice

1. Parameters use '.' to separate multi-level keys, '[]' to represent list indexes, and '=' to represent specified values, so '.', '[', ']', and '=' cannot appear in the key
2. The specified value after '=' will parse the string into a json value for comparison, such as parsing the parameter "=true" as Boolean true
3. If the value after '=' is a string, it needs to be wrapped with escape characters \\" and \\". Eg: "country=\\"China\\""
