import { NextRequest, NextResponse } from 'next/server';
import crypto from 'crypto';

// IORA MCP Server configuration
const IORA_SERVER_URL = process.env.IORA_SERVER_URL || 'http://localhost:7145';
const IORA_SHARED_SECRET = process.env.IORA_SHARED_SECRET || 'iora-production-secret-2025';

// Generate HMAC signature for IORA MCP server authentication
function generateSignature(body: string): string {
  return crypto
    .createHmac('sha256', IORA_SHARED_SECRET)
    .update(body)
    .digest('hex');
}

export async function POST(request: NextRequest) {
  try {
    const { symbol, mint_receipt = true } = await request.json();

    if (!symbol) {
      return NextResponse.json(
        { error: 'Symbol is required' },
        { status: 400 }
      );
    }

    // Prepare request body for IORA MCP server
    const requestBody = JSON.stringify({
      symbol: symbol.toUpperCase(),
      mint_receipt,
    });

    // Generate HMAC signature
    const signature = generateSignature(requestBody);

    console.log(`🚀 Calling IORA MCP server for ${symbol}...`);

    // Call IORA MCP server feed_oracle endpoint
    const response = await fetch(`${IORA_SERVER_URL}/tools/feed_oracle`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-iora-signature': signature,
      },
      body: requestBody,
    });

    if (!response.ok) {
      throw new Error(`IORA server responded with ${response.status}`);
    }

    const ioraResult = await response.json();

    if (!ioraResult.ok) {
      throw new Error(ioraResult.error || 'IORA server returned error');
    }

    // Extract and format the result
    const result = {
      success: true,
      symbol,
      tx: ioraResult.data.tx,
      slot: ioraResult.data.slot,
      digest: ioraResult.data.digest,
      receipt_mint: ioraResult.data.receipt_mint,
      timestamp: new Date().toISOString(),
    };

    console.log(`✅ IORA demo completed for ${symbol}:`, result);

    return NextResponse.json(result);

  } catch (error) {
    console.error('Demo API error:', error);
    
    return NextResponse.json(
      {
        error: 'Failed to execute oracle demo. Please ensure IORA MCP server is running.',
        details: error instanceof Error ? error.message : 'Unknown error',
      },
      { status: 500 }
    );
  }
}

export async function GET() {
  // Health check endpoint
  try {
    const response = await fetch(`${IORA_SERVER_URL}/tools/health`, {
      method: 'GET',
    });

    if (!response.ok) {
      throw new Error(`Health check failed: ${response.status}`);
    }

    const health = await response.json();

    return NextResponse.json({
      status: 'healthy',
      iora_server: IORA_SERVER_URL,
      iora_health: health,
      timestamp: new Date().toISOString(),
    });

  } catch (error) {
    return NextResponse.json(
      {
        status: 'unhealthy',
        error: error instanceof Error ? error.message : 'Unknown error',
        iora_server: IORA_SERVER_URL,
        timestamp: new Date().toISOString(),
      },
      { status: 503 }
    );
  }
}
