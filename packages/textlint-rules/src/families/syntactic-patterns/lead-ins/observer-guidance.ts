import type { TxtDocumentNode } from "@textlint/ast-node-types";
import type { TextlintRuleModule } from "@textlint/types";
import {
  cleanSentence,
  startsWithAnyText,
  trimTerminalPunctuation,
  type SentenceMatch
} from "../../../shared/matchers/llm-slop.js";
import { allParagraphSentences } from "../../../shared/text/sections.js";

const PREFIXES = ["and ", "but ", "so "];
const OBSERVER_PATTERNS = [
  "you see it everywhere",
  "you see it after almost every",
  "you can watch it happen in real time",
  "you can tell the difference quickly"
];
const READER_ADDRESS_PATTERNS = ["if this hits home"];
const STUCK_PATTERNS = ["this is where people get stuck"];
const WHERE_BRIDGE_PATTERNS = [
  "that is where the confusion slips in",
  "that is where a lot of work gets lost",
  "that is where the guilt starts",
  "that is where the real progress lives",
  "that is where a lot of the misunderstanding begins",
  "that is where culture becomes visible"
];

function exactStart(
  text: string,
  patterns: readonly string[]
): string | undefined {
  const matched = startsWithAnyText(text, patterns);
  return matched !== undefined && text === matched ? matched : undefined;
}

function matchObserverGuidance(sentence: string): SentenceMatch | undefined {
  const stripped = cleanSentence(sentence, PREFIXES);
  const trimmed = trimTerminalPunctuation(stripped);
  const observer = exactStart(trimmed, OBSERVER_PATTERNS);

  if (observer !== undefined) {
    return { kind: "observer-frame", signal: observer };
  }

  const reader = startsWithAnyText(stripped, READER_ADDRESS_PATTERNS);
  if (reader !== undefined) {
    return { kind: "reader-address", signal: reader };
  }

  const stuck = exactStart(trimmed, STUCK_PATTERNS);
  if (stuck !== undefined) {
    return { kind: "stuck-frame", signal: stuck };
  }

  const bridge = exactStart(trimmed, WHERE_BRIDGE_PATTERNS);
  return bridge === undefined
    ? undefined
    : { kind: "where-bridge", signal: bridge };
}

const rule: TextlintRuleModule = (context) => {
  const { Syntax, RuleError, locator, report } = context;

  return {
    [Syntax.Document](node: TxtDocumentNode): void {
      for (const item of allParagraphSentences(node)) {
        const matched = matchObserverGuidance(item.sentence.text);
        if (matched === undefined) {
          continue;
        }

        report(
          item.paragraph,
          new RuleError(
            `Observer guidance found: ${matched.signal}. Replace the bridge with concrete evidence.`,
            {
              padding: locator.range([
                item.source.originalStartFor(item.sentence.start),
                item.source.originalEndFor(item.sentence.end)
              ])
            }
          )
        );
      }
    }
  };
};

export default rule;
