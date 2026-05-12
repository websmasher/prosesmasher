# Negation-Reframe Bad catches
Source: `/tmp/prosesmasher-textlint-fixtures.json` generated from the existing `.md` fixture corpus.
Scope: textlint `negation-reframe` findings only. This is a judgment file, not a golden baseline.
Total: 115
## Counts
- `bare-not-just-fragment`: 101
- `single-node-or-fragment`: 11
- `pronoun-reframe-pair`: 1
- `other-sentence-pair`: 1
- `same-prefix-pair`: 1

## Findings
### 1. `pronoun-reframe-pair`
- File: `fixtures/explainers/gpt_5_4_mini/burnout-at-work.md:3`
- Reason: factual definition/passive clause, not a rhetorical reframe
- Text:

```text
The World Health Organization describes burnout as an occupational phenomenon that comes from chronic workplace stress that has not been managed well. It is marked by exhaustion, cynicism or mental distance from the job, and reduced effectiveness.
```

### 2. `bare-not-just-fragment`
- File: `fixtures/explainers/gpt_5_4_mini/burnout-at-work.md:5`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 3. `bare-not-just-fragment`
- File: `fixtures/explainers/gpt_5_4_mini/burnout-at-work.md:46`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 4. `bare-not-just-fragment`
- File: `fixtures/explainers/gpt_5_4_mini/does-social-media-harm-attention-span.md:39`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 5. `bare-not-just-fragment`
- File: `fixtures/explainers/gpt_5_4_mini/how-chronic-stress-affects-the-body.md:55`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 6. `bare-not-just-fragment`
- File: `fixtures/explainers/gpt_5_4_mini/how-chronic-stress-affects-the-body.md:112`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 7. `bare-not-just-fragment`
- File: `fixtures/explainers/gpt_5_4_mini/how-loneliness-affects-mental-and-physical-health.md:3`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 8. `single-node-or-fragment`
- File: `fixtures/explainers/gpt_5_4_mini/why-adults-feel-tired-all-the-time.md:9`
- Reason: sentence extraction fragment or ordinary negation without a reframe
- Text:

```text
In other words,
```

### 9. `bare-not-just-fragment`
- File: `fixtures/explainers/gpt_5_4_mini/why-adults-feel-tired-all-the-time.md:41`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 10. `bare-not-just-fragment`
- File: `fixtures/explainers/gpt_5_4_mini/why-children-have-tantrums.md:111`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 11. `bare-not-just-fragment`
- File: `fixtures/explainers/gpt_5_4_mini/why-couples-stop-communicating-well.md:25`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 12. `bare-not-just-fragment`
- File: `fixtures/explainers/gpt_5_4_mini/why-couples-stop-communicating-well.md:79`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 13. `bare-not-just-fragment`
- File: `fixtures/explainers/gpt_5_4_mini/why-modern-friendships-fade-over-time.md:29`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 14. `bare-not-just-fragment`
- File: `fixtures/explainers/gpt_5_4_mini/why-modern-friendships-fade-over-time.md:39`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 15. `bare-not-just-fragment`
- File: `fixtures/explainers/gpt_5_4_mini/why-modern-friendships-fade-over-time.md:53`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 16. `bare-not-just-fragment`
- File: `fixtures/explainers/gpt_5_4_mini/why-people-struggle-to-build-habits.md:21`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 17. `bare-not-just-fragment`
- File: `fixtures/gpt_5_2_chat/adult_procrastination_causes_and_fixes/article.md:54`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 18. `bare-not-just-fragment`
- File: `fixtures/gpt_5_2_chat/how_burnout_develops_at_work/article.md:75`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 19. `bare-not-just-fragment`
- File: `fixtures/gpt_5_2_chat/how_burnout_develops_at_work/article.md:91`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 20. `single-node-or-fragment`
- File: `fixtures/gpt_5_2_chat/screen_time_and_child_attention/article.md:81`
- Reason: sentence extraction fragment or ordinary negation without a reframe
- Text:

```text
Give screens a clear
```

### 21. `bare-not-just-fragment`
- File: `fixtures/gpt_5_2_chat/stress_and_physical_symptoms/article.md:3`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 22. `bare-not-just-fragment`
- File: `fixtures/gpt_5_2_chat/why_couples_stop_communicating/article.md:67`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 23. `single-node-or-fragment`
- File: `fixtures/gpt_5_2_chat/why_couples_stop_communicating/article.md:77`
- Reason: sentence extraction fragment or ordinary negation without a reframe
- Text:

```text
Swap:
```

### 24. `bare-not-just-fragment`
- File: `fixtures/gpt_5_2_chat/why_people_lose_motivation_after_big_goals/article.md:35`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 25. `bare-not-just-fragment`
- File: `fixtures/gpt_5_2_chat/why_people_lose_motivation_after_big_goals/article.md:110`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 26. `bare-not-just-fragment`
- File: `fixtures/gpt_5_2_chat/why_people_wake_up_tired/article.md:13`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 27. `bare-not-just-fragment`
- File: `fixtures/gpt_5_2_chat/why_people_wake_up_tired/article.md:83`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 28. `bare-not-just-fragment`
- File: `fixtures/gpt_5_2_chat/why_people_wake_up_tired/article.md:154`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 29. `bare-not-just-fragment`
- File: `fixtures/gpt_5_2_chat/why_people_wake_up_tired/article.md:219`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 30. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4/adult_procrastination_causes_and_fixes/article.md:39`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 31. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4/how_burnout_develops_at_work/article.md:3`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 32. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4/how_burnout_develops_at_work/article.md:11`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 33. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4/how_burnout_develops_at_work/article.md:47`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 34. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4/how_burnout_develops_at_work/article.md:51`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 35. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4/social_anxiety_in_daily_life/article.md:5`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 36. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4/stress_and_physical_symptoms/article.md:13`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 37. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4/why_couples_stop_communicating/article.md:27`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 38. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4/why_people_lose_motivation_after_big_goals/article.md:7`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 39. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4/why_people_lose_motivation_after_big_goals/article.md:9`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 40. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4/why_people_lose_motivation_after_big_goals/article.md:65`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 41. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4/why_people_struggle_to_build_habits/article.md:9`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 42. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4/why_people_wake_up_tired/article.md:33`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 43. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4_mini/adult_procrastination_causes_and_fixes/article.md:21`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 44. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4_mini/adult_procrastination_causes_and_fixes/article.md:49`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 45. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4_mini/how_burnout_develops_at_work/article.md:3`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 46. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4_mini/how_burnout_develops_at_work/article.md:33`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 47. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4_mini/screen_time_and_child_attention/article.md:13`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 48. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4_mini/screen_time_and_child_attention/article.md:15`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 49. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4_mini/screen_time_and_child_attention/article.md:29`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 50. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4_mini/stress_and_physical_symptoms/article.md:3`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 51. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4_mini/stress_and_physical_symptoms/article.md:5`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 52. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4_mini/stress_and_physical_symptoms/article.md:28`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 53. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4_mini/why_friendships_fade/article.md:18`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 54. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4_mini/why_friendships_fade/article.md:22`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 55. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4_mini/why_people_lose_motivation_after_big_goals/article.md:27`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 56. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4_mini/why_people_lose_motivation_after_big_goals/article.md:29`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 57. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4_mini/why_people_lose_motivation_after_big_goals/article.md:51`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 58. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4_mini/why_people_struggle_to_build_habits/article.md:3`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 59. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4_mini/why_people_struggle_to_build_habits/article.md:11`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 60. `bare-not-just-fragment`
- File: `fixtures/gpt_5_4_mini/why_people_wake_up_tired/article.md:3`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 61. `other-sentence-pair`
- File: `fixtures/haiku/social_anxiety_in_daily_life/article.md:11`
- Reason: legitimate genetic-risk clarification, not slop
- Text:

```text
This doesn't mean that if your parent has social anxiety, you're guaranteed to develop it, but rather that you may inherit a predisposition toward it. This genetic component likely influences how your brain processes social information and perceives threat.
```

### 62. `bare-not-just-fragment`
- File: `fixtures/haiku/why_friendships_fade/article.md:85`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 63. `single-node-or-fragment`
- File: `fixtures/haiku/why_friendships_fade/article.md:91`
- Reason: sentence extraction fragment or ordinary negation without a reframe
- Text:

```text
The next time you think of an old friend, don't let that thought evaporate into the ether.
```

### 64. `single-node-or-fragment`
- File: `fixtures/haiku/why_people_lose_motivation_after_big_goals/article.md:73`
- Reason: sentence extraction fragment or ordinary negation without a reframe
- Text:

```text
When you hit week 3 or 4 and motivation drops, don't panic.
```

### 65. `bare-not-just-fragment`
- File: `fixtures/instagram/gpt_5_4/what-stress-does-to-the-body-and-how-to-slow-it-down.md:1`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 66. `bare-not-just-fragment`
- File: `fixtures/instagram/gpt_5_4/why-overloaded-children-seem-rude.md:9`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 67. `bare-not-just-fragment`
- File: `fixtures/instagram/gpt_5_4_mini/why-overloaded-children-seem-rude.md:15`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 68. `bare-not-just-fragment`
- File: `fixtures/instagram/gpt_5_4_mini/why-rest-is-not-the-same-as-recovery.md:3`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 69. `bare-not-just-fragment`
- File: `fixtures/linkedin/gpt_5_4/burnout-taught-me-about-productivity.md:9`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 70. `bare-not-just-fragment`
- File: `fixtures/linkedin/gpt_5_4/how-stress-hurts-decision-making-at-work.md:1`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 71. `bare-not-just-fragment`
- File: `fixtures/linkedin/gpt_5_4/what-people-get-wrong-about-habit-building.md:21`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 72. `bare-not-just-fragment`
- File: `fixtures/linkedin/gpt_5_4/why-high-performers-procrastinate.md:5`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 73. `bare-not-just-fragment`
- File: `fixtures/linkedin/gpt_5_4/why-high-performers-procrastinate.md:5`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 74. `bare-not-just-fragment`
- File: `fixtures/linkedin/gpt_5_4/why-modern-work-makes-people-feel-constantly-behind.md:17`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 75. `bare-not-just-fragment`
- File: `fixtures/linkedin/gpt_5_4_mini/burnout-taught-me-about-productivity.md:11`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 76. `bare-not-just-fragment`
- File: `fixtures/linkedin/gpt_5_4_mini/burnout-taught-me-about-productivity.md:24`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 77. `bare-not-just-fragment`
- File: `fixtures/linkedin/gpt_5_4_mini/how-stress-hurts-decision-making-at-work.md:1`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 78. `bare-not-just-fragment`
- File: `fixtures/linkedin/gpt_5_4_mini/what-people-get-wrong-about-habit-building.md:21`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 79. `bare-not-just-fragment`
- File: `fixtures/linkedin/gpt_5_4_mini/why-modern-work-makes-people-feel-constantly-behind.md:21`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 80. `bare-not-just-fragment`
- File: `fixtures/opus_4_6/screen_time_and_child_attention/article.md:19`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 81. `bare-not-just-fragment`
- File: `fixtures/opus_4_6/social_anxiety_in_daily_life/article.md:21`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 82. `bare-not-just-fragment`
- File: `fixtures/opus_4_6/why_couples_stop_communicating/article.md:11`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 83. `bare-not-just-fragment`
- File: `fixtures/opus_4_6/stress_and_physical_symptoms/article.md:81`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 84. `single-node-or-fragment`
- File: `fixtures/opus_4_6/why_friendships_fade/article.md:37`
- Reason: sentence extraction fragment or ordinary negation without a reframe
- Text:

```text
A spouse or partner, no matter how loving, cannot fill every role.
```

### 85. `bare-not-just-fragment`
- File: `fixtures/opus_4_6/why_people_lose_motivation_after_big_goals/article.md:23`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 86. `bare-not-just-fragment`
- File: `fixtures/opus_4_6/why_people_lose_motivation_after_big_goals/article.md:41`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 87. `same-prefix-pair`
- File: `fixtures/opus_4_6/why_people_lose_motivation_after_big_goals/article.md:45`
- Reason: truncated extraction
- Text:

```text
The point is not the distance. The point is that the identity of
```

### 88. `single-node-or-fragment`
- File: `fixtures/opus_4_6/why_people_struggle_to_build_habits/article.md:31`
- Reason: sentence extraction fragment or ordinary negation without a reframe
- Text:

```text

```

### 89. `bare-not-just-fragment`
- File: `fixtures/opus_4_6/why_people_struggle_to_build_habits/article.md:57`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 90. `bare-not-just-fragment`
- File: `fixtures/sonnet_4_6/how_burnout_develops_at_work/article.md:27`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 91. `bare-not-just-fragment`
- File: `fixtures/sonnet_4_6/how_burnout_develops_at_work/article.md:63`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 92. `bare-not-just-fragment`
- File: `fixtures/sonnet_4_6/screen_time_and_child_attention/article.md:73`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 93. `bare-not-just-fragment`
- File: `fixtures/sonnet_4_6/social_anxiety_in_daily_life/article.md:41`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 94. `single-node-or-fragment`
- File: `fixtures/sonnet_4_6/stress_and_physical_symptoms/article.md:13`
- Reason: sentence extraction fragment or ordinary negation without a reframe
- Text:

```text
The parasympathetic
```

### 95. `bare-not-just-fragment`
- File: `fixtures/sonnet_4_6/stress_and_physical_symptoms/article.md:19`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 96. `bare-not-just-fragment`
- File: `fixtures/sonnet_4_6/stress_and_physical_symptoms/article.md:49`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 97. `bare-not-just-fragment`
- File: `fixtures/sonnet_4_6/why_couples_stop_communicating/article.md:67`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 98. `single-node-or-fragment`
- File: `fixtures/sonnet_4_6/why_couples_stop_communicating/article.md:71`
- Reason: sentence extraction fragment or ordinary negation without a reframe
- Text:

```text
Using
```

### 99. `bare-not-just-fragment`
- File: `fixtures/sonnet_4_6/why_friendships_fade/article.md:55`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 100. `bare-not-just-fragment`
- File: `fixtures/sonnet_4_6/why_people_struggle_to_build_habits/article.md:13`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 101. `bare-not-just-fragment`
- File: `fixtures/sonnet_4_6/why_people_struggle_to_build_habits/article.md:33`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 102. `bare-not-just-fragment`
- File: `fixtures/sonnet_4_6/why_people_struggle_to_build_habits/article.md:39`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 103. `bare-not-just-fragment`
- File: `fixtures/sonnet_4_6/why_people_wake_up_tired/article.md:112`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 104. `bare-not-just-fragment`
- File: `fixtures/twitter/gpt_5_4/what-stress-does-to-the-body.md:1`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 105. `single-node-or-fragment`
- File: `fixtures/twitter/gpt_5_4/why-children-have-tantrums.md:19`
- Reason: sentence extraction fragment or ordinary negation without a reframe
- Text:

```text
If a child is hurting themselves, cannot recover, or the outbursts are getting worse with age, get a pediatrician involved.
```

### 106. `single-node-or-fragment`
- File: `fixtures/twitter/gpt_5_4/why-habits-fail-even-with-motivation.md:3`
- Reason: sentence extraction fragment or ordinary negation without a reframe
- Text:

```text

```

### 107. `bare-not-just-fragment`
- File: `fixtures/twitter/gpt_5_4/why-habits-fail-even-with-motivation.md:7`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 108. `bare-not-just-fragment`
- File: `fixtures/twitter/gpt_5_4/why-loneliness-affects-mental-and-physical-health.md:1`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 109. `bare-not-just-fragment`
- File: `fixtures/twitter/gpt_5_4_mini/what-stress-does-to-the-body.md:1`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 110. `bare-not-just-fragment`
- File: `fixtures/twitter/gpt_5_4_mini/why-adults-procrastinate-and-what-helps.md:11`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 111. `bare-not-just-fragment`
- File: `fixtures/twitter/gpt_5_4_mini/why-loneliness-affects-mental-and-physical-health.md:1`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 112. `bare-not-just-fragment`
- File: `fixtures/twitter/gpt_5_4_mini/why-people-wake-up-tired-even-after-enough-sleep.md:5`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 113. `bare-not-just-fragment`
- File: `fixtures/twitter/gpt_5_4_mini/why-social-media-affects-attention-span.md:9`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 114. `bare-not-just-fragment`
- File: `fixtures/why_do_we_dream.md:27`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

### 115. `bare-not-just-fragment`
- File: `fixtures/why_do_we_dream.md:31`
- Reason: bare `not just` is not enough evidence; match must include the full contrast or move to another rule
- Text:

```text
not just
```

