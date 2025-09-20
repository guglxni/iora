#!/usr/bin/env node

/**
 * Mock Coral Registry Server for Testing IORA Registry Readiness
 *
 * This is a simple Express.js server that mimics the Coral Registry API
 * to test IORA's registry integration functionality.
 */

import express from 'express';
import crypto from 'crypto';

const app = express();
app.use(express.json());

// In-memory storage for registered services
const registeredServices = new Map();

// Middleware for request logging
app.use((req, res, next) => {
  const reqId = crypto.randomUUID().substring(0, 8);
  console.log(`[${new Date().toISOString()}] ${req.method} ${req.path} (${reqId})`);

  // Log request body for POST/PUT requests
  if (['POST', 'PUT'].includes(req.method) && req.body) {
    console.log(`  Body: ${JSON.stringify(req.body, null, 2)}`);
  }

  next();
});

// Registry status endpoint
app.get('/api/status', (req, res) => {
  res.json({
    online: true,
    version: "1.0.0-mock",
    totalServices: registeredServices.size,
    uptime: process.uptime(),
    server: "iora-mock-registry"
  });
});

// Register service endpoint
app.post('/api/services/register', (req, res) => {
  try {
    const serviceData = req.body;

    if (!serviceData.name || !serviceData.version) {
      return res.status(400).json({
        success: false,
        error: "Service name and version are required"
      });
    }

    const serviceId = `${serviceData.name}@${serviceData.version}`;

    // Store the service
    registeredServices.set(serviceId, {
      ...serviceData,
      registeredAt: new Date().toISOString(),
      serviceId: serviceId,
      status: 'active'
    });

    console.log(`✅ Service registered: ${serviceId}`);

    res.json({
      success: true,
      serviceId: serviceId,
      message: `Service ${serviceId} registered successfully`
    });

  } catch (error) {
    console.error('Registration error:', error);
    res.status(500).json({
      success: false,
      error: 'Internal server error during registration'
    });
  }
});

// Unregister service endpoint
app.post('/api/services/unregister', (req, res) => {
  try {
    const { serviceId } = req.body;

    if (!serviceId) {
      return res.status(400).json({
        success: false,
        error: "Service ID is required"
      });
    }

    if (registeredServices.has(serviceId)) {
      registeredServices.delete(serviceId);
      console.log(`✅ Service unregistered: ${serviceId}`);

      res.json({
        success: true,
        message: `Service ${serviceId} unregistered successfully`
      });
    } else {
      res.status(404).json({
        success: false,
        error: `Service ${serviceId} not found`
      });
    }

  } catch (error) {
    console.error('Unregistration error:', error);
    res.status(500).json({
      success: false,
      error: 'Internal server error during unregistration'
    });
  }
});

// Update service endpoint
app.put('/api/services/update', (req, res) => {
  try {
    const serviceData = req.body;
    const serviceId = `${serviceData.name}@${serviceData.version}`;

    if (!registeredServices.has(serviceId)) {
      return res.status(404).json({
        success: false,
        error: `Service ${serviceId} not found. Register first.`
      });
    }

    // Update the service
    const existingService = registeredServices.get(serviceId);
    registeredServices.set(serviceId, {
      ...existingService,
      ...serviceData,
      updatedAt: new Date().toISOString()
    });

    console.log(`🔄 Service updated: ${serviceId}`);

    res.json({
      success: true,
      message: `Service ${serviceId} updated successfully`
    });

  } catch (error) {
    console.error('Update error:', error);
    res.status(500).json({
      success: false,
      error: 'Internal server error during update'
    });
  }
});

// Check service registration endpoint
app.post('/api/services/check', (req, res) => {
  try {
    const { name, version } = req.body;
    const serviceId = `${name}@${version}`;

    if (registeredServices.has(serviceId)) {
      const service = registeredServices.get(serviceId);
      res.json({
        success: true,
        registered: true,
        serviceId: serviceId,
        service: service
      });
    } else {
      res.json({
        success: true,
        registered: false
      });
    }

  } catch (error) {
    console.error('Check error:', error);
    res.status(500).json({
      success: false,
      error: 'Internal server error during check'
    });
  }
});

// List all services endpoint
app.get('/api/services', (req, res) => {
  try {
    const services = Array.from(registeredServices.entries()).map(([serviceId, service]) => ({
      serviceId,
      ...service
    }));

    res.json({
      success: true,
      services: services,
      total: services.length
    });

  } catch (error) {
    console.error('List error:', error);
    res.status(500).json({
      success: false,
      error: 'Internal server error during service listing'
    });
  }
});

// Health check endpoint
app.get('/health', (req, res) => {
  res.json({
    status: 'healthy',
    timestamp: new Date().toISOString(),
    services: registeredServices.size
  });
});

// Start the server
const PORT = process.env.CORAL_REGISTRY_PORT || 8080;

app.listen(PORT, () => {
  console.log(`🐚 Mock Coral Registry Server started on port ${PORT}`);
  console.log(`📊 Status: http://localhost:${PORT}/api/status`);
  console.log(`📋 Services: http://localhost:${PORT}/api/services`);
  console.log(`❤️ Health: http://localhost:${PORT}/health`);
  console.log('');
  console.log('🧪 Ready to test IORA registry integration!');
  console.log('   Run IORA registry commands from another terminal');
  console.log('');
});

// Graceful shutdown
process.on('SIGINT', () => {
  console.log('\n👋 Mock Coral Registry shutting down...');
  console.log(`📊 Final stats: ${registeredServices.size} services registered`);
  process.exit(0);
});

process.on('SIGTERM', () => {
  console.log('\n👋 Mock Coral Registry terminated...');
  process.exit(0);
});
