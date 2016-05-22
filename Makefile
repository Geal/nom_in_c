UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Linux)
    LDL = -ldl
else
    LDL = -lSystem
endif

all: main

main: embed_capi src/main.c
	gcc -Wall -o main src/main.c -Iembed_capi -Lembed_capi/target/debug -lembed_capi $(LDL) -lpthread -lc -lm

embed_capi: embed

embed embed_capi:
	cd $@ && cargo build

clean:
	rm -f main

.PHONY: embed embed_capi
