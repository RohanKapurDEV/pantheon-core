FROM node:17.3-alpine

ARG DATABASE_URL=${DATABASE_URL}
ENV DATABASE_URL=${DATABASE_URL}

ARG WALLET_PRIVATE_KEY=${WALLET_PRIVATE_KEY}
ENV WALLET_PRIVATE_KEY=${WALLET_PRIVATE_KEY}

ARG SOLANA_RPC_HTTP_MAINNET_URL=${SOLANA_RPC_HTTP_MAINNET_URL}
ENV SOLANA_RPC_HTTP_MAINNET_URL=${SOLANA_RPC_HTTP_MAINNET_URL}

ARG SOLANA_RPC_HTTP_DEVNET_URL=${SOLANA_RPC_HTTP_DEVNET_URL}
ENV SOLANA_RPC_HTTP_DEVNET_URL=${SOLANA_RPC_HTTP_DEVNET_URL}

ARG CLUSTER=${CLUSTER}
ENV CLUSTER=${CLUSTER}

# Create app directory
WORKDIR /usr/src/app

# A wildcard is used to ensure both package.json AND package-lock.json are copied
COPY package*.json ./

# Install app dependencies
RUN npm install

# Bundle app source
COPY . .

# Creates a "dist" folder with the production build
RUN npm run build

# Start the server using the production build
CMD [ "node", "dist/main.js" ]