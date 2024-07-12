.PHONY = amd64 arm64 clean

amd64:
arm64:
	cargo build --target build-targets/aarch64-aos.json
clean:
	cargo clean