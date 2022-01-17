#!/bin/bash
if [ -z "$1" ] || [ -z "$2" ]
then
        echo "The (1)# of peers and (2)payload size should not be empty"
else
        for ((i=8; i<= $2; i*=2))
        do
                echo "test throughput for payloadSize $i bytes:"
                echo "start publisher"
                num_of_peers=$(ps -a | grep z_sub_thr | wc -l)
                echo "$num_of_peers"
                while (( $num_of_peers != $1 ))  #check if all subscribers launched
                do
                      sleep 0.1
                done
                taskset -c 0 ./z_pub_thr $i
        done
fi

