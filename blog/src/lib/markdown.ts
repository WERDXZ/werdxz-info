import { marked } from "marked";
import markedKatex from "marked-katex-extension";
import markedAlert from "marked-alert";

// Configure marked with GFM (GitHub Flavored Markdown)
// NOTE: breaks: false is required for KaTeX block math to work properly
// With breaks: true, newlines become <br> before KaTeX can tokenize $$...$$ blocks
marked.setOptions({
  gfm: true,
  breaks: false,
});

// Add GFM alerts extension ([!NOTE], [!WARNING], etc.)
marked.use(markedAlert());

// Add KaTeX extension for LaTeX math rendering
// Supports $inline$ and $$block$$ math
marked.use(
  markedKatex({
    throwOnError: false,
    output: "html",
  })
);

/**
 * Renders markdown to HTML with LaTeX math and GFM alerts support
 * Code blocks will be highlighted client-side with highlight.js
 */
export async function renderMarkdown(content: string): Promise<string> {
  const html = await marked.parse(content);
  return html;
}
