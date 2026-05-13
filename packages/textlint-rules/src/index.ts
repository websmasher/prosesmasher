import { everything } from "./presets/everything.js";
import { metricRules } from "./registries/metrics.js";
import { orthographyRules } from "./registries/orthography.js";
import { phraseRules } from "./registries/phrases.js";
import { syntacticPatternRules } from "./registries/syntactic-patterns.js";
import { termPolicyRules } from "./registries/term-policy.js";
import { wordRules } from "./registries/words.js";

export const rules = {
  ...metricRules,
  ...orthographyRules,
  ...phraseRules,
  ...syntacticPatternRules,
  ...termPolicyRules,
  ...wordRules
};

export const presets = {
  everything
};

export default {
  rules,
  presets
};
