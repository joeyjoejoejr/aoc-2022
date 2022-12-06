CC = cc
CFLAGS = -Wall -Wextra -pedantic
RUSTC = rustc
RFLAGS = #--cfg test_input
OUTDIR = target
TARGETS = $(basename $(subst src,$(OUTDIR),$(wildcard src/*.*)))

all: $(TARGETS)

$(OUTDIR):
	mkdir -p $(OUTDIR)

$(OUTDIR)/%: src/%.c $(OUTDIR)
	$(CC) $(CFLAGS) $< -o $@

$(OUTDIR)/%: src/%.rs $(OUTDIR)
	$(RUSTC) $(RFLAGS) $< -o $@

clean:
	rm -rf $(OUTDIR)
