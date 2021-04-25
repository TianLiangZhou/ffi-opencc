## ffi-opencc

该项目是将中文简体转换成繁体，使用`rust`构建动态链接库来给`php`调用。
该库主要是为了提高`php`转换繁体字的性能而构建（__特别是长文章__）。

### 环境

需要`php >= 7.4` 以上的版本并且开启了`FFI`扩展。如果你需要自己编译库还需要装`rust` 工具链。

还需要设置`php.ini` 中的`ffi.enable`为`On`。

该库需要依赖`opencc`库。

目前有两种方式来构建依赖。

1. 手动安装`opencc`
    
    > 比如: `yum install opencc` 或者 `apt-get install opencc` 甚至还你可以[自己编译](https://github.com/BYVoid/OpenCC)

2. 为库目录下的`lib/libopencc.so.1.1.2`创建软链

    ```shell
        [root@meshell]# ln -s ./lib/libopencc.so.1.1.2 /lib64/libopencc.so.1.1
        [root@meshell]# ln -s /lib64/libopencc.so.1.1 /lib64/libopencc.so
    ```

> 该目录不支持`window`环境，如果你需要请自行编译。

### Usage

该库提供六种转换用法：简转繁，繁转简，简转繁，繁转简，简转繁，繁转简。

```php
<?php

include __DIR__ . '/../src/OpenCC.php';

// new 方法支持opencc配置目录
$oc = \FastFFI\Opencc\OpenCC::new();

echo "简体转繁体: ", $oc->s2t("头可断，血可流"), "\n";

echo "繁体转简体: ", $oc->t2s("台湾省，是中華人民共和國法定的34個省級行政區之一，也是中華人民共和國法律上的两个岛屿省份之一，其轄區包括臺灣島及其附屬島嶼、澎湖列岛等地"), "\n";

echo "简体转台正繁体: ", $oc->s2tw("头可断，血可流"), "\n";

echo "台正繁体转简体: ", $oc->tw2s("台湾省，是中華人民共和國法定的34個省級行政區之一，也是中華人民共和國法律上的两个岛屿省份之一，其轄區包括臺灣島及其附屬島嶼、澎湖列岛等地"), "\n";

echo "简体转香港繁体: ", $oc->s2hk("头可断，血可流"), "\n";

echo "香港繁体转简体: ", $oc->hk2s("頭可斷，血可流"), "\n";

```

以上程序执行后的结果:

> 简体转繁体: 頭可斷，血可流
> 
> 繁体转简体: 台湾省，是中华人民共和国法定的34个省级行政区之一，也是中华人民共和国法律上的两个岛屿省份之一，其辖区包括台湾岛及其附属岛屿、澎湖列岛等地
> 
> 简体转台正繁体: 頭可斷，血可流
> 
> 台正繁体转简体: 台湾省，是中华人民共和国法定的34个省级行政区之一，也是中华人民共和国法律上的两个岛屿省份之一，其辖区包括台湾岛及其附属岛屿、澎湖列岛等地
> 
> 简体转香港繁体: 頭可斷，血可流
> 
> 香港繁体转简体: 头可断，血可流


[在线转换](http://loocode.com/tool/opencc/simplified-chinese-to-traditional-chinese)

### FAQ

- 在`centos`上执行失败?

  确定是不是`glibc`版本过低。可以使用`ldd lib/libffi_opencc.so` 来查看库信息。
  如果出现`/lib64/libc.so.6: version 'glibc_2.18' not found`就说明你服务的`glibc`版本过低。
  下载glibc编译升级，下载地址: `wget http://mirrors.ustc.edu.cn/gnu/libc/glibc-2.18.tar.gz` 
  
