# golang-rust-grpc


## golang_client

### 环境设置
```
export GOPROXY=https://goproxy.io
go get -u github.com/golang/protobuf/{proto,protoc-gen-go}
将protoc-gen-go加入环境变量：
echo export PATH="$PATH:/Users/lr/go/bin" >> ~/.bash_profile
source ~/.bash_profile
```

### 编译proto文件
```
cd ./proto
protoc -I . --go_out=plugins=grpc:. ./mygrpc.proto
```

### 编写golang_client实现代码


### go mod设置
```
go mod init golang_client

go mod tidy
```


### 把proto目录移动到vendor下
```
lrdeMacBook-Pro:golang_client lr$ ll vendor
total 8
drwxr-xr-x  3 lr  staff    96 Feb 22 10:10 github.com
drwxr-xr-x  3 lr  staff    96 Feb 22 10:10 golang.org
drwxr-xr-x  5 lr  staff   160 Feb 22 10:10 google.golang.org
-rw-r--r--  1 lr  staff  3888 Feb 22 10:10 modules.txt
drwxr-xr-x  4 lr  staff   128 Feb 22 10:11 proto
```

### 重启编译器

### 编译
```
CGO_ENABLED=0 GOOS=darwin GOARCH=amd64 go build -o ./bin/mygrpc_client .
```
