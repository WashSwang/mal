STEPS = step0_repl step1_read_print step2_eval step3_env step4_if_fn_do

all: $(STEPS)

dist: mal

mal: step0_repl
	cp $< $@

%: src/%.rs
	cargo build --release --bin $*
	cp target/release/$* $@

STEP0_DEPS = Cargo.toml
STEP1_DEPS = $(STEP0_DEPS) src/types.rs src/reader.rs src/printer.rs
STEP2_DEPS = $(STEP1_DEPS) src/env.rs
STEP4_DEPS = $(STEP2_DEPS) src/core.rs

step0_repl: $(STEP0_DEPS)
step1_read_print: $(STEP1_DEPS)
step2_eval: $(STEP2_DEPS)
step3_env: $(STEP2_DEPS)
step4_if_fn_do: $(STEP4_DEPS)

.PHONY: clean

clean:
	cargo clean
	rm -f $(STEPS)
	rm -f mal