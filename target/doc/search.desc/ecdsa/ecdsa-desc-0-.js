searchState.loadedDescShard("ecdsa", 0, "RustCrypto: ECDSA\nOID for ECDSA with SHA-224 digests.\nOID for ECDSA with SHA-256 digests.\nOID for ECDSA with SHA-384 digests.\nOID for ECDSA with SHA-512 digests.\nEncoded elliptic curve point sized appropriately for a …\nContains the error value\nSignature errors.\nMaximum supported value for the recovery ID (inclusive).\nECDSA signature with low-S normalization applied.\nContains the success value\nMarker trait for elliptic curves with prime order.\nRecovery IDs, a.k.a. “recid”.\nByte representation of a signature.\nResult type.\nECDSA signature (fixed-size). Generic over elliptic curve …\nFixed-size byte array containing an ECDSA signature\nSupport for decoding/encoding signatures as bytes.\nSize of a fixed sized signature for the given elliptic …\nAn extended <code>Signature</code> type which is parameterized by an …\nECDSA secret key used for signing. Generic over prime …\nECDSA public key used for verifying signatures. Generic …\nBorrow the inner <code>AffinePoint</code> for this public key.\nBorrow the secret <code>NonZeroScalar</code> value for this key.\nSupport for ASN.1 DER-encoded ECDSA signatures as …\nGet the length of this signature when encoded.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nInitialize <code>VerifyingKey</code> from an affine point.\nConvert a <code>u8</code> into a <code>RecoveryId</code>.\nInitialize signing key from a raw scalar serialized as a …\nParse a signature from fixed-width bytes, i.e. 2 * the …\nParse a signature from fixed-with bytes.\nParse a signature from ASN.1 DER.\nInitialize <code>VerifyingKey</code> from an <code>EncodedPoint</code>.\nCreate a <code>Signature</code> from the serialized <code>r</code> and <code>s</code> scalar …\nInitialize <code>VerifyingKey</code> from a SEC1-encoded public key.\nInitialize signing key from a raw scalar serialized as a …\nParse a signature from a byte slice.\nParse a signature from a byte slice.\nCreate a new error with an associated source.\nLow-level ECDSA primitives.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nDid the affine x-coordinate of 𝑘×𝑮 overflow the …\nIs the affine y-coordinate of 𝑘×𝑮 odd?\nCreate a new error with no associated source\nCreate a new <code>RecoveryId</code> from the following 1-bit arguments:\nCreate a new signature with an explicitly provided OID.\nCreate a new signature, determining the OID from the given …\nNormalize signature into “low S” form as described in …\nGet the ECDSA OID for this signature.\nGet the <code>r</code> component of this signature\nGenerate a cryptographically random <code>SigningKey</code>.\nRecover a <code>VerifyingKey</code> from the given message <code>Digest</code>, …\nRecover a <code>VerifyingKey</code> from the given message, signature, …\nRecover a <code>VerifyingKey</code> from the given <code>prehash</code> of a …\nGet the <code>s</code> component of this signature\nSign the given message digest, returning a signature and …\nSign the given message prehash, returning a signature and …\nSign the given message, hashing it with the curve’s …\nGet the fixed-width ECDSA signature.\nSplit the signature into its <code>r</code> and <code>s</code> components, …\nSplit the signature into its <code>r</code> and <code>s</code> scalars.\nConvert this <code>RecoveryId</code> into a <code>u8</code>.\nEncode signature as its byte representation.\nSerialize this <code>SigningKey</code> as bytes\nSerialize this signature as bytes.\nSerialize this signature as bytes.\nSerialize this signature as ASN.1 DER.\nSerialize this <code>VerifyingKey</code> as a SEC1 <code>EncodedPoint</code>, …\nEncode signature as a byte vector.\nGiven a public key, message digest, and signature, use …\nGiven a public key, message, and signature, use trial …\nGiven a public key, message digest, and signature, use …\nGet the <code>VerifyingKey</code> which corresponds to this <code>SigningKey</code>.\nMaximum overhead of an ASN.1 DER-encoded ECDSA signature …\nMaximum size of an ASN.1 DER encoded signature for the …\nASN.1 DER-encoded signature as specified in RFC5912 …\nBorrow this signature as a byte slice\nReturns the argument unchanged.\nParse signature from DER-encoded bytes.\nCalls <code>U::from(self)</code>.\nGet the length of the signature in bytes\nPreferred digest to use when computing ECDSA signatures …\nBind a preferred <code>Digest</code> algorithm to an elliptic curve …\nTry to sign the given prehashed message using ECDSA.\nVerify the given prehashed message using ECDSA.\nPartial implementation of the <code>bits2int</code> function as defined …\nSign a prehashed message digest using the provided secret …\nTry to sign the prehashed message.\nTry to sign the given message digest deterministically …\nVerify message digest against the provided signature.\nVerify the prehashed message against the provided ECDSA …\nVerify the prehashed message against the provided ECDSA …")