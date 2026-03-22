# Required terms + recommended terms (term pool) checks

## Summary

Added 2 new term checks from the steady-parent validator pattern:
- `required-terms` — ALL configured terms must appear (per-term pass/fail)
- `recommended-terms` — at least N from a pool must appear, with optional stem matching

34 total checks now. 13 new tests.

## Config format

```json
{
  "terms": {
    "requiredTerms": ["ownership", "borrowing"],
    "recommendedTerms": {
      "terms": ["ownership", "borrowing", "lifetimes", "traits", "async"],
      "minCount": 3,
      "allowInflections": true
    }
  }
}
```

## Key files

- `domain/types/src/config.rs` — added `TermPool` struct, `required_terms`, `recommended_terms` fields
- `terms/required_terms.rs` — per-term presence check with case-insensitive matching
- `terms/recommended_terms.rs` — pool count check with optional rough stemming
- `config_dto.rs` — `TermPoolDto` for JSON deserialization
