# Cassegrain (MVP)

## Project Overview
The Cassegrain Backend is the core API server that orchestrates interactions between the Frontend, Blockchain Layer (Solana + Magic Block ER), Off-Chain Storage (Filecoin IPFS), and various external services (IoT, Oracles, AI models). It handles business logic, data persistence, and ensures the integrity of supply chain operations.

This MVP backend provides the necessary APIs for user management, product registration, event logging, data querying, and blockchain synchronization.

## Features (MVP)

**User Management:** Handles user registration, authentication, and profile management for Manufacturers, Retailers, and Consumers.

**Product Lifecycle API:**
- Endpoints for creating/registering new products (metadata, IPFS hashes).
- Endpoints for logging granular supply chain events (e.g., manufacturing, packaging, shipment updates, quality checks) to Magic Block ER.

**Data Querying & Synchronization:**
- Retrieves and processes product history from Solana Mainnet (including ER settlements).
- Fetches detailed metadata and files from Filecoin IPFS.
- Serves aggregated and formatted data to the frontend.
- IoT & External API Integration: Receives data streams from IoT devices and integrates with third-party logistics APIs.

**Blockchain Orchestration:**
- Manages interactions with Solana smart contracts (product registration, ownership, escrow).
- Sends high-frequency events to and monitors settlements from Magic Block Ephemeral Rollups.
- Interfaces with Pyth/Chainlink oracles for verified off-chain data.

**Messaging System:**
- Basic API for supplier-buyer communication.

## Tech Stack
- Language: JavaScript (Node.js)
- Framework: Express.js (if required)
- Database: PostgreSQL (for off-chain data like user profiles, cached blockchain data, messaging)
- NoSQL/Cache (Optional): Redis (for session management, caching frequently accessed data)
- Blockchain Interaction: Solana, Magic Block SDK/API
- Off-Chain Storage: Filecoin IPFS client library 
- Oracles Integration: Pyth/Chainlink client libraries
- AI Integration: Python child processes or separate microservice interaction (if ML models are exposed via an API)
- Authentication: JWT (JSON Web Tokens) or session-based authentication

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

**Prerequisites**
- Node.js (LTS version recommended)
- npm or Yarn
- PostgreSQL database instance
- (Optional) Redis instance
- Solana development environment (for deploying and interacting with smart contracts)
- IPFS node (local or remote service like Pinata)

**Installation**

Clone the repository

2.  Install dependencies:
    `npm install`  
    _or_ 
    
    `yarn install`

3.  Configure Environment Variables (not yet applicable):

```
Create a .env file in the root of the project and add the following:
PORT=5000 DATABASE_URL="postgresql://user:password@host:port/database" JWT_SECRET="your_jwt_secret_key" SOLANA_RPC_URL="https://api.devnet.solana.com" # Or your local validator MAGIC_BLOCK_API_KEY="your_magic_block_api_key" MAGIC_BLOCK_RPC_URL="your_magic_block_rpc_url" # From Magic Block documentation IPFS_API_URL="http://localhost:5001" # Or your IPFS gateway # Add any other keys or configurations for Oracles, AI services, etc.
```

4.  Database Setup:
* Ensure your PostgreSQL instance is running.
* Run database migrations (if using a migration tool)

5.  Run the development server:
    `npm run dev`
    _or_
     
    `yarn dev`
The API server will run on http://localhost:5000 (or your configured port).

## API Endpoints (These are just for example purposes and are not implemented yet)
- POST /api/auth/register - Register a new user
- POST /api/auth/login - Authenticate user
- POST /api/products/register - Register a new product on-chain (Manufacturer)
- POST /api/products/:productId/event - Log a new supply chain event for a product (Manufacturer/IoT)
- GET /api/products/:productId/history - Retrieve full product history for consumer/retailer
- GET /api/marketplace/products - Browse available products
- POST /api/transactions/initiate - Initiate a P2P transaction with escrow (Retailer)
- GET /api/users/:userId/profile - Get user profile
- POST /api/messages - Send a message

```
Project Structure (High-Level)
cassegrain-backend/
├── src/
│   ├── api/                # API route definitions (e.g., auth, products, transactions)
│   ├── services/           # Business logic and external service interactions (blockchain, IPFS, IoT)
│   ├── models/             # Database models and schemas
│   ├── controllers/        # Handle incoming requests and delegate to services
│   ├── middlewares/        # Authentication, validation, error handling
│   ├── utils/              # Utility functions, helpers
│   ├── config/             # Environment setup, database connection
│   └── app.js              # Main application entry point
├── database/               # Database migrations, seeders
├── .env                    # Environment variables
├── package.json
└── README.md
```
🤝 Contributing
Contributions are welcome! Please follow these steps:

Fork the repository.
Create a new branch (git checkout -b feature/your-feature-name).
Make your changes.
Commit your changes (git commit -m 'feat: Implement new API endpoint').
Push to the branch (git push origin feature/your-feature-name).
Open a Pull Request.
Please ensure your code adheres to the project's coding standards, includes API documentation, and has appropriate tests.

