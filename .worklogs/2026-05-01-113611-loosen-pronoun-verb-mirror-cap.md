# Loosen pronoun verb-mirror word-count cap

## Summary

Bumped `pronoun_verb_mirror_corrective` word-count cap from 22/22 to 24/36 so it catches verb-mirror slop where the affirmative reframe runs long. Real example surfaced by user: a 34-word S2 ("We start with the keywords the company wants to rank for, look at the visible competitor URLs from the SERP for each one, then build the competitor list from whoever shows up the most.") was being skipped by the previous cap.

## Decisions

The slop signal in `we/you/they do not [V] X. we/you/they [V] Y.` is the verb mirror itself — repeating the same verb across S1 and S2 with the same pronoun subject. Sentence length is not a strong precision lever for this particular shape; it's a tightness lever inherited from earlier matchers where shorter reframes correlated with pithier slop.

For verb-mirror specifically, an explanatory long S2 is just as slop-y as a short pithy one when the verb is mirrored. Bumped S2 cap to 36 (matching the order-of-magnitude of Pattern A's 28-word cap with some headroom). S1 cap moved 22 → 24 for symmetry but barely matters in practice (S1 is structurally short: pronoun + "do not" + verb + short complement).

The `[V] to [INFINITIVE]` skip remains the load-bearing FP guard.

## Audit

Re-ran on the 202-file fixture corpus. **Same 6 pronoun verb-mirror hits as before, no new false positives.** The cap loosening exposed only the missing real-world case, not extra noise.

All 91 cadence-pattern tests still pass.

## Key files

- `apps/prosesmasher/crates/app/checks/cadence-patterns/runtime/src/heur_05_negation_reframe.rs:1456` — single line cap change

## Next steps

- Same audit/threshold review pattern can be applied to other matchers if real-world reports surface long-S2 cases.
