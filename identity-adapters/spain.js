//! Spain identity adapter
//! 
//! Region ID: 1
//! National ID: DNI (e.g., "12345678A")

const base = require('./base');

/**
 * Compute personId from Spanish DNI
 * @param {string} dni - Spanish DNI (e.g., "12345678A")
 * @param {string} globalSalt - Global salt
 * @returns {string} Hex-encoded personId
 */
function computePersonIdFromDNI(dni, globalSalt) {
  // Normalize DNI (uppercase, remove spaces)
  const normalized = dni.toUpperCase().replace(/\s/g, '');
  
  // Validate format (8 digits + 1 letter)
  if (!/^\d{8}[A-Z]$/.test(normalized)) {
    throw new Error('Invalid DNI format');
  }
  
  // Region 1 = Spain
  return base.computePersonId(normalized, 1, globalSalt);
}

module.exports = {
  computePersonIdFromDNI,
  computePersonId: computePersonIdFromDNI, // Alias
};

