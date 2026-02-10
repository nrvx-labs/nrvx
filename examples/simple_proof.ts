import { createClient, solToLamports } from '../sdk/src';

const { prover, disclosure } = createClient();

function main() {
  const predicate = disclosure.minSol(100);
  const blinding = new Uint8Array(32).fill(42);
  const proof = prover.prove({
    predicate,
    blinding,
    witness: {
      balanceLamports: solToLamports(250),
      saltHex: 'c0ffee',
    },
  });

  const proofHex = Buffer.from(proof.proofHash).toString('hex');
  const publicHex = Buffer.from(proof.publicInputsHash).toString('hex');

  process.stdout.write(`predicate=${predicate.label}\n`);
  process.stdout.write(`proof_hash=${proofHex}\n`);
  process.stdout.write(`public_inputs_hash=${publicHex}\n`);
}

main();
