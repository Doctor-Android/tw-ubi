# Identity Adapters

Country-agnostic identity adapters for computing `personId` from national identifiers.

## personId Derivation

**Uniform for all regions:**
```
personId = HASH(global_salt || region_id || national_identifier)
```

The system **NEVER** sees national identifiers. Only `personId` exists inside the system.

## Adapter Structure

Each adapter:
1. Receives national identifier (off-chain)
2. Computes `personId = HASH(global_salt || region_id || national_identifier)`
3. Returns hex-encoded `personId` (32 bytes)

## Example Adapters

- `spain.js` - Spain (DNIe/Certificado/Cl@ve)
- `usa.js` - USA (SSN-based, simplified)
- `mexico.js` - Mexico (CURP-based)
- `germany.js` - Germany (ID card)

## Usage

```javascript
const adapter = require('./spain')
const personId = adapter.computePersonId('12345678A', 1) // region 1 = Spain
// Returns: hex-encoded 32-byte personId
```

## Security

- `global_salt` must be kept secret
- National identifiers never stored
- Only `personId` sent to backend

