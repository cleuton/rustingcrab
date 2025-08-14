package main

/*
#cgo CFLAGS:  -I${SRCDIR}/include
#cgo LDFLAGS: -L${SRCDIR}/lib -lgorust -ldl -lm -lpthread
#include "gorust.h"
*/
import "C"
import "fmt"

func main() {
	res := C.somar(C.int(10), C.int(20))
	fmt.Printf("Result from Rust: %d\n", int(res))
}
