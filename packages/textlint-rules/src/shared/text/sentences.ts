export type SplitSentence = {
  readonly end: number;
  readonly start: number;
  readonly text: string;
};

const SENTENCE_SEGMENTER = new Intl.Segmenter("en", {
  granularity: "sentence"
});

function firstNonWhitespaceIndex(text: string): number | undefined {
  for (let index = 0; index < text.length; index += 1) {
    if (text[index]?.trim() !== "") {
      return index;
    }
  }

  return undefined;
}

function trimEndIndex(text: string): number {
  for (let index = text.length; index > 0; index -= 1) {
    if (text[index - 1]?.trim() !== "") {
      return index;
    }
  }

  return 0;
}

export function splitSentences(text: string): SplitSentence[] {
  const sentences: SplitSentence[] = [];

  for (const segment of SENTENCE_SEGMENTER.segment(text)) {
    const trimStart = firstNonWhitespaceIndex(segment.segment);
    if (trimStart === undefined) {
      continue;
    }

    const trimEnd = trimEndIndex(segment.segment);
    const start = segment.index + trimStart;
    const end = segment.index + trimEnd;

    sentences.push({
      end,
      start,
      text: segment.segment.slice(trimStart, trimEnd)
    });
  }

  return sentences;
}
