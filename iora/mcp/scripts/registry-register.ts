#!/usr/bin/env tsx

import "dotenv/config";
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { createRegistryClient } from "../src/lib/registry.js";

/**
 * Registry Registration Script
 * Registers the IORA MCP service with a local Coral Registry
 */

async function main() {
  console.log("🚀 Starting IORA MCP Registry Registration\n");

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
    console.log("   Auth Token:", process.env.CORAL_REGISTRY_TOKEN ? "✓ Configured" : "✗ Not configured");
    console.log("");

    // Register service
    console.log("🔄 Registering service with Coral Registry...");
    const result = await registryClient.register();

    if (result.success) {
      console.log("✅ Service registration successful!");
      console.log("   Service ID:", result.serviceId);
      console.log("");

      // Start heartbeat if enabled
      if (process.env.CORAL_REGISTRY_AUTO_REGISTER === "true") {
        console.log("💓 Starting registry heartbeat...");
        registryClient.startHeartbeat();
        console.log("   Heartbeat enabled - service will auto-update every",
                   process.env.CORAL_REGISTRY_HEARTBEAT_INTERVAL || "60", "seconds");
      }

      console.log("\n🎉 IORA MCP is now discoverable in the Coral Registry!");
      console.log("   Compatible agents can now find and connect to this service.");
    } else {
      console.error("❌ Service registration failed:");
      console.error("   Error:", result.error);
      process.exit(1);
    }

  } catch (error) {
    console.error("💥 Registry registration script failed:");
    console.error("   Error:", error instanceof Error ? error.message : String(error));
    process.exit(1);
  }
}

// Handle graceful shutdown
process.on("SIGINT", () => {
  console.log("\n👋 Registry registration cancelled by user");
  process.exit(0);
});

process.on("SIGTERM", () => {
  console.log("\n👋 Registry registration terminated");
  process.exit(0);
});

// Run the script
main().catch((error) => {
  console.error("💥 Unexpected error:", error);
  process.exit(1);
});
