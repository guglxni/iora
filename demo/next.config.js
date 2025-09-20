/** @type {import('next').NextConfig} */
const nextConfig = {
  env: {
    IORA_SERVER_URL: process.env.IORA_SERVER_URL || 'http://localhost:7145',
    IORA_SHARED_SECRET: process.env.IORA_SHARED_SECRET || 'iora-production-secret-2025',
  },
  async headers() {
    return [
      {
        source: '/api/:path*',
        headers: [
          { key: 'Access-Control-Allow-Origin', value: '*' },
          { key: 'Access-Control-Allow-Methods', value: 'GET, POST, PUT, DELETE, OPTIONS' },
          { key: 'Access-Control-Allow-Headers', value: 'Content-Type, Authorization' },
        ],
      },
    ];
  },
};

module.exports = nextConfig;
