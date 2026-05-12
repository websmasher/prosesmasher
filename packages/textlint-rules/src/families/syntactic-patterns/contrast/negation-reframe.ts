import type { TextlintRuleModule } from "@textlint/types";
import { findNegationReframes } from "../../../shared/matchers/syntactic-templates.js";

const rule: TextlintRuleModule = (context) => {
  const { Syntax, RuleError, getSource, locator, report } = context;

  return {
    [Syntax.Str](node): void {
      const text = getSource(node);

      for (const match of findNegationReframes(text)) {
        report(
          node,
          new RuleError(
            `Negation reframe found: "${match.text}". Rewrite without the not-X-then-Y construction.`,
            {
              padding: locator.range([match.start, match.end])
            }
          )
        );
      }
    }
  };
};

export default rule;
