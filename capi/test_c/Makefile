UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Linux)
    LDL = -ldl
else
    LDL = -lSystem
endif

all:
	gcc -Wall -o main main.c -I.. -L../target/debug -lembed_capi $(LDL) -lpthread -lc -lm

clean:
	rm -f main
