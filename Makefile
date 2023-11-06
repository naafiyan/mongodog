PROJECTS = social_server social_client mongowner mongowner_macros

all: $(PROJECTS)

social_server:
	cd social-rs/server && cargo build

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
	cd mongowner/src && rm -f delete_original.rs
	cd mongowner/src && rm -f delete.rs
	cp mongowner/delete.rs mongowner/src

.PHONY: all clean
