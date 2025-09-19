FROM node:20-alpine
WORKDIR /app
COPY package.json package-lock.json* pnpm-lock.yaml* ./
RUN npm i --no-audit --no-fund
COPY tsconfig.json ./tsconfig.json
COPY src ./src
ENV PORT=7070
EXPOSE 7070
CMD ["npm","run","dev"]



