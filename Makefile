RUSTC ?= rustc
RFLAGS ?= -g

dummy1 := $(shell mkdir bin 2> /dev/null)

all:
	$(RUSTC) $(RFLAGS) -o bin/algorithms --lib crate.rc

check:
	RUST_LOG=rustc=0,::rt::backtrace $(RUSTC) $(RFLAGS) -o bin/test-algorithms --test crate.rc
	 $(DEBUGGER) bin/test-algorithms

check1:
	$(RUSTC) $(RFLAGS) -o bin/test-algorithms --test crate.rc
	export RUST_LOG=test-algorithms::algorithms=3 && $(DEBUGGER) bin/test-algorithms test_algorithms

clean:
	rm -rf bin
