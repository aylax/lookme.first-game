.PHONY: clean run

run:
	@cargo run --features bevy/dynamic

clean:
	@cargo clean
