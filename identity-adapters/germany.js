//! Germany identity adapter
//! 
//! Region ID: 4
//! National ID: ID card number

const base = require('./base');

/**
 * Compute personId from German ID card number
 * @param {string} idNumber - German ID card number
 * @param {string} globalSalt - Global salt
 * @returns {string} Hex-encoded personId
 */
function computePersonIdFromID(idNumber, globalSalt) {
  // Normalize ID (remove spaces, dashes)
  const normalized = idNumber.replace(/[\s-]/g, '');
  
  // Validate format (alphanumeric, 9-10 chars)
  if (!/^[A-Z0-9]{9,10}$/.test(normalized)) {
    throw new Error('Invalid German ID format');
  }
  
  // Region 4 = Germany
  return base.computePersonId(normalized, 4, globalSalt);
}

module.exports = {
  computePersonIdFromID,
  computePersonId: computePersonIdFromID, // Alias
};

