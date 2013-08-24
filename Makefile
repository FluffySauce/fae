RC=rustc
RCFLAGS=-O
LDFLAGS=-L$(HOME)/lib -L$(shell pwd)/lib
TESTS=bin/simple bin/noise bin/noiseimage

.PHONY: all
all: fae $(TESTS)

.PHONY: fae
fae:
	$(RC) $(RCFLAGS) $(LDFLAGS) --out-dir=lib fae.rs

bin/simple: tst/simple.rs fae
	$(RC) $(RCFLAGS) $(LDFLAGS) -o $@ $<

bin/noise: tst/noise.rs fae
	$(RC) $(RCFLAGS) $(LDFLAGS) -o $@ $<

bin/noiseimage: tst/noiseimage.rs fae
	$(RC) $(RCFLAGS) $(LDFLAGS) -o $@ $<

.PHONY: clean
clean:
	-rm -f $(TESTS)
	-rm -rf lib/libfae*
	-rm -rf lib/*.dSYM
	-rm -rf bin/*.dSYM
