import type { TxtDocumentNode } from "@textlint/ast-node-types";
import type { TextlintRuleModule } from "@textlint/types";
import {
  cleanSentence,
  type SentenceMatch
} from "../../../shared/matchers/llm-slop.js";
import { allParagraphSentences } from "../../../shared/text/sections.js";

const PREFIXES = ["however, ", "but ", "and ", "so "];
const EVIDENCE_SUBJECTS = ["the evidence"];
const RESEARCH_SUBJECTS = ["the research", "what the research"];
const RESEARCHER_SUBJECTS = ["researchers"];
const EVIDENCE_PREDICATES = ["is strongest", "is not subtle", "points"];
const RESEARCH_PREDICATES = [
  "is not mysterious",
  "points",
  "backs",
  "does show is"
];
const RESEARCHER_PREDICATES = ["keep finding"];
const PRESTIGE_SUFFIXES = [
  "'s work is famous for a reason",
  "\u2019s work is famous for a reason"
];

function matchesSubjectPredicate(
  text: string,
  subjects: readonly string[],
  predicates: readonly string[]
): boolean {
  return subjects.some((subject) =>
    predicates.some((predicate) => text.startsWith(`${subject} ${predicate}`))
  );
}

function matchAuthorityPadding(sentence: string): SentenceMatch | undefined {
  const stripped = cleanSentence(sentence, PREFIXES);

  if (PRESTIGE_SUFFIXES.some((suffix) => stripped.includes(suffix))) {
    return { kind: "prestige-frame", signal: "work is famous for a reason" };
  }

  if (
    matchesSubjectPredicate(stripped, EVIDENCE_SUBJECTS, EVIDENCE_PREDICATES) ||
    stripped.startsWith("the strongest recent evidence points")
  ) {
    return { kind: "evidence-frame", signal: "the evidence" };
  }

  if (
    matchesSubjectPredicate(stripped, RESEARCH_SUBJECTS, RESEARCH_PREDICATES) ||
    stripped.startsWith("the broader research backs") ||
    matchesSubjectPredicate(
      stripped,
      RESEARCHER_SUBJECTS,
      RESEARCHER_PREDICATES
    )
  ) {
    return { kind: "research-frame", signal: "the research" };
  }

  return undefined;
}

const rule: TextlintRuleModule = (context) => {
  const { Syntax, RuleError, locator, report } = context;

  return {
    [Syntax.Document](node: TxtDocumentNode): void {
      for (const item of allParagraphSentences(node)) {
        const matched = matchAuthorityPadding(item.sentence.text);
        if (matched === undefined) {
          continue;
        }

        report(
          item.paragraph,
          new RuleError(
            `Authority padding found: ${matched.signal}. Cite the actual evidence or remove the frame.`,
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
