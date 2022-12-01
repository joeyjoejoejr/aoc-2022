CC = cc
CFLAGS = -Wall -Wextra -pedantic
OUTDIR = target
TARGETS = $(basename $(subst src,$(OUTDIR),$(wildcard src/*.c)))

all: $(TARGETS)

$(OUTDIR):
	mkdir -p $(OUTDIR)

$(OUTDIR)/%: src/%.c $(OUTDIR)
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -rf $(OUTDIR)
