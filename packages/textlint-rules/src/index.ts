import colonDramatic from "./families/orthography/colon-dramatic.js";
import emDashes from "./families/orthography/em-dashes.js";
import exclamationDensity from "./families/orthography/exclamation-density.js";
import fakeTimestamps from "./families/orthography/fake-timestamps.js";
import sentenceCase from "./families/orthography/sentence-case.js";
import smartQuotes from "./families/orthography/smart-quotes.js";
import prohibitedPhrases from "./families/phrases/prohibited-phrases.js";
import negationReframe from "./families/syntactic-patterns/contrast/negation-reframe.js";
import prohibitedWords from "./families/words/prohibited-words.js";
import { everything } from "./presets/everything.js";

export const rules = {
  "colon-dramatic": colonDramatic,
  "em-dashes": emDashes,
  "exclamation-density": exclamationDensity,
  "fake-timestamps": fakeTimestamps,
  "negation-reframe": negationReframe,
  "prohibited-phrases": prohibitedPhrases,
  "prohibited-words": prohibitedWords,
  "sentence-case": sentenceCase,
  "smart-quotes": smartQuotes
};

export const presets = {
  everything
};

export default {
  rules,
  presets
};
