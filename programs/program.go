package main

//0m0.062s
//0m0.031s
//0m0.049s

// missing file mandatory check
// int value mandatory check
// undefined is not ok

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func x() {
	fmt.Println("x was called")
}

func main() {

	//var varname *string
	//varname = nil

	//fmt.Println(strings.ToUpper(*varname))

	for x := 0; x < 1000; x++ {
		//if false {
		//	this_never_happens()
		//}
		data, err := ioutil.ReadFile("input.txt")
		if err != nil {
			fmt.Println("File reading error", err)
			return
		}
		lines := strings.Split(string(data), "\n")
		sum := 0
		for l := range lines {
			line := lines[l]
			if line == "" {
				continue
			}
			value, err := strconv.Atoi(line)

			if err != nil {
				fmt.Println("not an int:"+line+":", err)
				continue
			}
			sum += value

		}
		fmt.Println("SUM:", sum)
	}

}
