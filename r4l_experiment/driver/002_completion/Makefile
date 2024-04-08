
KDIR := ../../../linux
PWD := $(shell pwd)

default:
	$(MAKE) LLVM=1 -C $(KDIR) M=$(PWD)

all:
	$(MAKE) LLVM=1 -C $(KDIR)  M=$(PWD) modules

.PHONY: clean
clean:
	rm -f *.ko *.o .*.cmd .*.o.d *.mod *.mod.o *.mod.c *.symvers *.markers *.unsigned *.order *~
