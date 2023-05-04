package main

import (
	"fmt"
	"io"
	"net/http"
	"os"
)

func main() {
	for i := 1; i < 470; i++ {
		url := fmt.Sprintf("http://heshun.tjc.org.tw/Hymns/%03d.mp3", i)
		fileName := fmt.Sprintf("../hymns/%03d.mp3", i)

		file, err := os.Create(fileName)
		fmt.Println(fileName)
		if err != nil {
			fmt.Println(err)
		}

		res, err := http.Get(url)
		fmt.Println(url)
		if err != nil {
			fmt.Println(err)
		}
		fmt.Printf("status: %s \n", res.Status)

		_, err = io.Copy(file, res.Body)
		if err != nil {
			fmt.Println(err)
		}

		res.Body.Close()
		file.Close()
	}
}
