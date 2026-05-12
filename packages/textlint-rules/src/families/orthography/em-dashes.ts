import type { TextlintRuleModule } from "@textlint/types";

const CLOSED_EM_DASH = "\u2014";

function isWhitespace(value: string | undefined): boolean {
  return value === undefined || value.trim() === "";
}

function isClosedEmDash(text: string, index: number): boolean {
  if (text[index] !== CLOSED_EM_DASH) {
    return false;
  }

  return !isWhitespace(text[index - 1]) && !isWhitespace(text[index + 1]);
}

const rule: TextlintRuleModule = (context) => {
  const { Syntax, RuleError, getSource, locator, report } = context;

  return {
    [Syntax.Str](node) {
      const text = getSource(node);

      for (let index = 0; index < text.length; index += 1) {
        if (!isClosedEmDash(text, index)) {
          continue;
        }

        report(
          node,
          new RuleError("Closed em dash found. Replace it with a comma, colon, parenthesis, or spaced dash.", {
            padding: locator.range([index, index + CLOSED_EM_DASH.length])
          })
        );
      }
    }
  };
};

export default rule;
