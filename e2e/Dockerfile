FROM mcr.microsoft.com/playwright:v1.52.0-noble
WORKDIR /app
COPY package.json package-lock.json ./
RUN npm ci
COPY test-e2e.mjs ./
CMD ["node", "test-e2e.mjs"]
