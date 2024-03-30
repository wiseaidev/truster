import pandas as pd
import matplotlib.pyplot as plt

file_names = ["fib-c.csv", "fib-rs.csv", "fib-go.csv", "fib-ts.csv", "fib-jl.csv" ]
file_names = [f"dataset/{file_name}" for file_name in file_names]
langs = ["C", "Rust", "Go", "TypeScript", "Julia" ]

colors = ['#d25154', '#F15854', '#4DC1DA', '#5DA5DA', '#F17CB0']

fig, ax = plt.subplots(nrows=1, ncols=1, figsize=(32, 32))

for i, file in enumerate(file_names):
    data = pd.read_csv(file)

    mean_time = data['Time'].mean()

    run_data = data
    ax.plot(run_data["Run"] , run_data['Time'], colors[i])
    ax.set_ylim([0, 20])
        
    ax.axhline(y=mean_time, color=colors[i], linestyle='--', label=langs[i])
        
    fig.tight_layout(pad=20.0)

plt.title('Average time taken for computing 45th Fibonacci sequence')
plt.xlabel('Programming Language')
plt.legend(loc='upper center', bbox_to_anchor=(0.95, 0.95))
plt.ylabel('Time (seconds)')
plt.show()
