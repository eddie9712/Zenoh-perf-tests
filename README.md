# Zenoh-perf-tests
A repository for testing Zenoh performance
## Recommand to follow these [guides]() to get consistent results:  
1. Disable turbo boost:  

2. Set CPU affinity:  
3. Set process priority:  
⚠️To set the CPU affinity and the process priority at the same time:    
`nice -n $priority taskset -c $cpu_number $your_process`
4. Disable hyperthreading:  
5. Set scaling_governor to "performance"  
6. Do one dry run to warm the cache
