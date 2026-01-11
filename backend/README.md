# TW-UBI Backend

Constitutional income infrastructure backend.

## Setup

1. **Install PostgreSQL**
   ```bash
   sudo apt install postgresql postgresql-contrib
   ```

2. **Create database**
   ```bash
   sudo -u postgres createdb ubi
   ```

3. **Run migrations**
   ```bash
   psql ubi < migrations/001_initial_schema.sql
   ```

4. **Configure environment**
   ```bash
   cp .env.example .env
   # Edit .env with your settings
   ```

5. **Run server**
   ```bash
   cargo run
   ```

## API Endpoints

- `POST /api/users/register` - Register person
- `POST /api/users/reset-wallet` - Reset wallet (MFA required)
- `POST /api/ubi/claim` - Claim UBI for current epoch
- `POST /api/conversion/request` - Request UE→BU conversion
- `POST /api/conversion/claim/{id}` - Claim unlocked BU
- `POST /api/oracle/submit` - Submit oracle data
- `GET /api/admin/export-state` - Export system state (forkability)
- `GET /health` - Health check

## Constitutional Invariants

All invariants are enforced:
- One person = one claim per epoch
- UE issuance fixed at 696 UE per epoch
- BU supply fixed at genesis
- UE balances don't decay
- Conversion power decays via rateIndex
- All state changes emit events

