AS := as-2-trunk as-2-42 as-2-41
LD := ld-2-trunk ld-2-42 ld-2-41
CC1 := cc1

# rv64
rv64: stamps/binutils stamps/cc1
	eval "$(foreach X,$(CC1), $(foreach Y,$(AS),$(foreach Z,$(LD),echo $X-$Y-$Z && rm -rf $X-$Y-$Z && mkdir $X-$Y-$Z && cp -r $X/* $Y/* $Z/* $X-$Y-$Z &&))) true"
	cargo build --bins --release
	cp ./target/release/cc_rand .
	cp ./target/release/cxx_rand .


stamps/binutils:
	rm -rf binutils-2-trunk binutils-2-42 binutils-2-41
	rm -rf as-2-trunk as-2-42 as-2-41 ld-2-trunk ld-2-42 ld-2-41

	cd riscv-gnu-toolchain && git submodule update --init binutils
	cd riscv-gnu-toolchain/binutils && git checkout master
	mkdir binutils-2-trunk
	cd binutils-2-trunk && ../riscv-gnu-toolchain/configure --prefix=$(shell pwd)/binutils-2-trunk --with-multilib-generator="rv64gcv-lp64d--"
	cd binutils-2-trunk && $(MAKE) stamps/build-binutils-linux -j32

	cd riscv-gnu-toolchain/binutils && git checkout c7f28aad0c99d1d2fec4e52ebfa3735d90ceb8e9
	mkdir binutils-2-42
	cd binutils-2-42 && ../riscv-gnu-toolchain/configure --prefix=$(shell pwd)/binutils-2-42 --with-multilib-generator="rv64gcv-lp64d--"
	cd binutils-2-42 && $(MAKE) stamps/build-binutils-linux -j32

	cd riscv-gnu-toolchain/binutils && git checkout 675b9d612cc59446e84e2c6d89b45500cb603a8d
	mkdir binutils-2-41
	cd binutils-2-41 && ../riscv-gnu-toolchain/configure --prefix=$(shell pwd)/binutils-2-41 --with-multilib-generator="rv64gcv-lp64d--"
	cd binutils-2-41 && $(MAKE) stamps/build-binutils-linux -j32

	mkdir -p as-2-trunk as-2-42 as-2-41 ld-2-trunk ld-2-42 ld-2-41

	cp ./binutils-2-trunk/riscv64-unknown-linux-gnu/bin/as as-2-trunk/as
	cp ./binutils-2-42/riscv64-unknown-linux-gnu/bin/as as-2-42/as
	cp ./binutils-2-41/riscv64-unknown-linux-gnu/bin/as as-2-41/as
	cp ./binutils-2-trunk/riscv64-unknown-linux-gnu/bin/ld ld-2-trunk/ld
	cp ./binutils-2-42/riscv64-unknown-linux-gnu/bin/ld ld-2-42/ld
	cp ./binutils-2-41/riscv64-unknown-linux-gnu/bin/ld ld-2-41/ld

	mkdir -p stamps
	touch stamps/binutils

stamps/cc1:
	rm -rf cc1-trunk
	cd riscv-gnu-toolchain && git submodule update --init gcc
	cd riscv-gnu-toolchain/gcc && git checkout master

	mkdir cc1-trunk
	cd cc1-trunk && ../riscv-gnu-toolchain/configure --prefix=$(shell pwd)/cc1-trunk --with-multilib-generator="rv64gcv-lp64d--"
	cd cc1-trunk && $(MAKE) linux -j32

	mkdir -p cc1
	cp -r ./cc1-trunk/libexec/gcc/riscv64-unknown-linux-gnu/14.0.1/* cc1

	mkdir -p stamps
	touch stamps/cc1
