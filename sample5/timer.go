package main

import (
    "fmt"
    "time"
)

type Cosa struct {
    Dato string
}
func (c Cosa) Imprimir(){
    fmt.Println(c.Dato)
}

func sub(c *Cosa){
       time.Sleep(1 * time.Second)
       //fmt.Println("A")
       c.Imprimir()
       sub(c)
}

var m Cosa
func sub2(c *Cosa){
	c.Dato = "DDD"
       time.Sleep(5 * time.Second)
       fmt.Println("AAAAAAAA")
       sub2(c)
}

func main() {
    m = Cosa{Dato:"A"}
    go sub2(&m)
    sub(&m)

}
