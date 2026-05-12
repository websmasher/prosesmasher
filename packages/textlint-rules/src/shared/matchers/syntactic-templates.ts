import { findPhraseMatches } from "./phrases.js";
import { normalizeForMatch } from "../text/normalize.js";
import { splitSentences, type SplitSentence } from "../text/sentences.js";
import { wordTokens, type Token } from "../text/tokens.js";

const NEGATION_WORDS = new Set([
  "not",
  "isn't",
  "aren't",
  "wasn't",
  "weren't",
  "don't",
  "doesn't",
  "didn't",
  "can't",
  "cannot",
  "won't",
  "wouldn't",
  "shouldn't",
  "couldn't"
]);

const DO_AUXILIARIES = new Set(["do", "does", "did"]);

export type NegationReframeMatch = {
  readonly end: number;
  readonly start: number;
  readonly text: string;
};

function findNegationIndex(tokens: readonly Token[]): number | undefined {
  for (let index = 0; index < tokens.length; index += 1) {
    const token = tokens[index];

    if (token !== undefined && NEGATION_WORDS.has(token.normalized)) {
      return index;
    }
  }

  return undefined;
}

function positivePrefixBeforeNegation(
  tokens: readonly Token[],
  negationIndex: number
): readonly string[] {
  const prefix = tokens
    .slice(0, negationIndex)
    .map((token) => token.normalized);
  const last = prefix.at(-1);

  if (last !== undefined && DO_AUXILIARIES.has(last)) {
    return prefix.slice(0, -1);
  }

  return prefix;
}

function startsWithWords(
  tokens: readonly Token[],
  words: readonly string[]
): boolean {
  if (words.length === 0 || tokens.length < words.length) {
    return false;
  }

  for (let index = 0; index < words.length; index += 1) {
    if (tokens[index]?.normalized !== words[index]) {
      return false;
    }
  }

  return true;
}

function hasCommaBeforeNegation(text: string, negationStart: number): boolean {
  for (let index = negationStart - 1; index >= 0; index -= 1) {
    const char = text[index];

    if (char === ",") {
      return true;
    }

    if (char !== undefined && char.trim() !== "") {
      return false;
    }
  }

  return false;
}

function inlineNegationContrast(
  sentence: SplitSentence
): NegationReframeMatch | undefined {
  const tokens = wordTokens(sentence.text);
  const negationIndex = findNegationIndex(tokens);

  if (negationIndex === undefined) {
    return undefined;
  }

  const negation = tokens[negationIndex];

  if (
    negation === undefined ||
    !hasCommaBeforeNegation(sentence.text, negation.start)
  ) {
    return undefined;
  }

  return {
    end: sentence.end,
    start: sentence.start,
    text: sentence.text
  };
}

function sentencePairReframe(
  a: SplitSentence,
  b: SplitSentence
): NegationReframeMatch | undefined {
  const aTokens = wordTokens(a.text);
  const bTokens = wordTokens(b.text);
  const negationIndex = findNegationIndex(aTokens);

  if (negationIndex === undefined) {
    return undefined;
  }

  const positivePrefix = positivePrefixBeforeNegation(aTokens, negationIndex);

  if (startsWithWords(bTokens, positivePrefix)) {
    return {
      end: b.end,
      start: a.start,
      text: `${a.text} ${b.text}`
    };
  }

  const bStart = bTokens
    .slice(0, 2)
    .map((token) => token.normalized)
    .join(" ");

  if (
    ["it is", "it means", "they are", "they need", "you need"].includes(bStart)
  ) {
    return {
      end: b.end,
      start: a.start,
      text: `${a.text} ${b.text}`
    };
  }

  return undefined;
}

export function findNegationReframes(text: string): NegationReframeMatch[] {
  const sentences = splitSentences(text);
  const matches: NegationReframeMatch[] = [];

  for (const sentence of sentences) {
    const inlineMatch = inlineNegationContrast(sentence);

    if (inlineMatch !== undefined) {
      matches.push(inlineMatch);
    }
  }

  for (let index = 0; index < sentences.length - 1; index += 1) {
    const current = sentences[index];
    const next = sentences[index + 1];

    if (current === undefined || next === undefined) {
      continue;
    }

    const pairMatch = sentencePairReframe(current, next);

    if (pairMatch !== undefined) {
      matches.push(pairMatch);
    }
  }

  for (const match of findPhraseMatches(normalizeForMatch(text), [
    "not just"
  ])) {
    matches.push(match);
  }

  return matches;
}
