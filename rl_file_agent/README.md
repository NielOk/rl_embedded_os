RL File Agent that monitors and organizes filesystem activity through kernel-level instrumentation.

First, to copy the scripts for rl_file_agent to the shared directory, run copy_files_to_rl_test.sh. After launching vm with launch_vm.sh, to mount the shared directory, you must run run_in_vm.sh in the vm. 

Once that setup is done, go to /mnt/hostshare/rl_file_agent in the vm, and run build_and_run.sh