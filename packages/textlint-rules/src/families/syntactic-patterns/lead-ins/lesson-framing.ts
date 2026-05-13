import type { TxtDocumentNode } from "@textlint/ast-node-types";
import type { TextlintRuleModule } from "@textlint/types";
import {
  cleanSentence,
  startsWithAnyText,
  type SentenceMatch
} from "../../../shared/matchers/llm-slop.js";
import { allParagraphSentences } from "../../../shared/text/sections.js";

const PREFIXES = ["and ", "but ", "so "];
const LESSON_SUMMARY_PATTERNS = [
  "the biggest lesson was simple",
  "the practical lesson for me was simple",
  "the practical lesson was simple"
];
const FIX_CUES = [
  "plain",
  "boring",
  "not heroic",
  "usually smaller than people want"
];

function matchLessonFraming(sentence: string): SentenceMatch | undefined {
  const stripped = cleanSentence(sentence, PREFIXES);
  const lesson = startsWithAnyText(stripped, LESSON_SUMMARY_PATTERNS);

  if (lesson !== undefined) {
    return { kind: "lesson-summary", signal: lesson };
  }

  if (!stripped.startsWith("the fix is ")) {
    return undefined;
  }

  const cue = FIX_CUES.find((item) => stripped.includes(item));
  return cue === undefined ? undefined : { kind: "fix-wrapper", signal: cue };
}

const rule: TextlintRuleModule = (context) => {
  const { Syntax, RuleError, locator, report } = context;

  return {
    [Syntax.Document](node: TxtDocumentNode): void {
      for (const item of allParagraphSentences(node)) {
        const matched = matchLessonFraming(item.sentence.text);
        if (matched === undefined) {
          continue;
        }

        report(
          item.paragraph,
          new RuleError(
            `Lesson framing found: ${matched.signal}. Replace the wrapper with the lesson.`,
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
