PROJECTDIR = ../push_swap

all:
	cargo build
	cp ./target/debug/spst $(PROJECTDIR)
