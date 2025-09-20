#!/usr/bin/env tsx

import "dotenv/config";
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { createRegistryClient } from "../src/lib/registry.js";

/**
 * Registry Status Script
 * Checks the status of the Coral Registry and IORA service registration
 */

async function main() {
  console.log("📊 Checking IORA MCP Registry Status\n");

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

    console.log("🔗 Registry Configuration:");
    console.log("   Registry URL:", process.env.CORAL_REGISTRY_URL);
    console.log("   Auth Token:", process.env.CORAL_REGISTRY_TOKEN ? "✓ Configured" : "✗ Not configured");
    console.log("   Service: iora-mcp v1.0.0");
    console.log("");

    // Check registry status
    console.log("🌐 Checking registry connectivity...");
    const statusResult = await registryClient.getStatus();

    if (statusResult.online) {
      console.log("✅ Registry is online");
      console.log("   Version:", statusResult.version || "unknown");
      console.log("   Total Services:", statusResult.services || "unknown");
      console.log("");
    } else {
      console.log("❌ Registry is offline or unreachable");
      console.log("   Error:", statusResult.error);
      console.log("");
      console.log("💡 Make sure the Coral Registry is running and accessible");
      process.exit(1);
    }

    // Check service registration
    console.log("🔍 Checking IORA service registration...");
    const registrationResult = await registryClient.isRegistered();

    if (registrationResult.registered) {
      console.log("✅ IORA service is registered");
      console.log("   Service ID:", registrationResult.serviceId);
      console.log("   Status: Discoverable by compatible agents");
    } else {
      console.log("❌ IORA service is not registered");
      if (registrationResult.error) {
        console.log("   Error:", registrationResult.error);
      }
      console.log("   Status: Not discoverable by agents");
      console.log("");
      console.log("💡 Run 'npm run registry:register' to register the service");
    }

    console.log("");
    console.log("📋 Summary:");
    console.log("   Registry:", statusResult.online ? "🟢 Online" : "🔴 Offline");
    console.log("   IORA Service:", registrationResult.registered ? "🟢 Registered" : "🔴 Not Registered");

    if (statusResult.online && registrationResult.registered) {
      console.log("\n🎉 IORA MCP is fully operational and discoverable!");
      console.log("   Compatible agents can connect to this service through the Coral Registry.");
    }

  } catch (error) {
    console.error("💥 Registry status check failed:");
    console.error("   Error:", error instanceof Error ? error.message : String(error));
    process.exit(1);
  }
}

// Handle graceful shutdown
process.on("SIGINT", () => {
  console.log("\n👋 Registry status check cancelled by user");
  process.exit(0);
});

process.on("SIGTERM", () => {
  console.log("\n👋 Registry status check terminated");
  process.exit(0);
});

// Run the script
main().catch((error) => {
  console.error("💥 Unexpected error:", error);
  process.exit(1);
});
