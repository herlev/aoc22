.PHONY = run test init testo wrun push

day != date +%d

run:
	cargo run --bin $(day)

test:
	cargo test --bin $(day)

wrun:
	cargo watch -qcs "make --silent run"

# Run tests showing output
testo:
	cargo test --bin $(day) -- --nocapture

# Download puzzle
puzzles/$(day).md:
	aoc read --overwrite -p puzzles/$(day).md
	
# Download puzzle input
inputs/$(day).txt:
	aoc download --overwrite -i inputs/$(day).txt

# Copy template file
src/bin/$(day).rs:
	./gen-day.sh $(day)

init: puzzles/$(day).md inputs/$(day).txt src/bin/$(day).rs
	env PAGER="less -r" glow -p puzzles/$(day).md
	
push:
	git add src/bin/$(day).rs
	git commit -m "add day $(day)"
	git push
