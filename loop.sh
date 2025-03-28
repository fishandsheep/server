#!/bin/bash

# 检查参数是否正确
if [ "$#" -ne 3 ]; then
    echo "Usage: $0 <ip> <port> <num>"
    exit 1
fi

# 解析参数
IP="$1"
PORT="$2"
NUM="$3"

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

