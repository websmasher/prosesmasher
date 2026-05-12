import type { TxtParentNode } from "@textlint/ast-node-types";
import type { TextlintRuleModule } from "@textlint/types";
import { RuleHelper } from "textlint-rule-helper";
import { splitSentences } from "../../../shared/text/sentences.js";
import { sourceText } from "../../../shared/text/traverse.js";
import { splitWhitespace } from "../../../shared/text/whitespace.js";

type RepeatMatch = {
  readonly end: number;
  readonly opener: string;
  readonly sentences: readonly string[];
  readonly start: number;
};

function cleanWord(word: string): string {
  return word
    .toLocaleLowerCase("en")
    .replaceAll("\u2019", "'")
    .replaceAll("\u2018", "'");
}

function firstWord(sentence: string): string | undefined {
  const word = splitWhitespace(sentence)[0];
  if (word === undefined) {
    return undefined;
  }

  return cleanWord(word);
}

function findTripleRepeats(text: string): RepeatMatch[] {
  const sentences = splitSentences(text);
  const matches: RepeatMatch[] = [];

  if (sentences.length < 3) {
    return matches;
  }

  for (let index = 0; index <= sentences.length - 3; index += 1) {
    const first = sentences[index];
    const second = sentences[index + 1];
    const third = sentences[index + 2];
    if (first === undefined || second === undefined || third === undefined) {
      continue;
    }

    const firstOpener = firstWord(first.text);
    const secondOpener = firstWord(second.text);
    const thirdOpener = firstWord(third.text);
    if (
      firstOpener === undefined ||
      firstOpener !== secondOpener ||
      secondOpener !== thirdOpener
    ) {
      continue;
    }

    matches.push({
      end: third.end,
      opener: firstOpener,
      sentences: [first.text, second.text, third.text],
      start: first.start
    });
  }

  return matches;
}

const rule: TextlintRuleModule = (context) => {
  const { Syntax, RuleError, locator, report } = context;
  const helper = new RuleHelper(context);
  const ignoredParents = [
    Syntax.List,
    Syntax.ListItem,
    Syntax.Table,
    Syntax.TableCell
  ];

  return {
    [Syntax.Paragraph](node: TxtParentNode): void {
      if (helper.isChildNode(node, ignoredParents)) {
        return;
      }

      const source = sourceText(node);
      for (const match of findTripleRepeats(source.text)) {
        report(
          node,
          new RuleError(
            `Triple repeat opener found: "${match.opener}". Vary the sentence openers.`,
            {
              padding: locator.range([
                source.originalStartFor(match.start),
                source.originalEndFor(match.end)
              ])
            }
          )
        );
      }
    }
  };
};

export default rule;
