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

.PHONY: all clean
