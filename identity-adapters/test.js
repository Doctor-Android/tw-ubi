//! Test identity adapters

const spain = require('./spain');
const usa = require('./usa');
const mexico = require('./mexico');
const germany = require('./germany');

// Test salt - NOT for production use
const GLOBAL_SALT = 'test-salt-change-in-production';

console.log('Testing identity adapters...\n');

// Test Spain
try {
  const personId1 = spain.computePersonIdFromDNI('12345678A', GLOBAL_SALT);
  console.log('Spain DNI "12345678A":', personId1);
  console.log('Length:', personId1.length, '(should be 64 hex chars = 32 bytes)');
} catch (e) {
  console.error('Spain error:', e.message);
}

// Test USA
try {
  const personId2 = usa.computePersonIdFromSSN('123-45-6789', GLOBAL_SALT);
  console.log('\nUSA SSN "123-45-6789":', personId2);
  console.log('Length:', personId2.length);
} catch (e) {
  console.error('USA error:', e.message);
}

// Test Mexico
try {
  const personId3 = mexico.computePersonIdFromCURP('ABCD123456HDFGHI01', GLOBAL_SALT);
  console.log('\nMexico CURP:', personId3);
  console.log('Length:', personId3.length);
} catch (e) {
  console.error('Mexico error:', e.message);
}

// Test Germany
try {
  const personId4 = germany.computePersonIdFromID('T22000129', GLOBAL_SALT);
  console.log('\nGermany ID:', personId4);
  console.log('Length:', personId4.length);
} catch (e) {
  console.error('Germany error:', e.message);
}

console.log('\n✅ All adapters working!');

