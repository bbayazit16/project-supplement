package main

import (
	"bytes"
	"fmt"
	"image"
	"image/jpeg"
	"io/ioutil"
	"net/http"
	"os"
	"path/filepath"
	"strconv"
	"time"
)

func write(imgbytes []byte, fname string, fn string) {
	img, _, err := image.Decode(bytes.NewReader(imgbytes))
	if err != nil {
		fmt.Println(err)
	}
	flname := fname + "/" + fn + ".jpg"
	out, _ := os.Create(flname)
	defer out.Close()
	var opts jpeg.Options
	opts.Quality = 1024
	err = jpeg.Encode(out, img, &opts)
	if err != nil {
		fmt.Println(err)
	}
}

func get() []byte {
	url := "https://thispersondoesnotexist.com/image"
	response, err := http.Get(url)
	if err != nil {
		fmt.Println(err)
	}
	val, err := ioutil.ReadAll(response.Body)
	if err != nil {
		fmt.Println(err)
	}
	return val
}

func gen(fname string) string {
	fpath, perr := os.Executable()
	if perr != nil {
		panic(perr)
	}
	return filepath.Dir(fpath) + "/" + fname
}

func fold(fname string) {
	fipath := gen(fname)
	err := os.Mkdir(fipath, 0755)
	if err != nil {
		ferr := os.RemoveAll(fipath)
		if ferr != nil {
			panic(ferr)
		}
		fold(fname)
	}
}

func main() {
	finame := gen("doesntexistimages")
	fold("doesntexistimages")
	fmt.Println(finame)
	i := 0
	for {
		write(get(), finame, strconv.Itoa(i))
		time.Sleep(1 * time.Second)
		i++
	}
}
