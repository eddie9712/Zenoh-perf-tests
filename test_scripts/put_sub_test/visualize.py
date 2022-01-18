import matplotlib.pyplot as plt
import os
import re

calculate_average = []
throughput_average = []
collect_payloadsize = []
unit = ""

results = os.popen('ls | grep result')
files = results.read()
result_files = re.split("\n", files)
result_files.pop()


for f in result_files:
    fptr = open(f)
    collect_payloadsize.clear()
    calculate_average.clear()
    throughput_average.clear()
    for line in fptr:
        if re.search("Start", line):
            payload = re.split("\s", line)
            collect_payloadsize.append(int(payload[5]))
        elif re.search("End", line):
            sum = 0
            num = 0
            for idx, val in enumerate(calculate_average):
                sum += val
                num = idx
            throughput_average.append(sum/(num+1))
            calculate_average.clear()
        else:
            if re.search("msg/s", line):    # Read raw data, and detect the unit
                unit = "msgs/s"
            else:
                unit = "MB/s"
            data = re.split("\s", line)
            calculate_average.append(float(data[0]))
    plt.plot(collect_payloadsize, throughput_average, marker = 'o')

plt.xscale("log", basex=2)
plt.xticks(collect_payloadsize)
plt.xlabel("payload size (Bytes)")
plt.ylabel("Throughput: "+unit)
plt.grid()
plt.show()
