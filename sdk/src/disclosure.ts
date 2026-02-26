import BN from 'bn.js';

import { Predicate, PredicateKind, DisclosurePolicy } from './types';

export class DisclosureBuilder {
  constructor(private readonly namespace: string) {}

  minSol(sol: number): Predicate {
    return {
      kind: PredicateKind.MinBalance,
      threshold: new BN(sol).mul(new BN(1_000_000_000)),
      label: `min_sol_${sol}`,
    };
  }

  maxSol(sol: number): Predicate {
    return {
      kind: PredicateKind.MaxBalance,
      threshold: new BN(sol).mul(new BN(1_000_000_000)),
      label: `max_sol_${sol}`,
    };
  }

  tradingVolume(thresholdUsd: number): Predicate {
    return {
      kind: PredicateKind.TradingVolume,
      threshold: new BN(thresholdUsd),
      label: `volume_${thresholdUsd}`,
    };
  }

  notBlacklisted(): Predicate {
    return {
      kind: PredicateKind.NotBlacklisted,
      threshold: new BN(0),
      label: 'not_blacklisted',
    };
  }

  ownsNft(collectionHash: string): Predicate {
    if (!/^[0-9a-f]{64}$/i.test(collectionHash)) {
      throw new Error('collectionHash must be a 64-character hex string');
    }
    return {
      kind: PredicateKind.NftOwnership,
      threshold: new BN(collectionHash.slice(0, 16), 16),
      label: `nft_${collectionHash.slice(0, 8)}`,
    };
  }

  asPolicy(predicate: Predicate, commitment: Uint8Array, expiresInSeconds: number): DisclosurePolicy {
    const expiresAt = Math.floor(Date.now() / 1000) + expiresInSeconds;
    return {
      commitment,
      predicateKind: predicate.kind,
      threshold: predicate.threshold,
      expiresAt,
    };
  }

  namespaceTag(): string {
    return this.namespace;
  }
}
// rev-01090
