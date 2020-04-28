package main

import (
	"fmt"
	"github.com/panjf2000/gnet"
	"log"
	"regexp"
	"strconv"
	"sync"
)

type SafeCounter struct {
	v   int64
	mux sync.Mutex
}

func (c *SafeCounter) Inc(value int64) {
	c.mux.Lock()
	c.v += value
	c.mux.Unlock()
}

func (c *SafeCounter) Get() int64 {
	c.mux.Lock()
	defer c.mux.Unlock()
	return c.v
}

var x = SafeCounter{}

type echoServer struct {
	*gnet.EventServer
}
var RES = ([]byte)("HTTP/1.1 200 OK\n\n")
var REG = regexp.MustCompile("\\D+")
func (es *echoServer) React(frame []byte, c gnet.Conn) (out []byte, action gnet.Action) {
	rawReq := (string)(frame)
	if rawReq[0:1] == "G" {
		out = ([]byte)(fmt.Sprintf("HTTP/1.1 200 OK\n\n%d\r\n", x.Get()))
		_ = c.Close()
		return
	}
	result, _ := strconv.ParseInt(REG.ReplaceAllString(rawReq[len(rawReq)-10:], ""), 10, 64)
	x.Inc(result)
	out = RES
	_ = c.Close()
	return
}

func main() {
	echo := new(echoServer)
	log.Fatal(gnet.Serve(echo, "tcp://:80" ))
}

