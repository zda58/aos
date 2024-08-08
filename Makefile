.PHONY = amd64 arm64 clean

amd64:
arm64:
	cargo build --target build-targets/aarch64-aos.json
make arm64l:
	qemu-system-aarch64 -M virt -cpu max -kernel 
clean:
	cargo clean