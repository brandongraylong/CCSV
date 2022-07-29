# Build
all: setup clean build copy
setup:
	mkdir -p includes
	mkdir -p lib
clean:
	rm -f target/release/libccsv.a
build:
	cargo build --release
copy: target/release/libccsv.a
	cp target/release/libccsv.a ./lib/

# Build and run C
example_build_c:
	mkdir -p bin/examples/C
	gcc -Iincludes/ examples/C/main.c lib/libccsv.a -o bin/examples/C/main.out
example_run_c:
	./bin/examples/C/main.out
example_valgrind_c:
	valgrind ./bin/examples/C/main.out

# Build, run, and test C++
example_build_c++:
	mkdir -p bin/examples/C++
	gcc -Iincludes/ examples/C++/main.cc -lstdc++ lib/libccsv.a -o bin/examples/C++/main.out
example_run_c++:
	./bin/examples/C++/main.out
example_valgrind_c++:
	valgrind ./bin/examples/C++/main.out
