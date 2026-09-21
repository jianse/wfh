import cbor from 'cbor'

/**
 * Decode attestationObject and extract credentialId + publicKey
 * @param {string} attestationObject - Base64URL encoded string
 * @returns {object} { credentialId, publicKey }
 */
function extractCredential(attestationObject) {
  // Step 1: Base64URL decode
  const base64urlDecode = (input) => {
    input = input.replace(/-/g, '+').replace(/_/g, '/')
    const pad = input.length % 4
    if (pad) {
      input += '='.repeat(4 - pad)
    }
    return Buffer.from(input, 'base64')
  }

  const attestationBuffer = base64urlDecode(attestationObject)

  // Step 2: CBOR decode
  const attestationStruct = cbor.decodeAllSync(attestationBuffer)[0]

  console.log('Attestation Struct:', attestationStruct)

  // Step 3: Extract authData
  const authData = attestationStruct.authData
  let offset = 0

  // Skip RP ID hash (32 bytes), flags (1 byte), signCount (4 bytes)
  offset += 32 + 1 + 4

  // Step 4: Parse attestedCredentialData
  offset += 16

  const credIdLen = authData.readUInt16BE(offset)
  offset += 2

  const credentialId = authData.slice(offset, offset + credIdLen)
  offset += credIdLen

  // Step 5: Parse publicKey (CBOR)
  const publicKeyBytes = authData.slice(offset)
  const publicKey = cbor.decodeAllSync(publicKeyBytes)[0]

  return {
    credentialId: credentialId.toString('hex'),
    publicKey,
  }
}

// Example usage:
const attestationObject =
  'o2NmbXRkbm9uZWdhdHRTdG10oGhhdXRoRGF0YVikSZYN5YgOjGh0NBcPZHZgW4/krrmihjLHmVzzuoMdl2NFAAAAAAiYcFjK3EuBtuEw3lDcvpYAICasKMyLB6T+98gqnCsj9oi2UhbBttXQkGN0/xufqwUKpQECAyYgASFYIORrML2weZlRfhNx8vBRIq38GuzQLoaX2sd7T/IQq5NPIlgg38zoSL1eeBtME3/xcjlos7HkcXviDPe+dCbwXzChYsc='

const result = extractCredential(attestationObject)
console.log('Credential ID:', result.credentialId)
console.log('Public Key:', result.publicKey)
