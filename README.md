# ErcNftMetadataIndexer

## Description

A smart contract suite implementing a novel NFT fractionalization protocol using ERC-1155 under the hood, enabling on-chain governance of fractionalized assets via weighted voting and decentralized reserve management.

## Features

- Indexes ERC-721 and ERC-1155 token metadata from Ethereum, Polygon, and Arbitrum blockchains.
- Persists indexed NFT metadata in a PostgreSQL database optimized for JSONB storage and efficient querying.
- Implements a robust event listener using WebSockets to capture `Transfer` and `MetadataUpdate` events from specified smart contracts.
- Utilizes a rate-limiting strategy with exponential backoff to prevent API throttling from blockchain data providers.
- Provides a GraphQL API endpoint for querying NFT metadata, including filtering, sorting, and pagination.
- Generates thumbnail images and optimized media files for NFTs using serverless functions.
- Supports configurable metadata refresh intervals to ensure data accuracy and handle dynamic metadata updates.
- Employs a distributed caching layer using Redis to minimize database load and accelerate API response times.
## Installation

```bash
pip install git+https://github.com/Lyne6666/ErcNftMetadataIndexer.git
```

## Usage

```bash
python -m ercnftmetadataindexer --verbose
```

## Contributing

We welcome contributions! Here's how to get started:

1. Fork this repository
2. Create a new branch for your feature (`git checkout -b feature/your-feature`)
3. Commit your changes (`git commit -am 'Add some awesome feature'`)
4. Push to the branch (`git push origin feature/your-feature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.
