import { marked } from "marked";
import markedAlert from "marked-alert";
import katexExtension from "./katex-extension";

export interface TocHeading {
  id: string;
  text: string;
  level: number;
}

// Simple slug generation for heading IDs
function slugify(text: string): string {
  return text
    .toLowerCase()
    .replace(/[^\w\s-]/g, "")
    .replace(/\s+/g, "-")
    .replace(/-+/g, "-")
    .trim();
}

// Strip markdown formatting from text (bold, italic, code, links)
function stripMarkdown(text: string): string {
  return text
    .replace(/\[([^\]]+)\]\([^)]+\)/g, "$1") // [text](url) -> text
    .replace(/`([^`]+)`/g, "$1") // `code` -> code
    .replace(/\*\*([^*]+)\*\*/g, "$1") // **bold** -> bold
    .replace(/\*([^*]+)\*/g, "$1") // *italic* -> italic
    .replace(/__([^_]+)__/g, "$1") // __bold__ -> bold
    .replace(/_([^_]+)_/g, "$1"); // _italic_ -> italic
}

// Strip HTML tags from text
function stripHtml(text: string): string {
  return text.replace(/<[^>]+>/g, "");
}

// Extract headings from markdown for TOC
export function extractHeadings(markdown: string): TocHeading[] {
  const headings: TocHeading[] = [];
  const headingRegex = /^(#{1,6})\s+(.+)$/gm;

  let match;
  while ((match = headingRegex.exec(markdown)) !== null) {
    const level = match[1].length;
    const rawText = match[2].trim();
    const text = rawText;
    const strippedText = stripMarkdown(rawText);
    const id = slugify(strippedText);

    // Only include h2 and h3 for TOC
    if (level === 2 || level === 3) {
      headings.push({ id, text, level });
    }
  }

  return headings;
}

// Custom renderer to add IDs to headings
// marked v16 passes an object with text and depth properties
const renderer = {
  heading({ text, depth }: { text: string; depth: number }): string {
    const strippedText = stripHtml(text);
    const id = slugify(strippedText);
    return `<h${depth} id="${id}">${text}</h${depth}>\n`;
  },
};

// Configure marked with GFM (GitHub Flavored Markdown)
// NOTE: breaks: false is required for KaTeX block math to work properly
// With breaks: true, newlines become <br> before KaTeX can tokenize $$...$$ blocks
marked.setOptions({
  gfm: true,
  breaks: false,
});

// Add KaTeX extension FIRST - it needs to tokenize math blocks before other processing
marked.use(katexExtension({
  throwOnError: false,
  nonStandard: true,
}));

// Add GFM alerts extension ([!NOTE], [!WARNING], etc.)
marked.use(markedAlert());

// Add custom renderer for heading IDs LAST
marked.use({ renderer });

/**
 * Renders markdown to HTML with LaTeX math and GFM alerts support
 * Code blocks will be highlighted client-side with highlight.js
 */
export async function renderMarkdown(content: string): Promise<string> {
  const html = await marked.parse(content);
  return html;
}
