export type SourceText = {
  readonly text: string;
};

export function sourceText(text: string): SourceText {
  return {
    text
  };
}
