EXE = rce

rule:
	cargo rustc --release -- -C target-cpu=native --emit link=$(EXE)