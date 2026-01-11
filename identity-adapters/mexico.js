//! Mexico identity adapter
//! 
//! Region ID: 3
//! National ID: CURP

const base = require('./base');

/**
 * Compute personId from Mexican CURP
 * @param {string} curp - Mexican CURP
 * @param {string} globalSalt - Global salt
 * @returns {string} Hex-encoded personId
 */
function computePersonIdFromCURP(curp, globalSalt) {
  // Normalize CURP (uppercase, remove spaces)
  const normalized = curp.toUpperCase().replace(/\s/g, '');
  
  // Validate format (18 characters)
  if (!/^[A-Z]{4}\d{6}[HM][A-Z]{5}[0-9A-Z]\d$/.test(normalized)) {
    throw new Error('Invalid CURP format');
  }
  
  // Region 3 = Mexico
  return base.computePersonId(normalized, 3, globalSalt);
}

module.exports = {
  computePersonIdFromCURP,
  computePersonId: computePersonIdFromCURP, // Alias
};

