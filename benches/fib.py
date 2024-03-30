import time
import csv

def fibonacci(n):
    if n <= 1:
        return n
    else:
        return fibonacci(n-1) + fibonacci(n-2)

if __name__ == "__main__":
    total_time = 0
    with open('fib-py.csv', mode='w') as csv_file:
        fieldnames = ['Run', 'Time']
        writer = csv.DictWriter(csv_file, fieldnames=fieldnames)

        writer.writeheader()
        for i in range(1, 21):
            start_time = time.time()
            fibonacci(45)
            elapsed_time = time.time() - start_time
            total_time += elapsed_time
            print(f"Run {i} Time taken: {elapsed_time} seconds")
            writer.writerow({'Run': i, 'Time': elapsed_time})

        print(f"Average time taken: {total_time/20}s")
