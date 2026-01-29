import { PublicKey, Connection, Commitment } from '@solana/web3.js';

export interface RpcOptions {
  endpoint: string;
  commitment?: Commitment;
}

export function createConnection(options: RpcOptions): Connection {
  return new Connection(options.endpoint, options.commitment ?? 'confirmed');
}

export function deriveRegistryAddress(programId: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync([Buffer.from('fixr-registry')], programId);
}

export function derivePolicyAddress(
  owner: PublicKey,
  commitment: Uint8Array,
  programId: PublicKey,
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('policy'), owner.toBuffer(), Buffer.from(commitment)],
    programId,
  );
}

export function deriveProofRecordAddress(
  policy: PublicKey,
  proofHash: Uint8Array,
  programId: PublicKey,
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('proof'), policy.toBuffer(), Buffer.from(proofHash)],
    programId,
  );
}
