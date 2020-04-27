package main

import (
	"bufio"
	"fmt"
	"log"
	"net"
	"strconv"
	"strings"
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

func main() {
	ln, err := net.Listen("tcp", ":80")
	if err != nil {
		log.Println(err)
		return
	}
	for {
		conn, err := ln.Accept()

		if err != nil {
			log.Println(err)
			return
		}

		go requestHandler(conn)

	}
}

var x = SafeCounter{}

func requestHandler(conn net.Conn) {
	//_ = conn.SetDeadline(time.Now().Add(time.Millisecond * 2))
	reader := bufio.NewReader(conn)
	scanner := bufio.NewScanner(reader)

	firstLine := ""
	for len(firstLine) == 0 {
		for scanner.Scan() {
			firstLine = scanner.Text()
			if len(firstLine) > 0 {
				log.Println(scanner.Text())
				break
			}
		}
	}

	is_post := firstLine[0:1] == "P"

	if is_post {
		_, _ = conn.Write([]byte("HTTP/1.1 200 OK\n\n"))
		_ = conn.Close()
		last := ""
		for scanner.Scan() {
			last = scanner.Text()
		}
		num, _ := strconv.ParseInt(strings.Trim(last, " \n\r"), 10, 64)

		x.Inc(num)

	} else {
		s := fmt.Sprintf("HTTP/1.1 200 OK\n\n%d\r\n", x.Get())

		_, _ = conn.Write([]byte(s))
		_ = conn.Close()
	}
}
