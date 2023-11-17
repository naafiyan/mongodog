PROJECTS = social_server social_client mongowner mongowner_macros

all: $(PROJECTS)

social_server:
	cd social-rs/server && cargo build

run_server:
	cd social-rs/server && cargo run

mongowner:
	cd mongowner && cargo build

mongowner_macros:
	cd mongowner && cargo build

social_client:
	cd social-rs/frontend

clean:
	cd social-rs/server && cargo clean
	cd mongowner/mongowner-macros && cargo clean
	cd mongowner && cargo clean

.PHONY: all clean $(PROJECTS)
