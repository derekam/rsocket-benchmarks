package main

import (
	"bufio"
	"fmt"
	"github.com/rsocket/rsocket-go/payload"
	"io/ioutil"
	"log"
	"os"
	"path/filepath"
)

func main() {
	f, _ := os.Create("go_results.csv")
	defer f.Close()
	w := bufio.NewWriter(f)

	var counts = [...]int { 10_000_000, 1_000_000, 100_000, 10_000, 1_000, 100 }

	var files []string

	root := "../resources"
	err := filepath.Walk(root, visit(&files))
	if err != nil {
		panic(err)
	}
	bench := &Benchmark{nil, counts[0], nil}
	go startserver(bench)
	for i, file := range files[1:] {
		fmt.Println(file)
		// Open our jsonFile
		jsonFile, err := os.Open(file)
		// if we os.Open returns an error then handle it
		if err != nil {
			fmt.Println(err)
		}
		fmt.Println("Successfully Opened json")
		// read our opened jsonFile as a byte array.
		byteValue, _ := ioutil.ReadAll(jsonFile)
		pl := payload.New(byteValue, nil)
		bench.data = pl
		bench.count = counts[i]
		runbenchmarks(*bench, w)
	}

	fmt.Println("done")


	w.Flush()

}

func visit(files *[]string) filepath.WalkFunc {
	return func(path string, info os.FileInfo, err error) error {
		if err != nil {
			log.Fatal(err)
		}
		*files = append(*files, path)
		return nil
	}
}