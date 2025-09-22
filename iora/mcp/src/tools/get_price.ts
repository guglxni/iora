import { GetPriceIn, GetPriceOut } from "../schemas.js";
import { runIora } from "../lib/spawnIORA.js";
import fetch from "node-fetch";

export async function get_price(input: unknown) {
  const args = GetPriceIn.parse(input);
  
  // Multi-API price aggregation (production-grade implementation)
  const symbol = args.symbol.toUpperCase();
  console.log(`🔍 Fetching ${symbol} price from multiple sources...`);
  
  const pricePromises = [];
  
  // CoinGecko API
  if (process.env.COINGECKO_API_KEY) {
    pricePromises.push(
      fetch(`https://api.coingecko.com/api/v3/simple/price?ids=${getCoinGeckoId(symbol)}&vs_currencies=usd&x_cg_demo_api_key=${process.env.COINGECKO_API_KEY}`)
        .then(res => res.json())
        .then((data: any) => ({ source: 'CoinGecko', price: data[getCoinGeckoId(symbol)]?.usd }))
        .catch(() => null)
    );
  }
  
  // CoinMarketCap API  
  if (process.env.COINMARKETCAP_API_KEY) {
    pricePromises.push(
      fetch(`https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?symbol=${symbol}`, {
        headers: { 'X-CMC_PRO_API_KEY': process.env.COINMARKETCAP_API_KEY }
      })
        .then(res => res.json())
        .then((data: any) => ({ source: 'CoinMarketCap', price: data.data?.[symbol]?.quote?.USD?.price }))
        .catch(() => null)
    );
  }
  
  // CryptoCompare API
  if (process.env.CRYPTOCOMPARE_API_KEY) {
    pricePromises.push(
      fetch(`https://min-api.cryptocompare.com/data/price?fsym=${symbol}&tsyms=USD&api_key=${process.env.CRYPTOCOMPARE_API_KEY}`)
        .then(res => res.json())
        .then((data: any) => ({ source: 'CryptoCompare', price: data.USD }))
        .catch(() => null)
    );
  }
  
  // CoinPaprika API
  if (process.env.COINPAPRIKA_API_KEY) {
    pricePromises.push(
      fetch(`https://api.coinpaprika.com/v1/tickers/${getCoinPaprikaId(symbol)}?quotes=USD`, {
        headers: { 'Authorization': process.env.COINPAPRIKA_API_KEY }
      })
        .then(res => res.json())
        .then((data: any) => ({ source: 'CoinPaprika', price: data.quotes?.USD?.price }))
        .catch(() => null)
    );
  }
  
  // Fallback to single source if no API keys configured
  if (pricePromises.length === 0) {
    console.warn('⚠️ No API keys configured, falling back to single source');
    const out = await runIora("get_price", ["--symbol", args.symbol]);
    return GetPriceOut.parse(out);
  }
  
  // Wait for all API calls and aggregate results
  const results = await Promise.allSettled(pricePromises);
  const validPrices = results
    .map(result => result.status === 'fulfilled' ? result.value : null)
    .filter(result => result && result.price && !isNaN(result.price));
  
  if (validPrices.length === 0) {
    throw new Error(`No valid price data found for ${symbol} from any source`);
  }
  
  // Calculate weighted average (simple average for now)
  const avgPrice = validPrices.reduce((sum, p) => sum + (p?.price || 0), 0) / validPrices.length;
  const sources = validPrices.map(p => p?.source || 'Unknown').join(', ');
  
  console.log(`✅ Multi-API aggregation: ${symbol} = $${avgPrice.toFixed(2)} (${validPrices.length} sources: ${sources})`);
  
  return {
    symbol,
    price: avgPrice,
    source: `Multi-API (${sources})`,
    timestamp: new Date().toISOString(),
    confidence: Math.min(0.95, 0.7 + (validPrices.length * 0.1)) // Higher confidence with more sources
  };
}

// Helper functions to map symbols to API-specific IDs
function getCoinGeckoId(symbol: string): string {
  const mapping: Record<string, string> = {
    'BTC': 'bitcoin',
    'ETH': 'ethereum', 
    'SOL': 'solana',
    'ADA': 'cardano',
    'USDC': 'usd-coin',
    'USDT': 'tether'
  };
  return mapping[symbol] || symbol.toLowerCase();
}

function getCoinPaprikaId(symbol: string): string {
  const mapping: Record<string, string> = {
    'BTC': 'btc-bitcoin',
    'ETH': 'eth-ethereum',
    'SOL': 'sol-solana',
    'ADA': 'ada-cardano',
    'USDC': 'usdc-usd-coin',
    'USDT': 'usdt-tether'
  };
  return mapping[symbol] || `${symbol.toLowerCase()}-${symbol.toLowerCase()}`;
}
