import type { TxtDocumentNode } from "@textlint/ast-node-types";
import type { TextlintRuleModule } from "@textlint/types";
import {
  cleanSentence,
  containsAny,
  type SentenceMatch
} from "../../../shared/matchers/llm-slop.js";
import {
  allParagraphSentences,
  type SectionSentence
} from "../../../shared/text/sections.js";

const PREFIXES = [
  "ultimately, ",
  "overall, ",
  "in the end, ",
  "in short, ",
  "and ",
  "but "
];
const IMPORTANCE_CUES = ["most important", "single most important", "deepest"];
const SUMMARY_NOUNS = ["insight", "reason", "idea", "step"];
const AUTHORITY_CLOSE_PATTERNS = [
  "the research is clear",
  "science is clear",
  "decades of research",
  "research leaves no doubt"
];
const ACCEPTANCE_CLOSE_PATTERNS = [
  "not something you have to accept as normal",
  "not something you need to accept as normal",
  "is not a luxury"
];
const RESPONSE_CLOSE_PATTERNS = ["the practical response is plain"];
const COMPRESSION_CLOSE_PATTERNS = [
  "the whole trick",
  "the core fact",
  "the rest is detail"
];

function matchInsightClose(text: string): string | undefined {
  if (
    IMPORTANCE_CUES.some((cue) => text.includes(cue)) &&
    SUMMARY_NOUNS.some((noun) => text.includes(noun))
  ) {
    return "importance+summary-noun";
  }

  return undefined;
}

function matchConclusion(
  sentence: string,
  isTail: boolean
): SentenceMatch | undefined {
  const stripped = cleanSentence(sentence, PREFIXES);

  if (isTail) {
    const insight = matchInsightClose(stripped);
    if (insight !== undefined) {
      return { kind: "insight-close", signal: insight };
    }

    const authority = containsAny(stripped, AUTHORITY_CLOSE_PATTERNS);
    if (authority !== undefined) {
      return { kind: "authority-close", signal: authority };
    }

    const acceptance = containsAny(stripped, ACCEPTANCE_CLOSE_PATTERNS);
    if (acceptance !== undefined) {
      return { kind: "acceptance-close", signal: acceptance };
    }
  }

  const response = containsAny(stripped, RESPONSE_CLOSE_PATTERNS);
  if (response !== undefined) {
    return { kind: "response-close", signal: response };
  }

  const compression = containsAny(stripped, COMPRESSION_CLOSE_PATTERNS);
  return compression === undefined
    ? undefined
    : { kind: "compression-close", signal: compression };
}

const rule: TextlintRuleModule = (context) => {
  const { Syntax, RuleError, locator, report } = context;

  return {
    [Syntax.Document](node: TxtDocumentNode): void {
      const sentences = allParagraphSentences(node);
      const tailStart = Math.max(sentences.length - 3, 0);

      sentences.forEach((item: SectionSentence, index: number) => {
        const matched = matchConclusion(item.sentence.text, index >= tailStart);
        if (matched === undefined) {
          return;
        }

        report(
          item.paragraph,
          new RuleError(
            `Boilerplate conclusion found: ${matched.signal}. Replace the closer with a specific ending.`,
            {
              padding: locator.range([
                item.source.originalStartFor(item.sentence.start),
                item.source.originalEndFor(item.sentence.end)
              ])
            }
          )
        );
      });
    }
  };
};

export default rule;
