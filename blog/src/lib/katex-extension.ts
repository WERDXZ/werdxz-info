/**
 * Custom KaTeX extension for marked
 * Based on zero-md's implementation for proper block math support
 * https://github.com/nicohomework/zero-md
 */

import type { MarkedExtension, Tokens } from "marked";
import katex from "katex";

interface KatexOptions {
  nonStandard?: boolean;
  throwOnError?: boolean;
  displayMode?: boolean;
  [key: string]: unknown;
}

interface KatexToken extends Tokens.Generic {
  type: "inlineKatex" | "blockKatex";
  raw: string;
  text: string;
  displayMode: boolean;
}

// Inline math: $...$ or $$...$$ (on same line)
const inlineRule =
  /^(\${1,2})(?!\$)((?:\\.|[^\\\n])*?(?:\\.|[^\\\n$]))\1(?=[\s?!.,:？！。，：]|$)/;

// Non-standard: allow math without requiring spaces before/after
const inlineRuleNonStandard =
  /^(\${1,2})(?!\$)((?:\\.|[^\\\n])*?(?:\\.|[^\\\n$]))\1/;

// Block math: $$ on separate lines with content between
// Allow optional whitespace after opening $$ and before closing $$
const blockRule = /^(\${1,2})[ \t]*\n((?:\\[^]|[^\\])+?)\n[ \t]*\1(?:\n|$)/;

function createRenderer(options: KatexOptions, displayMode: boolean) {
  return (token: KatexToken): string => {
    try {
      return katex.renderToString(token.text, {
        ...options,
        displayMode: token.displayMode || displayMode,
      });
    } catch (e) {
      if (options.throwOnError) {
        throw e;
      }
      console.warn("KaTeX render error:", e);
      return `<span class="katex-error" title="${String(e)}">${token.text}</span>`;
    }
  };
}

function inlineKatex(
  options: KatexOptions,
  renderer: (token: KatexToken) => string
) {
  const nonStandard = options?.nonStandard;
  const ruleReg = nonStandard ? inlineRuleNonStandard : inlineRule;

  return {
    name: "inlineKatex",
    level: "inline" as const,
    start(src: string) {
      let index: number;
      let indexSrc = src;

      while (indexSrc) {
        index = indexSrc.indexOf("$");
        if (index === -1) {
          return;
        }
        const f = nonStandard
          ? index > -1
          : index === 0 || indexSrc.charAt(index - 1) === " ";
        if (f) {
          const possibleKatex = indexSrc.substring(index);
          if (possibleKatex.match(ruleReg)) {
            return index;
          }
        }
        indexSrc = indexSrc.substring(index + 1).replace(/^\$+/, "");
      }
    },
    tokenizer(src: string): KatexToken | undefined {
      const match = src.match(ruleReg);
      if (match) {
        return {
          type: "inlineKatex",
          raw: match[0],
          text: match[2].trim(),
          displayMode: match[1].length === 2,
        };
      }
    },
    renderer(token: Tokens.Generic) {
      return renderer(token as KatexToken);
    },
  };
}

function blockKatex(
  options: KatexOptions,
  renderer: (token: KatexToken) => string
) {
  return {
    name: "blockKatex",
    level: "block" as const,
    tokenizer(src: string): KatexToken | undefined {
      const match = src.match(blockRule);
      if (match) {
        return {
          type: "blockKatex",
          raw: match[0],
          text: match[2].trim(),
          displayMode: true,
        };
      }
    },
    renderer(token: Tokens.Generic) {
      return renderer(token as KatexToken) + "\n";
    },
  };
}

export default function katexExtension(options: KatexOptions = {}): MarkedExtension {
  const opts: KatexOptions = {
    nonStandard: true,
    throwOnError: false,
    ...options,
  };

  return {
    extensions: [
      inlineKatex(opts, createRenderer(opts, false)),
      blockKatex(opts, createRenderer(opts, true)),
    ],
  };
}
