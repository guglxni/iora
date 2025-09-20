import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import {
  RegistrySettings,
  loadRegistrySettings,
  validateRegistrySettings,
  getRegistryAuthHeader,
  DEFAULT_SERVICE_METADATA,
  ServiceMetadata
} from "./registry-config.js";

// Registry publishing client
export class CoralRegistryClient {
  private settings: RegistrySettings;
  private metadata: ServiceMetadata;
  private server: Server;

  constructor(server: Server, customSettings?: Partial<RegistrySettings>, customMetadata?: Partial<ServiceMetadata>) {
    this.settings = { ...loadRegistrySettings(), ...customSettings };
    this.metadata = { ...DEFAULT_SERVICE_METADATA, ...customMetadata };
    this.server = server;

    // Validate settings on initialization
    const validation = validateRegistrySettings(this.settings);
    if (!validation.valid) {
      throw new Error(`Invalid registry settings: ${validation.errors.join(", ")}`);
    }
  }

  /**
   * Register service with local Coral Registry
   */
  async register(): Promise<{ success: boolean; serviceId?: string; error?: string }> {
    try {
      // Create registration payload with full service metadata
      const payload = {
        ...this.metadata,
        transport: "http",
        baseUrl: `http://localhost:${process.env.PORT || 7070}`,
        runtime: {
          nodeVersion: process.version,
          platform: process.platform,
          arch: process.arch,
          uptime: process.uptime(),
          memoryUsage: process.memoryUsage(),
        },
        registrationTime: new Date().toISOString(),
      };

      const authHeader = getRegistryAuthHeader(this.settings);
      const headers: Record<string, string> = {
        "Content-Type": "application/json",
        ...authHeader
      };

      console.log(`🔍 Registering service with Coral Registry at ${this.settings.url}`);

      const response = await fetch(`${this.settings.url}/api/services/register`, {
        method: "POST",
        headers,
        body: JSON.stringify(payload),
        signal: AbortSignal.timeout(this.settings.timeout)
      });

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`Registry registration failed: ${response.status} ${response.statusText} - ${errorText}`);
      }

      const result = await response.json();
      console.log(`✅ Service registered successfully with ID: ${result.serviceId}`);
      return { success: true, serviceId: result.serviceId };
    } catch (error) {
      const errorMsg = error instanceof Error ? error.message : "Unknown error";
      console.error(`❌ Registry registration failed: ${errorMsg}`);
      return { success: false, error: errorMsg };
    }
  }

  /**
   * Unregister service from registry
   */
  async unregister(serviceId?: string): Promise<{ success: boolean; error?: string }> {
    try {
      const payload = {
        serviceId: serviceId || `${this.metadata.name}@${this.metadata.version}`
      };

      const authHeader = getRegistryAuthHeader(this.settings);
      const headers: Record<string, string> = {
        "Content-Type": "application/json",
        ...authHeader
      };

      console.log(`🔍 Unregistering service ${payload.serviceId} from Coral Registry`);

      const response = await fetch(`${this.settings.url}/api/services/unregister`, {
        method: "POST",
        headers,
        body: JSON.stringify(payload),
        signal: AbortSignal.timeout(this.settings.timeout)
      });

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`Registry unregistration failed: ${response.status} ${response.statusText} - ${errorText}`);
      }

      console.log(`✅ Service unregistered successfully`);
      return { success: true };
    } catch (error) {
      const errorMsg = error instanceof Error ? error.message : "Unknown error";
      console.error(`❌ Registry unregistration failed: ${errorMsg}`);
      return { success: false, error: errorMsg };
    }
  }

  /**
   * Update service metadata in registry
   */
  async update(): Promise<{ success: boolean; error?: string }> {
    try {
      const payload = {
        ...this.metadata,
        runtime: {
          nodeVersion: process.version,
          platform: process.platform,
          arch: process.arch,
          uptime: process.uptime(),
          memoryUsage: process.memoryUsage(),
        },
        lastUpdated: new Date().toISOString(),
      };

      const authHeader = getRegistryAuthHeader(this.settings);
      const headers: Record<string, string> = {
        "Content-Type": "application/json",
        ...authHeader
      };

      console.log(`🔄 Updating service metadata in Coral Registry`);

      const response = await fetch(`${this.settings.url}/api/services/update`, {
        method: "PUT",
        headers,
        body: JSON.stringify(payload),
        signal: AbortSignal.timeout(this.settings.timeout)
      });

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`Registry update failed: ${response.status} ${response.statusText} - ${errorText}`);
      }

      console.log(`✅ Service metadata updated successfully`);
      return { success: true };
    } catch (error) {
      const errorMsg = error instanceof Error ? error.message : "Unknown error";
      console.error(`❌ Registry update failed: ${errorMsg}`);
      return { success: false, error: errorMsg };
    }
  }

  /**
   * Check if service is registered
   */
  async isRegistered(): Promise<{ registered: boolean; serviceId?: string; error?: string }> {
    try {
      const payload = {
        name: this.metadata.name,
        version: this.metadata.version
      };

      const authHeader = getRegistryAuthHeader(this.settings);
      const headers: Record<string, string> = {
        "Content-Type": "application/json",
        ...authHeader
      };

      const response = await fetch(`${this.settings.url}/api/services/check`, {
        method: "POST",
        headers,
        body: JSON.stringify(payload),
        signal: AbortSignal.timeout(this.settings.timeout)
      });

      if (!response.ok) {
        if (response.status === 404) {
          return { registered: false };
        }
        const errorText = await response.text();
        throw new Error(`Registry check failed: ${response.status} ${response.statusText} - ${errorText}`);
      }

      const result = await response.json();
      return { registered: true, serviceId: result.serviceId };
    } catch (error) {
      return { registered: false, error: error instanceof Error ? error.message : "Unknown error" };
    }
  }

  /**
   * Get registry status
   */
  async getStatus(): Promise<{ online: boolean; version?: string; services?: number; error?: string }> {
    try {
      const authHeader = getRegistryAuthHeader(this.settings);
      const headers: Record<string, string> = {
        ...authHeader
      };

      const response = await fetch(`${this.settings.url}/api/status`, {
        method: "GET",
        headers,
        signal: AbortSignal.timeout(this.settings.timeout)
      });

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`Registry status check failed: ${response.status} ${response.statusText} - ${errorText}`);
      }

      const status = await response.json();
      return {
        online: true,
        version: status.version,
        services: status.totalServices
      };
    } catch (error) {
      return {
        online: false,
        error: error instanceof Error ? error.message : "Unknown error"
      };
    }
  }

  /**
   * Start heartbeat/keepalive mechanism
   */
  startHeartbeat(): void {
    if (this.settings.heartbeatInterval > 0) {
      console.log(`💓 Starting registry heartbeat every ${this.settings.heartbeatInterval}s`);

      setInterval(async () => {
        try {
          await this.update();
        } catch (error) {
          console.error(`❌ Heartbeat update failed: ${error instanceof Error ? error.message : 'Unknown error'}`);
        }
      }, this.settings.heartbeatInterval * 1000);
    }
  }
}

// Factory function to create registry client
export function createRegistryClient(
  server: Server,
  customSettings?: Partial<import("./registry-config.js").RegistrySettings>,
  customMetadata?: Partial<import("./registry-config.js").ServiceMetadata>
): CoralRegistryClient {
  return new CoralRegistryClient(server, customSettings, customMetadata);
}
