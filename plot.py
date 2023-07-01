#!/usr/bin/env python3

import argparse
import json
import os
import glob

from datetime import datetime
import matplotlib.pyplot as plt
import matplotlib.dates as mdates

time_format = "%Y-%m-%dT%H:%M:%S"

def plot(filename):
    time_arr = []
    cap_arr = []
    last_status = ""

    with open(filename, "r") as f:
        for line in f:
            j = json.loads(line)

            status = j["STATUS"]
            capacity = j["CAPACITY"]
            time = j["TIME"]
            time = datetime.strptime(time[:time.index(".")], time_format)

            if status != last_status:
                last_status = status
                cap_arr.append([])
                time_arr.append([])

            cap_arr[-1].append(capacity)
            time_arr[-1].append(time)

    for t, c in zip(time_arr, cap_arr):
        plt.plot(t, c)

def main():
    parser = argparse.ArgumentParser(prog="battery status plotter")
    parser.add_argument("file", nargs="*")
    args = parser.parse_args()
    if not args.file:
        files = glob.glob('/var/lib/battery-tracker/*')
        args.file = [max(files, key=os.path.getctime)]

    for file in args.file:
        plot(file)

    plt.xlabel("Time")
    plt.gca().xaxis.set_major_formatter(mdates.DateFormatter(time_format))
    plt.gcf().autofmt_xdate()

    plt.ylabel("Percentage")
    plt.gca().invert_yaxis()

    plt.title("Battery Status")
    plt.show()


if __name__ == "__main__":
    main()
