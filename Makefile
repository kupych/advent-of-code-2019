create:
		@cargo new day$(day) --bin --vcs none && \
		mkdir ./day$(day)/input && \
		touch ./day$(day)/input/input.txt && \
		rm ./day$(day)/src/main.rs && \
		cp ./.template ./day$(day)/src/main.rs
