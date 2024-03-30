package main

import (
  "encoding/csv"
  "fmt"
  "os"
  "time"
)

func fibonacci(n int) int {
  if n <= 1 {
    return n
  } else {
    return fibonacci(n-1) + fibonacci(n-2)
  }
}

func main() {
  totalTime := time.Duration(0)
  file, err := os.Create("fib-go.csv")
  if err != nil {
    panic(err)
  }
  defer file.Close()

  writer := csv.NewWriter(file)
  defer writer.Flush()

  fmt.Fprintf(file, "Run,Time\n")

  for i := 0; i < 20; i++ {
    startTime := time.Now()
    fibonacci(45)
    endTime := time.Now()
    executionTime := endTime.Sub(startTime).Seconds()
    totalTime += endTime.Sub(startTime)
    fmt.Printf("Run %d time taken: %v seconds\n", i+1, executionTime)
    writer.Flush()
    fmt.Fprintf(file, "%d,%f\n", i+1, executionTime)
  }
  fmt.Printf("Average time taken: %v s\n", totalTime.Seconds()/20)
}
