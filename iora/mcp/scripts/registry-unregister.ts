#!/usr/bin/env tsx

import "dotenv/config";
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { createRegistryClient } from "../src/lib/registry.js";

/**
 * Registry Unregistration Script
 * Removes the IORA MCP service from a local Coral Registry
 */

async function main() {
  console.log("🗑️ Starting IORA MCP Registry Unregistration\n");

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

    // Check if service is registered first
    console.log("🔍 Checking current registration status...");
    const checkResult = await registryClient.isRegistered();

    if (!checkResult.registered) {
      console.log("ℹ️ Service is not currently registered with the registry");
      console.log("   No unregistration needed.");
      return;
    }

    console.log("   Service is registered (ID:", checkResult.serviceId + ")");

    // Unregister service
    console.log("🔄 Unregistering service from Coral Registry...");
    const result = await registryClient.unregister(checkResult.serviceId);

    if (result.success) {
      console.log("✅ Service unregistration successful!");
      console.log("   Service is no longer discoverable in the Coral Registry.");
    } else {
      console.error("❌ Service unregistration failed:");
      console.error("   Error:", result.error);
      process.exit(1);
    }

  } catch (error) {
    console.error("💥 Registry unregistration script failed:");
    console.error("   Error:", error instanceof Error ? error.message : String(error));
    process.exit(1);
  }
}

// Handle graceful shutdown
process.on("SIGINT", () => {
  console.log("\n👋 Registry unregistration cancelled by user");
  process.exit(0);
});

process.on("SIGTERM", () => {
  console.log("\n👋 Registry unregistration terminated");
  process.exit(0);
});

// Run the script
main().catch((error) => {
  console.error("💥 Unexpected error:", error);
  process.exit(1);
});
