FROM node:17.3-alpine

ARG DATABASE_URL=${DATABASE_URL}
ENV DATABASE_URL=${DATABASE_URL}

ARG WALLET_PRIVATE_KEY=${WALLET_PRIVATE_KEY}
ENV WALLET_PRIVATE_KEY=${WALLET_PRIVATE_KEY}

ARG MAINNET_HTTP_URL=${MAINNET_HTTP_URL}
ENV MAINNET_HTTP_URL=${MAINNET_HTTP_URL}

ARG DEVNET_HTTP_URL=${DEVNET_HTTP_URL}
ENV DEVNET_HTTP_URL=${DEVNET_HTTP_URL}

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

RUN npx prisma generate

# Creates a "dist" folder with the production build
RUN npm run build

# Start the server using the production build
CMD [ "node", "dist/main.js" ]