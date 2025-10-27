import { marked } from "marked";

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

// Extract headings from markdown for TOC
export function extractHeadings(markdown: string): TocHeading[] {
  const headings: TocHeading[] = [];
  const headingRegex = /^(#{1,6})\s+(.+)$/gm;

  let match;
  while ((match = headingRegex.exec(markdown)) !== null) {
    const level = match[1].length;
    const rawText = match[2].trim();
    const text = rawText; // Keep original for display
    const strippedText = stripMarkdown(rawText); // Strip for ID generation
    const id = slugify(strippedText);

    // Only include h2 and h3 for TOC (like MDN)
    if (level === 2 || level === 3) {
      headings.push({ id, text, level });
    }
  }

  return headings;
}

// Strip HTML tags from text
function stripHtml(text: string): string {
  return text.replace(/<[^>]+>/g, "");
}

// Configure marked with custom renderer to add IDs to headings
const renderer = {
  heading(text: string, level: number): string {
    // marked passes already-rendered HTML, so strip tags for ID generation
    const strippedText = stripHtml(text);
    const id = slugify(strippedText);
    return `<h${level} id="${id}">${text}</h${level}>\n`;
  }
};

marked.use({ renderer });

// Markdown to HTML using marked
export function markdownToHTML(markdown: string): string {
  return marked.parse(markdown, { gfm: true, breaks: false }) as string;
}
