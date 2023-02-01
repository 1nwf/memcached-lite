run: 
	cargo run


test:
	cargo test -p client -- --nocapture --test-threads=1
