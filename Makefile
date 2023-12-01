PROJECTS = social_server social_client mongowner mongowner_macros mongowner_test

all: $(PROJECTS)

social_server:
	cd examples/social-rs/server && cargo build

run_server:
	cd examples/social-rs/server && cargo run

mongowner:
	cd mongowner && cargo build

mongowner_macros:
	cd mongowner && cargo build

mongowner_test:
	cd mongowner && cargo test

social_client:
	cd examples/social-rs/frontend && bun dev

clean:
	cd social-rs/server && cargo clean
	cd mongowner/mongowner-macros && cargo clean
	cd mongowner && cargo clean

social_db:
	cd examples/social-rs && mongod --dbpath=db


.PHONY: all clean $(PROJECTS)
