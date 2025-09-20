#!/usr/bin/env tsx

import "dotenv/config";
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { createRegistryClient } from "../src/lib/registry.js";

/**
 * Registry Check Script
 * Checks if IORA MCP service is registered with the Coral Registry
 */

async function main() {
  console.log("🔍 Checking IORA MCP Service Registration\n");

  try {
    // Validate environment
    if (!process.env.CORAL_REGISTRY_URL) {
      throw new Error("CORAL_REGISTRY_URL environment variable is required");
    }

    // Create a dummy server for registry client (not actually started)
    const server = new Server(
      { name: "iora-mcp", version: "1.0.0" },
      { capabilities: {} }
    );

    // Create registry client
    const registryClient = createRegistryClient(server);

    console.log("📋 Service Information:");
    console.log("   Name: iora-mcp");
    console.log("   Version: 1.0.0");
    console.log("   Registry URL:", process.env.CORAL_REGISTRY_URL);
    console.log("");

    // Check service registration
    console.log("🔍 Checking service registration status...");
    const result = await registryClient.isRegistered();

    if (result.registered) {
      console.log("✅ IORA MCP service is registered!");
      console.log("   Service ID:", result.serviceId);
      console.log("   Status: Active and discoverable");
      console.log("");
      console.log("🎯 Service Details:");
      console.log("   - Compatible agents can discover this service");
      console.log("   - 4 MCP tools available (get_price, analyze_market, feed_oracle, health)");
      console.log("   - HTTP transport on port", process.env.PORT || 7070);
      console.log("   - Supports HMAC authentication");
      console.log("");
      console.log("💡 To unregister: npm run registry:unregister");
      console.log("💡 To update metadata: npm run registry:update");
    } else {
      console.log("❌ IORA MCP service is not registered");
      if (result.error) {
        console.log("   Error:", result.error);
      }
      console.log("");
      console.log("💡 To register: npm run registry:register");
      console.log("💡 Check registry status: npm run registry:status");
      process.exit(1);
    }

  } catch (error) {
    console.error("💥 Registry check script failed:");
    console.error("   Error:", error instanceof Error ? error.message : String(error));
    process.exit(1);
  }
}

// Handle graceful shutdown
process.on("SIGINT", () => {
  console.log("\n👋 Registry check cancelled by user");
  process.exit(0);
});

process.on("SIGTERM", () => {
  console.log("\n👋 Registry check terminated");
  process.exit(0);
});

// Run the script
main().catch((error) => {
  console.error("💥 Unexpected error:", error);
  process.exit(1);
});
