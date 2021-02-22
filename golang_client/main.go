package main

import (
	"context"
	"fmt"
	"google.golang.org/grpc"
	pb "proto"
)


const (
	Address = "127.0.0.1:8888"
)


func main() {
	conn,err := grpc.Dial(Address,grpc.WithInsecure())
	if err != nil {
		fmt.Println("连接服务器失败",err)
	}

	defer conn.Close()
	c := pb.NewHelloServiceClient(conn)
	r,err := c.HelloWorld(context.Background(), &pb.HelloRequest{ Request: "Boss,I am grpc client!"})
	if err != nil {
		fmt.Println("请求grpc服务端失败！",err)
		return
	}
	fmt.Println("grpc服务端回应",r.Response)
}