cmd_/root/cicv-r4l-3-Trbe/r4l_experiment/driver/002_completion/Module.symvers :=  sed 's/ko$$/o/'  /root/cicv-r4l-3-Trbe/r4l_experiment/driver/002_completion/modules.order | scripts/mod/modpost      -o /root/cicv-r4l-3-Trbe/r4l_experiment/driver/002_completion/Module.symvers -e -i Module.symvers -T - 