import { useEffect } from "preact/hooks";
import { signal } from "@preact/signals";

interface TocHeading {
  id: string;
  text: string;
  level: number;
}

interface TableOfContentsProps {
  headings: TocHeading[];
}

const activeId = signal<string | null>(null);
const isOpen = signal(false);

export default function TableOfContents({ headings }: TableOfContentsProps) {

  useEffect(() => {
    if (headings.length === 0) return;

    const observer = new IntersectionObserver(
      (entries) => {
        // Find the first intersecting entry (top-most in viewport)
        const intersecting = entries.filter(entry => entry.isIntersecting);
        if (intersecting.length > 0) {
          // Sort by their position and take the top-most one
          intersecting.sort((a, b) => a.boundingClientRect.top - b.boundingClientRect.top);
          activeId.value = intersecting[0].target.id;
        }
      },
      {
        rootMargin: "-20% 0px -80% 0px",
        threshold: 0,
      }
    );

    // Use requestAnimationFrame to ensure DOM is painted
    const rafId = requestAnimationFrame(() => {
      headings.forEach(({ id }) => {
        const element = document.getElementById(id);
        if (element) observer.observe(element);
      });
    });

    return () => {
      cancelAnimationFrame(rafId);
      observer.disconnect();
    };
  }, [headings]);

  const handleClick = (e: Event, id: string) => {
    e.preventDefault();

    // Immediately update the active TOC item
    activeId.value = id;

    const element = document.getElementById(id);
    if (element) {
      // Scroll to position heading at 20% from top (matching IntersectionObserver rootMargin)
      const offset = window.innerHeight * 0.2;
      const elementPosition = element.getBoundingClientRect().top;
      const offsetPosition = elementPosition + window.scrollY - offset;

      window.scrollTo({
        top: offsetPosition,
      });
    }
  };

  if (headings.length === 0) return null;

  return (
    <>
      {/* Mobile: Collapsible inline */}
      <details class="toc-mobile">
        <summary onClick={() => (isOpen.value = !isOpen.value)}>
          On this page
        </summary>
        <nav class="toc-nav">
          {headings.map((heading) => (
            <a
              key={heading.id}
              href={`#${heading.id}`}
              onClick={(e) => handleClick(e, heading.id)}
              aria-current={activeId.value === heading.id ? "true" : undefined}
              class={`toc-link toc-level-${heading.level} ${
                activeId.value === heading.id ? "toc-active" : ""
              }`}
            >
              {heading.text}
            </a>
          ))}
        </nav>
      </details>

      {/* Desktop: Sticky sidebar */}
      <aside class="toc-desktop">
        <h3 class="toc-title">On this page</h3>
        <nav class="toc-nav">
          {headings.map((heading) => (
            <a
              key={heading.id}
              href={`#${heading.id}`}
              onClick={(e) => handleClick(e, heading.id)}
              aria-current={activeId.value === heading.id ? "true" : undefined}
              class={`toc-link toc-level-${heading.level} ${
                activeId.value === heading.id ? "toc-active" : ""
              }`}
            >
              {heading.text}
            </a>
          ))}
        </nav>
      </aside>
    </>
  );
}
