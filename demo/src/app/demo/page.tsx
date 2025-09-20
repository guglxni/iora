'use client';

import { useState } from 'react';
import Link from 'next/link';
import { ArrowLeft, Play, Loader2, ExternalLink } from 'lucide-react';

export default function DemoPage() {
  const [selectedSymbol, setSelectedSymbol] = useState('BTC');
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
    { symbol: 'BTC', name: 'Bitcoin' },
    { symbol: 'ETH', name: 'Ethereum' },
    { symbol: 'SOL', name: 'Solana' },
    { symbol: 'ADA', name: 'Cardano' },
  ];

  const runDemo = async () => {
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
                  className={`p-4 rounded-lg border-2 text-center transition-all ${
                    selectedSymbol === crypto.symbol
                      ? 'border-blue-500 bg-blue-50'
                      : 'border-gray-200 hover:border-gray-300'
                  }`}
                >
                  <div className="font-semibold">{crypto.symbol}</div>
                  <div className="text-sm text-gray-500">{crypto.name}</div>
                </button>
              ))}
            </div>
          </div>

          {/* Step 2: Run Demo */}
          <div className="mb-8">
            <h3 className="text-lg font-semibold text-gray-900 mb-4">
              Step 2: Execute Oracle Feed
            </h3>
            <div className="text-center">
              <button
                onClick={runDemo}
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
                    Run Live Demo
                  </>
                )}
              </button>
            </div>
            <p className="text-sm text-gray-500 mt-4 text-center">
              This will: Get real price data → Generate AI analysis → Submit to Solana blockchain → Mint NFT receipt
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
                      <h4 className="font-semibold text-pink-800 mb-2">🎨 NFT Receipt</h4>
                      <div className="text-pink-700">
                        <strong>NFT ID:</strong> 
                        <code className="bg-pink-100 px-2 py-1 rounded text-xs ml-2">
                          {result.receipt_mint}
                        </code>
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
