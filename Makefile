gtk3:
	cargo run --bin builder --features=gtk3

gtk4:
	cargo run --bin builder --features=gtk4

query3:
	cargo run --bin query --features=gtk3

query4:
	cargo run --bin query --features=gtk4

set3:
	cargo run --bin set --features=gtk3

set4:
	cargo run --bin set --features=gtk4

query-no-display:
	cargo run --bin query --features=gtk4,no_display
