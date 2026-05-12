# Orthography textlint diff

Rust was run with `em-dashes,smart-quotes,sentence-case,exclamation-density,fake-timestamps,colon-dramatic`. Textlint was run with the migrated orthography rules on the same 202 fixture files.

## Counts

- colon-dramatic: Rust evidence 69, Rust match-count 69, textlint 76
- em-dashes: Rust evidence 92, Rust match-count 134, textlint 134
- exclamation-density: Rust evidence 1, Rust match-count 1, textlint 1
- sentence-case: Rust evidence 139, Rust match-count 139, textlint 139
- smart-quotes: Rust evidence 343, Rust match-count 710, textlint 344

## File-level differences

### colon-dramatic

- `fixtures/gpt_5_2_chat/screen_time_and_child_attention/article.md`: Rust 0, textlint 1
- `fixtures/gpt_5_2_chat/why_couples_stop_communicating/article.md`: Rust 15, textlint 16
- `fixtures/gpt_5_4/adult_procrastination_causes_and_fixes/article.md`: Rust 2, textlint 3
- `fixtures/gpt_5_4/how_burnout_develops_at_work/article.md`: Rust 0, textlint 1
- `fixtures/gpt_5_4_mini/why_people_struggle_to_build_habits/article.md`: Rust 0, textlint 1
- `fixtures/linkedin/gpt_5_4/what-healthy-workplace-culture-looks-like.md`: Rust 1, textlint 2
- `fixtures/twitter/gpt_5_4/why-habits-fail-even-with-motivation.md`: Rust 0, textlint 1

### em-dashes

- `fixtures/haiku/adult_procrastination_causes_and_fixes/article.md`: Rust 12, textlint 20
- `fixtures/haiku/how_burnout_develops_at_work/article.md`: Rust 7, textlint 9
- `fixtures/haiku/screen_time_and_child_attention/article.md`: Rust 6, textlint 10
- `fixtures/haiku/social_anxiety_in_daily_life/article.md`: Rust 7, textlint 14
- `fixtures/haiku/stress_and_physical_symptoms/article.md`: Rust 7, textlint 10
- `fixtures/haiku/why_couples_stop_communicating/article.md`: Rust 12, textlint 21
- `fixtures/haiku/why_friendships_fade/article.md`: Rust 13, textlint 17
- `fixtures/haiku/why_people_lose_motivation_after_big_goals/article.md`: Rust 12, textlint 14
- `fixtures/haiku/why_people_struggle_to_build_habits/article.md`: Rust 11, textlint 14

### exclamation-density

- No file-level count differences.

### sentence-case

- No file-level count differences.

### smart-quotes

- `fixtures/gpt_5_2_chat/why_people_struggle_to_build_habits/article.md`: Rust 18, textlint 19

## Reviewed textlint-only colon-dramatic examples

- `fixtures/gpt_5_2_chat/screen_time_and_child_attention/article.md`: "What happened?"
- `fixtures/gpt_5_2_chat/why_couples_stop_communicating/article.md`: 30 p.m.
- `fixtures/gpt_5_4/adult_procrastination_causes_and_fixes/article.md`: 00 a.m.
- `fixtures/gpt_5_4/how_burnout_develops_at_work/article.md`: no Slack after 6 p.m.
- `fixtures/gpt_5_4_mini/adult_procrastination_causes_and_fixes/article.md`: a scoping review](https://pmc.ncbi.nlm.nih.gov/articles/PMC8868816/)).
- `fixtures/gpt_5_4_mini/why_people_lose_motivation_after_big_goals/article.md`: not all motivation is equal**.
- `fixtures/gpt_5_4_mini/why_people_struggle_to_build_habits/article.md`: 30 a.m.
- `fixtures/haiku/why_couples_stop_communicating/article.md`: "It sounds like you're feeling.
- `fixtures/haiku/why_people_lose_motivation_after_big_goals/article.md`: motivation drops *before* competence rises.
- `fixtures/linkedin/gpt_5_4/what-healthy-workplace-culture-looks-like.md`: 46 p.m.
- `fixtures/twitter/gpt_5_4/why-habits-fail-even-with-motivation.md`: 12 p.m.
