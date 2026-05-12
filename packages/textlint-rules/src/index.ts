import emDashes from "./families/orthography/em-dashes.js";
import prohibitedPhrases from "./families/phrases/prohibited-phrases.js";
import negationReframe from "./families/syntactic-patterns/contrast/negation-reframe.js";
import prohibitedWords from "./families/words/prohibited-words.js";
import { everything } from "./presets/everything.js";

export const rules = {
  "em-dashes": emDashes,
  "negation-reframe": negationReframe,
  "prohibited-phrases": prohibitedPhrases,
  "prohibited-words": prohibitedWords
};

export const presets = {
  everything
};

export default {
  rules,
  presets
};
