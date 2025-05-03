RL File Agent that monitors and organizes filesystem activity through kernel-level instrumentation.

After launching vm with launch_vm.sh (files will auto-update with this script), to mount the shared directory, you must run run_in_vm.sh in the vm. 

Once that setup is done, go to /mnt/hostshare/rl_file_agent in the vm, and run build_and_run.sh