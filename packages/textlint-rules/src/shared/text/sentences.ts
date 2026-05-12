import {
  split,
  type TxtParentNodeWithSentenceNodeContent,
  type TxtSentenceNode
} from "sentence-splitter";

export type SplitSentence = {
  readonly end: number;
  readonly start: number;
  readonly text: string;
};

function isSentenceNode(
  node: TxtParentNodeWithSentenceNodeContent
): node is TxtSentenceNode {
  return node.type === "Sentence";
}

export function splitSentences(text: string): SplitSentence[] {
  const sentences: SplitSentence[] = [];

  for (const node of split(text)) {
    if (!isSentenceNode(node)) {
      continue;
    }

    const [start, end] = node.range;

    sentences.push({
      end,
      start,
      text: node.raw
    });
  }

  return sentences;
}
