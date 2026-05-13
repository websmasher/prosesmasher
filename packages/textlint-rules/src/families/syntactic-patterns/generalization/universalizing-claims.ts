import type { TxtDocumentNode } from "@textlint/ast-node-types";
import type { TextlintRuleModule } from "@textlint/types";
import {
  cleanSentence,
  tokens,
  startsWithWords
} from "../../../shared/matchers/llm-slop.js";
import { allParagraphSentences } from "../../../shared/text/sections.js";

const PREFIXES = [
  "however, ",
  "but ",
  "and ",
  "so ",
  "ultimately, ",
  "after all, ",
  "in the end, "
];
const SUBJECT_PATTERNS = [
  ["everyone"],
  ["everybody"],
  ["we", "all"],
  ["most", "people"],
  ["most", "of", "us"],
  ["for", "most", "people"],
  ["no", "one"],
  ["nobody"]
] as const;
const DESIRE_VERBS = [
  "want",
  "wants",
  "need",
  "needs",
  "hope",
  "hopes",
  "deserve",
  "deserves",
  "crave",
  "craves"
];
const CERTAINTY_VERBS = ["know", "knows"];
const HUMAN_GROUP_SUBJECTS = [
  "adults",
  "children",
  "couples",
  "dads",
  "families",
  "kids",
  "moms",
  "parents",
  "people",
  "students",
  "teachers"
];
const GROUP_BEHAVIOR_GERUNDS = ["reaching", "trying", "waiting", "hoping"];

function matchGroupBehavior(words: readonly string[]): string | undefined {
  const [first, subject, third, gerund] = words;

  if (
    first === "most" &&
    subject !== undefined &&
    third === "keep" &&
    gerund !== undefined &&
    HUMAN_GROUP_SUBJECTS.includes(subject) &&
    GROUP_BEHAVIOR_GERUNDS.includes(gerund)
  ) {
    return `most ${subject} keep ${gerund}`;
  }

  return undefined;
}

function matchUniversalizing(sentence: string): string | undefined {
  const words = tokens(cleanSentence(sentence, PREFIXES));
  const group = matchGroupBehavior(words);

  if (group !== undefined) {
    return group;
  }

  for (const subject of SUBJECT_PATTERNS) {
    if (!startsWithWords(words, subject)) {
      continue;
    }

    const window = words.slice(subject.length, subject.length + 4);
    const verb =
      window.find((candidate) => DESIRE_VERBS.includes(candidate)) ??
      window.find((candidate) => CERTAINTY_VERBS.includes(candidate));

    if (verb !== undefined) {
      return `${subject.join(" ")} ${verb}`;
    }
  }

  return undefined;
}

const rule: TextlintRuleModule = (context) => {
  const { Syntax, RuleError, locator, report } = context;

  return {
    [Syntax.Document](node: TxtDocumentNode): void {
      for (const item of allParagraphSentences(node)) {
        const matched = matchUniversalizing(item.sentence.text);
        if (matched === undefined) {
          continue;
        }

        report(
          item.paragraph,
          new RuleError(
            `Universalizing claim found: ${matched}. Replace the broad claim with a bounded claim.`,
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
