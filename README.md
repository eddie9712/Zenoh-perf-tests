# Zenoh-perf-tests
A repository for testing Zenoh performance
## Recommand to follow these [guides](https://easyperf.net/blog/2019/08/02/Perf-measurement-environment-on-Linux) to get consistent results:  
1. Disable turbo boost:  
`echo 1 > /sys/devices/system/cpu/intel_pstate/no_turbo`  
2. Set CPU affinity:  
`taskset -c $cpu_number $your_process`
3. Set process priority: 
 `nice -n $priority_number $your_process`
⚠️To set the CPU affinity and the process priority at the same time:    
`nice -n $priority taskset -c $cpu_number $your_process`
4. Disable hyperthreading: 
`echo 0 | sudo tee /sys/devices/system/cpu/$cpu_num/online` 
⚠️Generally, we can disable the odd cpu number (disable one thread in a physical core) 
6. Set scaling_governor to "performance" 
`echo performacne > /sys/devices/system/cpu/$cpu_num/cpufreq/scaling_governor` 
8. Do one dry run to warm the cache

## Architecture:
The `test_scripts` folder: Shell scripts to build specific scenarios, and the visualization programs  
The `zenoh_src` folder: The primitive rust testing program which are used for test_scripts

## Quick start:  
Build rust executable files:  
```
cd zenoh_src
cargo build --all-targets --release
```
Then move the executable files to the specific scenario folder:  
```
cd target/release/examples
mv ./$executable_file ../../../../test_scripts
```
Finally, run the script:
`./$script_name`
