import { marked } from "marked";

// Configure marked with GFM (GitHub Flavored Markdown)
marked.setOptions({
  gfm: true,
  breaks: true,
});

/**
 * Renders markdown to HTML
 * Code blocks will be highlighted client-side with highlight.js
 */
export async function renderMarkdown(content: string): Promise<string> {
  const html = await marked.parse(content);
  return html;
}
