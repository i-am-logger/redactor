.PHONY: help present build build-docker run run-docker get-options-docker update-options-docker redact-docker check-env check-nix

# ANSI color codes for pretty printing
C_CYAN   = \033[96m
C_BLUE   = \033[94m
C_TEXT   = \033[0m
C_WARN   = \033[33m
C_BOLD   = \033[1m
C_RESET  = \033[0m

.DEFAULT_GOAL := help

present: check-nix ## open the presentation
	xdg-open presentation/slides.html

# Most commonly used commands first
run: check-nix ## Run the project using nix
	@printf "$(C_CYAN)Running project...$(C_RESET)\n"
	nix run

build: check-nix ## Build the project using Nix
	@printf "$(C_CYAN)Building project with Nix...$(C_RESET)\n"
	nix build

# Docker variants
run-docker: check-nix ## Run the Docker container
	@printf "$(C_CYAN)Running Docker container...$(C_RESET)\n"
	# nix run .#docker

	@printf "$(C_WARN)nix run .#docker has a bug with passing args $(C_RESET)\n"
	scripts/run-docker.sh redactor 0.1.0 8080 $(CACHE_DIR)
	
build-docker: check-nix ## Build Docker image using Nix
	@printf "$(C_CYAN)Building Docker image with Nix...$(C_RESET)\n"
	nix build .#docker

get-options-docker: ## call GET /api/options
	curl -iX GET http://localhost:8080/api/options
put-options-docker: ## call PUT /api/options to update with ner_options.json
	scripts/update_redact_options.sh http://localhost:8080/api/options ner_options.json
redact-docker: ## call /api/redact_text on the docker instance 
	scripts/redact.sh http://localhost:8080/api/redact_text stream.txt

# Development environment checks
check-env: ## Check for .env file and display environment variables
	@if [ ! -f .env ]; then \
		echo "$(C_WARN)Warning: .env file not found. Create one for development environment variables.$(C_RESET)"; \
	else \
		printf "$(C_CYAN)Environment variables loaded from .env:$(C_RESET)\n\n"; \
		printf "$(C_CYAN)$(C_BOLD)NAME$(C_RESET)                                    $(C_CYAN)$(C_BOLD)VALUE$(C_RESET)\n"; \
		printf "$(C_BLUE)───────────────────────────────────────────────────────$(C_RESET)\n"; \
		max_length=0; \
		while IFS='=' read -r key value || [ -n "$$key" ]; do \
			if [ -n "$$key" ] && [ "$${key:0:1}" != "#" ]; then \
				key_length=$${#key}; \
				[ $$key_length -gt $$max_length ] && max_length=$$key_length; \
			fi; \
		done < .env; \
		while IFS='=' read -r key value || [ -n "$$key" ]; do \
			if [ -n "$$key" ] && [ "$${key:0:1}" != "#" ]; then \
				printf "$(C_CYAN)$(C_BOLD)%-$${max_length}s$(C_RESET) = %s\n" "$$key" "$$value"; \
			fi; \
		done < .env; \
	fi

check-nix: ## Check Nix installation and development shell status
	@if ! command -v nix >/dev/null 2>&1; then \
		echo "$(C_WARN)Warning: Nix is not installed!$(C_RESET)"; \
		echo "$(C_WARN)Please install Nix using one of these methods:$(C_RESET)"; \
		echo "$(C_CYAN)• Linux/macOS: curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install$(C_RESET)"; \
		echo "$(C_CYAN)• Alternative: sh <(curl -L https://nixos.org/nix/install) --daemon$(C_RESET)"; \
		exit 1; \
	elif [ -z "$$IN_NIX_SHELL" ]; then \
		echo "$(C_WARN)Warning: Not running in a Nix development shell!$(C_RESET)"; \
		echo "$(C_WARN)Please run 'nix develop' first$(C_RESET)"; \
		exit 1; \
	fi

# Help command (listed last but shown first due to .DEFAULT_GOAL)
help: ## Display this help message
	@printf "$(C_CYAN)$(C_BOLD)Usage:$(C_RESET)\n"
	@printf "  $(C_CYAN)make$(C_RESET) $(C_CYAN)<target>$(C_RESET)\n"
	@printf "\n"
	@printf "$(C_CYAN)$(C_BOLD)Targets:$(C_RESET)\n"
	@awk 'BEGIN {FS = ":.*?## "} \
		/^[a-zA-Z_-]+:.*?##/ { printf "  $(C_CYAN)%-15s$(C_TEXT)  %s$(C_RESET)\n", $$1, $$2 }' \
		$(MAKEFILE_LIST)
