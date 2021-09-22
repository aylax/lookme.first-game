.PHONY: clean run

fmt:
	@cargo fmt
run:
	@cargo run --features bevy/dynamic

clean:
	@cargo clean
