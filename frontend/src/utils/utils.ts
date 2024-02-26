export function zeroPadHex(inputHex: string) {
  // Remove "0x" prefix if present
  let hexString = inputHex.startsWith("0x") ? inputHex.slice(2) : inputHex;

  // Ensure the hex string is 64 characters long by prepending zeros
  while (hexString.length < 64) {
    hexString = "0" + hexString;
  }

  // Add "0x" prefix back
  hexString = "0x" + hexString;

  return hexString;
}
