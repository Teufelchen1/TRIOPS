
set -e

export XLEN=32
export RISCV_GCC_OPTS="-I$(pwd)/include -I$(pwd)/src/env -march=rv32im_zifencei -mabi=ilp32 -static -mcmodel=medany -fvisibility=hidden -nostdlib -nostartfiles"

banner() {
	printf "##\n# %s\n##\n\n" "${1}"
}

banner "Build tests"
make -C src/isa XLEN=${XLEN} rv32ui rv32um

banner "Run tests"

exit=0
for file in src/isa/rv32ui-p-* src/isa/rv32um-p-*; do
	[ -f "${file}" -a -x "${file}" ] || continue

	name=${file##*/}
	printf "Running test case '%s':\t" "$name"

	if [ "${name}" = rv32ui-p-fence_i ]; then
		printf "SKIP\n"
		continue
	fi

	# riscv64-unknown-elf-objcopy --strip-debug -O binary ${file} ./test.hex

	ret=0; ../target/debug/rv --file ${file} --headless --testing &> /dev/null || ret=$?
	if [ "${ret}" -ne 0 ]; then
		exit=1
		printf "FAIL ❌\n"
		continue
	fi

	printf "OKAY ✅\n"
done

exit "${exit}"