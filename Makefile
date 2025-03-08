build:
	cd dysprosium-uci && cargo b -r
	cp target/release/dysprosium-uci $(EXE)
