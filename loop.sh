#!/bin/bash

# 默认值
IP="127.0.0.1"
PORT="8080"
NUM=5

# 解析命令行参数
while getopts "h:p:n:" opt; do
    case "$opt" in
        h) IP="$OPTARG" ;;
        p) PORT="$OPTARG" ;;
        n) NUM="$OPTARG" ;;
        ?) echo "Usage: $0 [-h ip] [-p port] [-n num]"; exit 1 ;;
    esac
done

# 确保 num 是正整数
if ! [[ "$NUM" =~ ^[0-9]+$ ]] || [ "$NUM" -lt 1 ]; then
    echo "Error: num must be a positive integer"
    exit 1
fi

# 循环执行 curl
for ((i=1; i<=NUM; i++)); do
    URL="http://$IP:$PORT/$i"
    echo "Request $i: $URL"
    curl -s "$URL" || echo "Failed to connect to $URL"
done

