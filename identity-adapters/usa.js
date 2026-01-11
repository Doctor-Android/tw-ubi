//! USA identity adapter
//! 
//! Region ID: 2
//! National ID: SSN (simplified, for demo only)

const base = require('./base');

/**
 * Compute personId from US SSN (simplified)
 * @param {string} ssn - US SSN (e.g., "123-45-6789")
 * @param {string} globalSalt - Global salt
 * @returns {string} Hex-encoded personId
 */
function computePersonIdFromSSN(ssn, globalSalt) {
  // Normalize SSN (remove dashes)
  const normalized = ssn.replace(/-/g, '');
  
  // Validate format (9 digits)
  if (!/^\d{9}$/.test(normalized)) {
    throw new Error('Invalid SSN format');
  }
  
  // Region 2 = USA
  return base.computePersonId(normalized, 2, globalSalt);
}

module.exports = {
  computePersonIdFromSSN,
  computePersonId: computePersonIdFromSSN, // Alias
};

