#!/bin/bash
if [ -z "$1" ] || [ -z "$2" ] || [ -z "$3" ]
then
        echo "The (1)# of sub peers and the (2)payload size (3)unit (possible value: msgs/s, MB/s) should not be empty"
else
        kill $(ps -a | grep z_sub_thr | awk '{print $1}')    # Check if there are subscribers exist, kill them before experiments start
        ls | grep result | xargs -d"\n" rm    # Clear the previous results
        for ((i=8; i<= $2; i*=2))
        do
            echo "test throughput for payloadSize $i bytes:"
            echo "Satrt $1 subscribers"
            for ((j=1; j<= $1; j++))
            do
                if (( $j % 3 == 1 ))
                then
                    taskset -c 2 ./z_sub_thr -u $3 -p $i -n 10000 >> "result$j.txt" &
                elif (( $j % 3 == 2))
                then
                    taskset -c 4 ./z_sub_thr -u $3 -p $i -n 10000 >> "result$j.txt" &
                else
                    taskset -c 6 ./z_sub_thr -u $3 -p $i -n 10000 >> "result$j.txt" &
                fi
            done
            while [[ ! -z $(ps -a| grep z_sub_thr) ]]
            do
                sleep 0.1
            done
            echo "End all subscribers"
            pid=$(ps -a | grep z_pub_thr | awk '{print $1}')
            kill $pid
        done
fi
