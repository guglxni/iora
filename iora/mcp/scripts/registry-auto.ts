#!/usr/bin/env tsx

import "dotenv/config";
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { createRegistryClient } from "../src/lib/registry.js";

/**
 * Registry Auto-Management Script
 * Automatically manages IORA MCP service registration and heartbeat
 * This script can run continuously to keep the service registered and updated
 */

async function main() {
  console.log("🤖 Starting IORA MCP Registry Auto-Management\n");

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

    console.log("📋 Auto-Management Configuration:");
    console.log("   Service: iora-mcp v1.0.0");
    console.log("   Registry URL:", process.env.CORAL_REGISTRY_URL);
    console.log("   Auth Token:", process.env.CORAL_REGISTRY_TOKEN ? "✓ Configured" : "✗ Not configured");
    console.log("   Auto-Register:", process.env.CORAL_REGISTRY_AUTO_REGISTER);
    console.log("   Heartbeat Interval:", process.env.CORAL_REGISTRY_HEARTBEAT_INTERVAL || "60", "seconds");
    console.log("");

    // Check current registration status
    console.log("🔍 Checking current registration status...");
    const checkResult = await registryClient.isRegistered();

    if (checkResult.registered) {
      console.log("✅ Service is already registered (ID:", checkResult.serviceId + ")");
      console.log("   Updating metadata...");
      await registryClient.update();
    } else {
      console.log("❌ Service is not registered");
      console.log("   Registering service...");

      const registerResult = await registryClient.register();
      if (!registerResult.success) {
        console.error("❌ Registration failed:", registerResult.error);
        process.exit(1);
      }
      console.log("✅ Service registered successfully!");
    }

    // Start heartbeat mechanism
    console.log("💓 Starting registry heartbeat...");
    registryClient.startHeartbeat();

    console.log("\n🎉 IORA MCP Registry Auto-Management Active!");
    console.log("   Service is registered and heartbeat is running");
    console.log("   Press Ctrl+C to stop");

    // Keep the process alive
    await new Promise(() => {}); // Wait indefinitely

  } catch (error) {
    console.error("💥 Registry auto-management script failed:");
    console.error("   Error:", error instanceof Error ? error.message : String(error));
    process.exit(1);
  }
}

// Handle graceful shutdown
process.on("SIGINT", () => {
  console.log("\n👋 Registry auto-management stopped by user");
  process.exit(0);
});

process.on("SIGTERM", () => {
  console.log("\n👋 Registry auto-management terminated");
  process.exit(0);
});

// Run the script
main().catch((error) => {
  console.error("💥 Unexpected error:", error);
  process.exit(1);
});
