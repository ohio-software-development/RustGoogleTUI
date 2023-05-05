.PHONY: all build implement


all: build implement
build: 
	cargo build --manifest-path=RustTUIOUSDC/Cargo.toml
	
implement:
	@echo "export PATH="$$"PATH:${HOME}/RustGoogleTUI/RustTUIOUSDC/target/debug/" >> ../.bashrc
	



# othello_main: main.o game.o othello.o
# 	$(CXX) $(LDFLAGS) -o $@ $^

# main.o: main.cpp game.hpp othello.hpp
# 	$(CXX) $(CXXFLAGS) -c -I../vendors -o $@ $<

# doc: # YOUR ANSWER HERE: add correct dependencies
# 	@echo "entering doc target"
# 	@doxygen Doxyfile
# # YOUR ANSWER HERE: write a command generate documentation using doxygen.
# # HINT: the command is NOT "doxygen -g doxygen.conf"
# # NOTE: Don't forget to indent your command properly.

# clean:
# 	@echo "entering clean target"
# 	-rm -f *.o
# 	-rm othello_main
