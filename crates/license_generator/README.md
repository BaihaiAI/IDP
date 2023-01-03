# License生成器（license_generator）

## 使用License启动IDP

### 启动
IDP需要在启动时指定合法的License文件与公钥pem文件才能启动，启动时通过参数`--license`和`--public-key`传入路径。如使用：
```
idp --license license --public-key rsa2048-pub.pem
```
这会传入与idp同一路径下的“license”文件和“rsa2048-pub.pem”文件。
生成License文件与公钥pem文件的方式见下文。

### License要求
- 需要在未过期时间内
- 版本要对应
- 签名要能通过验证

## 使用 openssl 生成一对公私钥
使用 license_generator 生成 License 需要提供私钥，私钥文件可由 openssl 生成。
在安装有 openssl 的系统上如下指令，将生成一个名为 rsa2048-priv.pem 的私钥文件，用于传入 license_generator：

私钥文件的格式要求是 rsa PKCS#1 with pem

通过 openssl version 查看您系统的 openssl 选择不同的命令参数进行生成

openssl 1.1:

> openssl genrsa -out rsa2048-priv.pem

openssl 3.0:

> openssl genrsa -traditional -out rsa2048-priv.pem

最后通过私钥文件生成出公钥文件

> openssl rsa -in rsa2048-priv.pem -pubout -out rsa2048-pub.pem

## 使用生成器（license_generator）生成License和公钥文件

### 需要配置的环境变量
生成器通过配置环境变量来配置如下内容：
- 生成器读取的私钥文件名（由openssl生成）：PRIV_KEY_PATH
- License的过期天数：LICENSE_EXPIRE_DAYS
- 生成器输出的License文件名：LICENSE_PATH
- 生成器输出的公钥文件名：PUB_KEY_PATH

### license 生成器使用
> ./license_generator --pri-key rsa2048-priv.pem

必要的参数 --pri-key 传入私钥路径，可选参数 --expire-in-days 设置输出的 license 文件将在多少天后过期

### 分发使用
将生成器生成的License和公钥pem文件发送给使用者用于启动idp
