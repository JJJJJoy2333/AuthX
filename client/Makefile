CC = gcc
CFLAGS = -I.
LDFLAGS = -L../target/release -lauthx

# On Linux, we need to set the library path to find the .so
RUN_ENV = LD_LIBRARY_PATH=../target/release

all: test_authx

test_authx: test_authx.c authx.h
	$(CC) $(CFLAGS) test_authx.c -o test_authx $(LDFLAGS)

run: test_authx
	$(RUN_ENV) ./test_authx

clean:
	rm -f test_authx
