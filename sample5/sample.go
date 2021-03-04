package main
import (
    "fmt"
)

func a(){
     b()
}
func main() {
    a := "1"
    b := "a"
    fmt.Println(a+b)
}
