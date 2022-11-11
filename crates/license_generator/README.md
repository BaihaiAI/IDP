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
## 使用openssl生成私钥
使用license_generator生成License需要提供私钥，私钥文件可由openssl生成。
在安装有openssl的系统上如下指令，将生成一个名为rsa2048-priv.pem的私钥文件，用于传入license_generator：
```
openssl genrsa -out rsa2048-priv.pem
openssl rsa -in rsa2048-priv.pem -pubout -out rsa2048-pub.pem
```
## 使用生成器（license_generator）生成License和公钥文件
### 需要配置的环境变量
生成器通过配置环境变量来配置如下内容：
- 生成器读取的私钥文件名（由openssl生成）：PRIV_KEY_PATH
- License的过期天数：LICENSE_EXPIRE_DAYS
- 生成器输出的License文件名：LICENSE_PATH
- 生成器输出的公钥文件名：PUB_KEY_PATH
### 执行生成器
执行如下指令进行License和公钥文件的生成
```
env PRIV_KEY_PATH="rsa2048-priv.pem" LICENSE_EXPIRE_DAYS=30 LICENSE_PATH="license" PUB_KEY_PATH="rsa2048-pub.pem" ./license_generator
```
其中env用于设置环境变量，按照上方列出的含义，此命令将会：
1. 让生成器读取名为“rsa2048-priv.pem”的私钥文件（PRIV_KEY_PATH="rsa2048-priv.pem"）
2. 将过期时间设置为30天（LICENSE_EXPIRE_DAYS=30）
3. 生成的license输出到文件“license”（LICENSE_PATH="license"）
4. 生成的公钥输出到文件“rsa2048-pub.pem”（PUB_KEY_PATH="rsa2048-pub.pem"）
具体的环境变量可以根据需要自行设置。
### 分发使用
将生成器生成的License和公钥pem文件发送给使用者用于启动idp。将openssl生成的私钥文件自行保存。