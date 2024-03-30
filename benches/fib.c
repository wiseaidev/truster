#include <stdio.h>
#include <time.h>

int fib(int n) {
    if (n <= 1) {
        return n;
    }
    return fib(n-1) + fib(n-2);
}

int main() {
    int n = 45;
    int num_runs = 20;
    double total_time = 0;
    
    FILE* csv_file = fopen("fib-c.csv", "w");
    fprintf(csv_file, "Run,Time\n");

    for (int i = 0; i < num_runs; i++) {
        clock_t start_time = clock();
        fib(n);
        clock_t end_time = clock();
        double run_time = (double)(end_time - start_time) / CLOCKS_PER_SEC;
        total_time += run_time;
        printf("Run %d time taken: %f seconds\n", i+1, run_time);
        
        fprintf(csv_file, "%d,%f\n", i+1, run_time);
    }
    
    fclose(csv_file);
    
    double avg_time = total_time / num_runs;
    printf("Average time taken for %d runs: %f seconds\n", num_runs, avg_time);
    return 0;
}
