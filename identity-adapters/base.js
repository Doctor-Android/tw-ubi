//! Base identity adapter
//! 
//! personId = HASH(global_salt || region_id || national_identifier)

const crypto = require('crypto');

/**
 * Compute personId from national identifier
 * @param {string} nationalIdentifier - National ID (never stored)
 * @param {number} regionId - Region ID (1=Spain, 2=USA, etc.)
 * @param {string} globalSalt - Global salt (from config)
 * @returns {string} Hex-encoded 32-byte personId
 */
function computePersonId(nationalIdentifier, regionId, globalSalt) {
  // personId = HASH(global_salt || region_id || national_identifier)
  const input = globalSalt + '|' + regionId + '|' + nationalIdentifier;
  const hash = crypto.createHash('sha256').update(input).digest();
  return hash.toString('hex');
}

module.exports = {
  computePersonId,
};

