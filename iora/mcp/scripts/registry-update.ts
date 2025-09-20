#!/usr/bin/env tsx

import "dotenv/config";
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { createRegistryClient } from "../src/lib/registry.js";

/**
 * Registry Update Script
 * Updates IORA MCP service metadata in the Coral Registry
 */

async function main() {
  console.log("🔄 Updating IORA MCP Registry Metadata\n");

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

    // Check if service is registered first
    console.log("🔍 Checking current registration status...");
    const checkResult = await registryClient.isRegistered();

    if (!checkResult.registered) {
      console.log("❌ Service is not currently registered with the registry");
      console.log("   Please register the service first using: npm run registry:register");
      process.exit(1);
    }

    console.log("   Service is registered (ID:", checkResult.serviceId + ")");

    // Update service metadata
    console.log("🔄 Updating service metadata in Coral Registry...");
    const result = await registryClient.update();

    if (result.success) {
      console.log("✅ Service metadata updated successfully!");
      console.log("   Updated information includes:");
      console.log("   - Runtime statistics (uptime, memory usage)");
      console.log("   - Current timestamp");
      console.log("   - Latest service capabilities");
      console.log("");
      console.log("🔄 Registry metadata is now current");
    } else {
      console.error("❌ Service metadata update failed:");
      console.error("   Error:", result.error);
      process.exit(1);
    }

  } catch (error) {
    console.error("💥 Registry update script failed:");
    console.error("   Error:", error instanceof Error ? error.message : String(error));
    process.exit(1);
  }
}

// Handle graceful shutdown
process.on("SIGINT", () => {
  console.log("\n👋 Registry update cancelled by user");
  process.exit(0);
});

process.on("SIGTERM", () => {
  console.log("\n👋 Registry update terminated");
  process.exit(0);
});

// Run the script
main().catch((error) => {
  console.error("💥 Unexpected error:", error);
  process.exit(1);
});
