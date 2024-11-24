// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from '@coral-xyz/anchor'
import { Cluster, PublicKey } from '@solana/web3.js'
import CryptoticketIDL from '../target/idl/cryptoticket.json'
import type { Cryptoticket } from '../target/types/cryptoticket'

// Re-export the generated IDL and type
export { Cryptoticket, CryptoticketIDL }

// The programId is imported from the program IDL.
export const CRYPTOTICKET_PROGRAM_ID = new PublicKey(CryptoticketIDL.address)

// This is a helper function to get the Cryptoticket Anchor program.
export function getCryptoticketProgram(provider: AnchorProvider) {
  return new Program(CryptoticketIDL as Cryptoticket, provider)
}

// This is a helper function to get the program ID for the Cryptoticket program depending on the cluster.
export function getCryptoticketProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
    case 'testnet':
      // This is the program ID for the Cryptoticket program on devnet and testnet.
      return new PublicKey('CounNZdmsQmWh7uVngV9FXW2dZ6zAgbJyYsvBpqbykg')
    case 'mainnet-beta':
    default:
      return CRYPTOTICKET_PROGRAM_ID
  }
}
