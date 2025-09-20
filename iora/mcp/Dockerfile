FROM node:20-alpine
WORKDIR /app

# Copy package files
COPY package.json package-lock.json* ./

# Install dependencies
RUN npm ci --only=production --no-audit --no-fund

# Install dev dependencies for build
RUN npm install --save-dev typescript tsx

# Copy source code
COPY tsconfig.json ./
COPY src ./src
COPY wallets ./wallets

# Build the application
RUN npm run build

# Set environment variables
ENV NODE_ENV=production
ENV PORT=8000

# Expose port
EXPOSE 8000

# Start the application in production mode
CMD ["npm", "start"]



# Force rebuild
# Rebuild Sat Sep 20 22:30:01 IST 2025
