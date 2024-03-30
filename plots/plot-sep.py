import pandas as pd
import matplotlib.pyplot as plt

file_names = ["fib-c.csv", "fib-rs.csv", "fib-go.csv", "fib-jl.csv", "fib-ts.csv", "fib-py.csv" ]
file_names = [f"dataset/{file_name}" for file_name in file_names]
langs = ["C", "Rust", "Go", "Julia", "TypeScript", "Python" ]

fig, axes = plt.subplots(nrows=2, ncols=3, figsize=(32, 32))

for (i, file), ax in zip(enumerate(file_names), axes.flatten()):
    data = pd.read_csv(file)

    mean_time = data['Time'].mean()

    run_data = data
    ax.plot(run_data["Run"] , run_data['Time'], '-o')
    ax.set_title(langs[i])
    ax.set_ylim([min(data['Time']) / 1.1, 1.1 * max(data['Time'])])
        
    ax.axhline(y=mean_time, color='r', linestyle='--', label='Mean')
    ax.legend(loc='upper right', bbox_to_anchor=(0.95, 0.95))
        
    fig.suptitle('45th Fibonacci Sequence Execution Time', fontsize=16)

    fig.tight_layout(pad=16.0)

plt.show()
