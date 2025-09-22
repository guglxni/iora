'use client';

import { useState } from 'react';
import Link from 'next/link';
import { ArrowLeft, Play, Loader2, ExternalLink } from 'lucide-react';

export default function DemoPage() {
  const [selectedSymbol, setSelectedSymbol] = useState('BTC');
  const [demoType, setDemoType] = useState('full');
  const [isLoading, setIsLoading] = useState(false);
  const [result, setResult] = useState<{
    price?: number;
    source?: string;
    analysis?: {
      insight: string;
      recommendation: string;
      confidence: number;
    };
    tx?: string;
    slot?: number;
    receipt_mint?: string;
    error?: string;
  } | null>(null);

  const cryptos = [
    { 
      symbol: 'BTC', 
      name: 'Bitcoin',
      logo: 'https://cryptologos.cc/logos/bitcoin-btc-logo.svg',
      color: 'text-orange-600'
    },
    { 
      symbol: 'ETH', 
      name: 'Ethereum',
      logo: 'https://cryptologos.cc/logos/ethereum-eth-logo.svg',
      color: 'text-blue-600'
    },
    { 
      symbol: 'SOL', 
      name: 'Solana',
      logo: 'https://cryptologos.cc/logos/solana-sol-logo.svg',
      color: 'text-purple-600'
    },
    { 
      symbol: 'ADA', 
      name: 'Cardano',
      logo: 'https://cryptologos.cc/logos/cardano-ada-logo.svg',
      color: 'text-blue-700'
    },
  ];

  const runDemoType = async (type: string) => {
    setDemoType(type);
    setIsLoading(true);
    setResult(null);

    try {
      // This will call your IORA MCP server
      const response = await fetch('/api/demo/oracle-feed', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          symbol: selectedSymbol,
          mint_receipt: true,
        }),
      });

      const data = await response.json();
      setResult(data);
    } catch (error) {
      console.error('Demo error:', error);
      setResult({
        error: 'Demo temporarily unavailable. Please try again.',
      });
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100">
      {/* Header */}
      <header className="bg-white shadow-sm">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-6">
            <div className="flex items-center">
              <Link
                href="/"
                className="flex items-center text-gray-500 hover:text-gray-700 mr-4"
              >
                <ArrowLeft className="h-5 w-5 mr-1" />
                Back
              </Link>
              <div className="flex-shrink-0">
                <h1 className="text-2xl font-bold text-gray-900">IORA Live Demo</h1>
                <p className="text-sm text-gray-500">Interactive Blockchain Oracle</p>
              </div>
            </div>
            <div className="flex items-center space-x-4">
              <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800">
                🟢 Live
              </span>
            </div>
          </div>
        </div>
      </header>

      {/* Demo Interface */}
      <main className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <div className="bg-white rounded-lg shadow-lg p-8">
          <h2 className="text-3xl font-bold text-gray-900 mb-8 text-center">
            Experience Real Blockchain Oracle
          </h2>

          {/* Step 1: Select Cryptocurrency */}
          <div className="mb-8">
            <h3 className="text-lg font-semibold text-gray-900 mb-4">
              Step 1: Select Cryptocurrency
            </h3>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              {cryptos.map((crypto) => (
                <button
                  key={crypto.symbol}
                  onClick={() => setSelectedSymbol(crypto.symbol)}
                  className={`p-4 rounded-lg border-2 text-center transition-all hover:shadow-md ${
                    selectedSymbol === crypto.symbol
                      ? 'border-blue-500 bg-blue-50 shadow-lg transform scale-105'
                      : 'border-gray-200 hover:border-gray-300 hover:bg-gray-50'
                  }`}
                >
                  <div className="flex flex-col items-center space-y-2">
                    <img 
                      src={crypto.logo} 
                      alt={`${crypto.name} logo`}
                      className="w-10 h-10 object-contain"
                      onError={(e) => {
                        // Fallback to a simple colored circle with symbol if image fails
                        e.currentTarget.style.display = 'none';
                        const fallback = e.currentTarget.nextElementSibling as HTMLElement;
                        if (fallback) fallback.style.display = 'flex';
                      }}
                    />
                    <div 
                      className={`w-10 h-10 rounded-full flex items-center justify-center text-white font-bold text-sm hidden ${crypto.color.replace('text-', 'bg-')}`}
                      style={{ display: 'none' }}
                    >
                      {crypto.symbol.substring(0, 2)}
                    </div>
                    <div>
                      <div className={`font-semibold ${crypto.color}`}>{crypto.symbol}</div>
                      <div className="text-xs text-gray-500">{crypto.name}</div>
                    </div>
                  </div>
                </button>
              ))}
            </div>
          </div>

          {/* Step 2: Choose Demo Type */}
          <div className="mb-8">
            <h3 className="text-lg font-semibold text-gray-900 mb-4">
              Step 2: Choose Demo Type
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
              <button
                onClick={() => runDemoType('full')}
                disabled={isLoading}
                className={`p-4 rounded-lg border-2 transition-colors ${
                  demoType === 'full'
                    ? 'border-blue-500 bg-blue-50'
                    : 'border-blue-200 bg-blue-50 hover:bg-blue-100'
                } disabled:opacity-50 disabled:cursor-not-allowed`}
              >
                <div className="text-center">
                  <div className="font-semibold text-blue-800">🔗 Full Workflow</div>
                  <div className="text-xs text-blue-600 mt-1">Complete Oracle Pipeline</div>
                </div>
              </button>
              <button
                onClick={() => runDemoType('rag')}
                disabled={isLoading}
                className={`p-4 rounded-lg border-2 transition-colors ${
                  demoType === 'rag'
                    ? 'border-green-500 bg-green-50'
                    : 'border-green-200 bg-green-50 hover:bg-green-100'
                } disabled:opacity-50 disabled:cursor-not-allowed`}
              >
                <div className="text-center">
                  <div className="font-semibold text-green-800">🧠 RAG Analysis</div>
                  <div className="text-xs text-green-600 mt-1">AI with Contextual Data</div>
                </div>
              </button>
              <button
                onClick={() => runDemoType('health')}
                disabled={isLoading}
                className={`p-4 rounded-lg border-2 transition-colors ${
                  demoType === 'health'
                    ? 'border-purple-500 bg-purple-50'
                    : 'border-purple-200 bg-purple-50 hover:bg-purple-100'
                } disabled:opacity-50 disabled:cursor-not-allowed`}
              >
                <div className="text-center">
                  <div className="font-semibold text-purple-800">📊 System Health</div>
                  <div className="text-xs text-purple-600 mt-1">API Status & Analytics</div>
                </div>
              </button>
              <button
                onClick={() => runDemoType('cache')}
                disabled={isLoading}
                className={`p-4 rounded-lg border-2 transition-colors ${
                  demoType === 'cache'
                    ? 'border-orange-500 bg-orange-50'
                    : 'border-orange-200 bg-orange-50 hover:bg-orange-100'
                } disabled:opacity-50 disabled:cursor-not-allowed`}
              >
                <div className="text-center">
                  <div className="font-semibold text-orange-800">💾 Cache Status</div>
                  <div className="text-xs text-orange-600 mt-1">Performance Metrics</div>
                </div>
              </button>
            </div>
            <div className="text-center">
              <button
                onClick={() => runDemoType('full')}
                disabled={isLoading}
                className="inline-flex items-center px-8 py-4 border border-transparent text-base font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isLoading ? (
                  <>
                    <Loader2 className="animate-spin -ml-1 mr-3 h-5 w-5" />
                    Processing...
                  </>
                ) : (
                  <>
                    <Play className="-ml-1 mr-3 h-5 w-5" />
                    Run Full Demo
                  </>
                )}
              </button>
            </div>
            <p className="text-sm text-gray-500 mt-4 text-center">
              Choose a demo type or run the complete workflow with all features enabled
            </p>
          </div>

          {/* Step 3: Results */}
          {result && (
            <div className="mb-8">
              <h3 className="text-lg font-semibold text-gray-900 mb-4">
                Step 3: Live Results
              </h3>
              
              {result.error ? (
                <div className="bg-red-50 border border-red-200 rounded-lg p-6">
                  <div className="text-red-800">
                    <strong>Error:</strong> {result.error}
                  </div>
                </div>
              ) : (
                <div className="space-y-6">
                  {/* Price Data */}
                  {result.price && (
                    <div className="bg-green-50 border border-green-200 rounded-lg p-6">
                      <h4 className="font-semibold text-green-800 mb-2">✅ Real Price Data</h4>
                      <div className="text-green-700">
                        <strong>{selectedSymbol}:</strong> ${result.price.toLocaleString()} USD
                        <br />
                        <span className="text-sm">Source: {result.source || 'Multi-API'}</span>
                      </div>
                    </div>
                  )}

                  {/* AI Analysis */}
                  {result.analysis && (
                    <div className="bg-blue-50 border border-blue-200 rounded-lg p-6">
                      <h4 className="font-semibold text-blue-800 mb-2">🤖 AI Analysis</h4>
                      <div className="text-blue-700">
                        <strong>Insight:</strong> {result.analysis.insight}
                        <br />
                        <strong>Recommendation:</strong> {result.analysis.recommendation}
                        <br />
                        <span className="text-sm">Confidence: {(result.analysis.confidence * 100).toFixed(1)}%</span>
                      </div>
                    </div>
                  )}

                  {/* Blockchain Transaction */}
                  {result.tx && (
                    <div className="bg-purple-50 border border-purple-200 rounded-lg p-6">
                      <h4 className="font-semibold text-purple-800 mb-2">⛓️ Solana Transaction</h4>
                      <div className="text-purple-700">
                        <strong>Transaction:</strong> 
                        <code className="bg-purple-100 px-2 py-1 rounded text-xs ml-2">
                          {result.tx}
                        </code>
                        <br />
                        <strong>Slot:</strong> {result.slot?.toLocaleString()}
                        <br />
                        <a
                          href={`https://explorer.solana.com/tx/${result.tx}?cluster=devnet`}
                          target="_blank"
                          rel="noopener noreferrer"
                          className="inline-flex items-center text-purple-600 hover:text-purple-800 mt-2"
                        >
                          View on Solana Explorer
                          <ExternalLink className="ml-1 h-4 w-4" />
                        </a>
                      </div>
                    </div>
                  )}

                  {/* NFT Receipt */}
                  {result.receipt_mint && (
                    <div className="bg-pink-50 border border-pink-200 rounded-lg p-6">
                      <h4 className="font-semibold text-pink-800 mb-2">🎨 NFT Receipt Minted</h4>
                      <div className="text-pink-700">
                        <strong>NFT ID:</strong> 
                        <code className="bg-pink-100 px-2 py-1 rounded text-xs ml-2">
                          {result.receipt_mint}
                        </code>
                        <br />
                        <strong>Collection:</strong> default-solana
                        <br />
                        <strong>Description:</strong> Oracle receipt for {selectedSymbol} at ${result.price?.toLocaleString()} 
                        <br />
                        <strong>Transaction:</strong> {result.tx?.substring(0, 16)}...
                        <br />
                        <a
                          href="https://www.crossmint.com/console/collections"
                          target="_blank"
                          rel="noopener noreferrer"
                          className="inline-flex items-center text-pink-600 hover:text-pink-800 mt-2"
                        >
                          View in Crossmint Dashboard
                          <ExternalLink className="ml-1 h-4 w-4" />
                        </a>
                        <br />
                        <span className="text-sm mt-2 block">✅ Minted via Crossmint production API</span>
                      </div>
                    </div>
                  )}
                </div>
              )}
            </div>
          )}

          {/* Info Panel */}
          <div className="bg-gray-50 rounded-lg p-6 mt-8">
            <h4 className="font-semibold text-gray-900 mb-3">What happens during this demo?</h4>
            <ul className="space-y-2 text-sm text-gray-600">
              <li>• <strong>Real API calls</strong> to 4+ cryptocurrency data providers</li>
              <li>• <strong>Live AI analysis</strong> using production LLM providers (Gemini, OpenAI, Mistral)</li>
              <li>• <strong>Actual blockchain transaction</strong> submitted to Solana devnet</li>
              <li>• <strong>Real NFT minting</strong> via Crossmint production API</li>
              <li>• <strong>Verifiable results</strong> you can check on Solana Explorer and Crossmint dashboard</li>
            </ul>
            <p className="text-xs text-gray-500 mt-4">
              <strong>Zero mocks or simulations.</strong> Everything you see is real and verifiable on-chain.
            </p>
          </div>
        </div>
      </main>
    </div>
  );
}
