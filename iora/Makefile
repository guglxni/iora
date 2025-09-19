SHELL := /bin/bash
export IORA_BIN ?= ./target/release/iora
export PORT ?= 7070

.PHONY: build run demo clean

build:
	cargo build --release
	cd mcp && npm i && npm run build

run:
	cd mcp && IORA_BIN=$(IORA_BIN) npm run dev

demo:
	@[ -n "$$CORAL_SHARED_SECRET" ] || (echo "Set CORAL_SHARED_SECRET"; exit 1)
	@echo "=== IORA MCP Demo ==="
	@echo
	@echo "1. Health Check:"
	@curl -s http://localhost:$(PORT)/tools/health | jq '.'
	@echo
	@echo "2. Get Price (BTC):"
	@b='{"symbol":"BTC"}'; s=$$(echo -n $$b | openssl dgst -sha256 -hmac "$$CORAL_SHARED_SECRET" | awk '{print $$2}'); \
	 curl -s -H "x-iora-signature: $$s" -H "content-type: application/json" \
	 -d "$$b" http://localhost:$(PORT)/tools/get_price | jq '.'
	@echo
	@echo "3. Analyze Market (BTC, Mistral):"
	@b='{"symbol":"BTC","horizon":"1d","provider":"mistral"}'; s=$$(echo -n $$b | openssl dgst -sha256 -hmac "$$CORAL_SHARED_SECRET" | awk '{print $$2}'); \
	 curl -s -H "x-iora-signature: $$s" -H "content-type: application/json" \
	 -d "$$b" http://localhost:$(PORT)/tools/analyze_market | jq '.'
	@echo
	@echo "4. Feed Oracle (BTC):"
	@b='{"symbol":"BTC"}'; s=$$(echo -n $$b | openssl dgst -sha256 -hmac "$$CORAL_SHARED_SECRET" | awk '{print $$2}'); \
	 curl -s -H "x-iora-signature: $$s" -H "content-type: application/json" \
	 -d "$$b" http://localhost:$(PORT)/tools/feed_oracle | jq '.'

clean:
	cargo clean
	rm -rf mcp/node_modules mcp/dist