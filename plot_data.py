import matplotlib.pyplot as plt

size = [10000, 20000, 30000, 40000, 50000, 60000, 70000, 80000, 90000, 100000]

plt.figure(figsize=(10, 6))
plt.title('Rust program')
plt.xlabel('data size')
plt.ylabel('processing time (s)')
plt.grid(True)

f = open('output.txt', 'r', encoding='UTF-8')

datalist = f.readlines()
for c, data in enumerate(datalist, start=1):
    match c:
        case 1:
            label = "selection_sort"
        case 2:
            label = "insertion_sort"
        case 3:
            label = "marge_sort"
        case 4:
            label = "quick_sort"
    t = "".join(data.split(": ")[1]).strip().lstrip('[').rstrip(']')
    times = list(map(float, t.split(",")))
    print(times)
    plt.plot(size, times, label=label)
    plt.legend()

f.close()

plt.savefig("output.jpg")
plt.show()
