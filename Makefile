build:
	git submodule update --init
	cd dysprosium-uci && cargo b -r
	cp target/release/dysprosium-uci $(EXE)
