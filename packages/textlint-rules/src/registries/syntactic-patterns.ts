import affirmationClosers from "../families/syntactic-patterns/closers/affirmation-closers.js";
import falseQuestion from "../families/syntactic-patterns/closers/false-question.js";
import summativeCloser from "../families/syntactic-patterns/closers/summative-closer.js";
import negationReframe from "../families/syntactic-patterns/contrast/negation-reframe.js";
import llmOpeners from "../families/syntactic-patterns/lead-ins/llm-openers.js";
import demonstrativeEmphasis from "../families/syntactic-patterns/repetition/demonstrative-emphasis.js";
import fragmentStacking from "../families/syntactic-patterns/repetition/fragment-stacking.js";
import tripleRepeat from "../families/syntactic-patterns/repetition/triple-repeat.js";

export const syntacticPatternRules = {
  "affirmation-closers": affirmationClosers,
  "demonstrative-emphasis": demonstrativeEmphasis,
  "false-question": falseQuestion,
  "fragment-stacking": fragmentStacking,
  "llm-openers": llmOpeners,
  "negation-reframe": negationReframe,
  "summative-closer": summativeCloser,
  "triple-repeat": tripleRepeat
};
