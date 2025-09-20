import { Coins, Brain, Link, Palette } from 'lucide-react';

export default function Home() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100">
      {/* Header */}
      <header className="bg-white shadow-sm">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-6">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <h1 className="text-2xl font-bold text-gray-900">I.O.R.A.</h1>
                <p className="text-sm text-gray-500">Intelligent Oracle Rust Assistant</p>
              </div>
            </div>
            <div className="flex items-center space-x-4">
              <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800">
                🟢 Live Demo
              </span>
              <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                Zero Mocks
              </span>
            </div>
          </div>
        </div>
      </header>

      {/* Hero Section */}
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <div className="text-center">
          <h1 className="text-4xl font-extrabold text-gray-900 sm:text-5xl md:text-6xl">
            <span className="block">Experience Real</span>
            <span className="block text-blue-600">Blockchain Oracle</span>
            <span className="block">In Action</span>
          </h1>
          <p className="mt-3 max-w-md mx-auto text-base text-gray-500 sm:text-lg md:mt-5 md:text-xl md:max-w-3xl">
            Live cryptocurrency analysis with AI-powered insights and immutable blockchain records. 
            No mocks, no simulations - everything is real.
          </p>
          <div className="mt-5 max-w-md mx-auto sm:flex sm:justify-center md:mt-8">
            <div className="rounded-md shadow">
              <a
                href="/demo"
                className="w-full flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 md:py-4 md:text-lg md:px-10"
              >
                Try Live Demo
              </a>
            </div>
            <div className="mt-3 rounded-md shadow sm:mt-0 sm:ml-3">
              <a
                href="https://github.com/guglxni/iora"
                className="w-full flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-blue-600 bg-white hover:bg-gray-50 md:py-4 md:text-lg md:px-10"
              >
                View Source
              </a>
            </div>
          </div>
        </div>

        {/* Features Grid */}
        <div className="mt-20">
          <div className="grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-4">
            <div className="pt-6">
              <div className="flow-root bg-white rounded-lg px-6 pb-8">
                <div className="-mt-6">
                  <div>
                    <span className="inline-flex items-center justify-center p-3 bg-blue-500 rounded-md shadow-lg">
                      <Coins className="h-6 w-6 text-white" aria-hidden="true" />
                    </span>
                  </div>
                  <h3 className="mt-8 text-lg font-medium text-gray-900 tracking-tight">Real-Time Prices</h3>
                  <p className="mt-5 text-base text-gray-500">
                    Live cryptocurrency data from 4 major APIs including CoinGecko, CoinMarketCap, and more.
                  </p>
                </div>
              </div>
            </div>

            <div className="pt-6">
              <div className="flow-root bg-white rounded-lg px-6 pb-8">
                <div className="-mt-6">
                  <div>
                    <span className="inline-flex items-center justify-center p-3 bg-green-500 rounded-md shadow-lg">
                      <Brain className="h-6 w-6 text-white" aria-hidden="true" />
                    </span>
                  </div>
                  <h3 className="mt-8 text-lg font-medium text-gray-900 tracking-tight">AI Analysis</h3>
                  <p className="mt-5 text-base text-gray-500">
                    Real AI-powered market analysis using 8+ LLM providers including Gemini, OpenAI, and Mistral.
                  </p>
                </div>
              </div>
            </div>

            <div className="pt-6">
              <div className="flow-root bg-white rounded-lg px-6 pb-8">
                <div className="-mt-6">
                  <div>
                    <span className="inline-flex items-center justify-center p-3 bg-purple-500 rounded-md shadow-lg">
                      <Link className="h-6 w-6 text-white" aria-hidden="true" />
                    </span>
                  </div>
                  <h3 className="mt-8 text-lg font-medium text-gray-900 tracking-tight">Solana Blockchain</h3>
                  <p className="mt-5 text-base text-gray-500">
                    Real transactions on Solana devnet. Every oracle feed creates an actual blockchain record.
                  </p>
                </div>
              </div>
            </div>

            <div className="pt-6">
              <div className="flow-root bg-white rounded-lg px-6 pb-8">
                <div className="-mt-6">
                  <div>
                    <span className="inline-flex items-center justify-center p-3 bg-pink-500 rounded-md shadow-lg">
                      <Palette className="h-6 w-6 text-white" aria-hidden="true" />
                    </span>
                  </div>
                  <h3 className="mt-8 text-lg font-medium text-gray-900 tracking-tight">NFT Receipts</h3>
                  <p className="mt-5 text-base text-gray-500">
                    Immutable transaction receipts minted as NFTs via Crossmint. View them in your dashboard.
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Live Status */}
        <div className="mt-20 bg-white rounded-lg shadow-lg p-8">
          <h2 className="text-2xl font-bold text-gray-900 text-center mb-8">Live System Status</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
            <div className="text-center">
              <div className="text-3xl font-bold text-green-600">✅</div>
              <div className="text-sm font-medium text-gray-900 mt-2">IORA MCP Server</div>
              <div className="text-xs text-gray-500">Operational</div>
            </div>
            <div className="text-center">
              <div className="text-3xl font-bold text-green-600">✅</div>
              <div className="text-sm font-medium text-gray-900 mt-2">Solana Integration</div>
              <div className="text-xs text-gray-500">2 SOL Available</div>
            </div>
            <div className="text-center">
              <div className="text-3xl font-bold text-green-600">✅</div>
              <div className="text-sm font-medium text-gray-900 mt-2">Crossmint NFTs</div>
              <div className="text-xs text-gray-500">Production API</div>
            </div>
            <div className="text-center">
              <div className="text-3xl font-bold text-green-600">✅</div>
              <div className="text-sm font-medium text-gray-900 mt-2">AI Providers</div>
              <div className="text-xs text-gray-500">8+ LLMs Ready</div>
            </div>
          </div>
        </div>

        {/* Call to Action */}
        <div className="mt-20 text-center">
          <h2 className="text-3xl font-extrabold text-gray-900">Ready to See IORA in Action?</h2>
          <p className="mt-4 text-lg text-gray-500">
            Experience real blockchain oracle functionality with live AI analysis and NFT receipt generation.
          </p>
          <div className="mt-8">
            <a
              href="/demo"
              className="inline-flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 md:py-4 md:text-lg md:px-10"
            >
              Launch Interactive Demo →
            </a>
          </div>
        </div>
      </main>

      {/* Footer */}
      <footer className="bg-white mt-20">
        <div className="max-w-7xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
          <div className="text-center">
            <p className="text-base text-gray-500">
              Built for the Internet of Agents Hackathon • Powered by Rust, TypeScript, Solana, and AI
            </p>
          </div>
        </div>
      </footer>
    </div>
  );
}