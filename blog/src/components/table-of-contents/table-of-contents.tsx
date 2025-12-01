import { $, component$, useSignal, useVisibleTask$ } from "@builder.io/qwik";
import type { TocHeading } from "~/lib/markdown";
import styles from "./table-of-contents.module.css";

interface TableOfContentsProps {
  headings: TocHeading[];
}

export const TableOfContents = component$<TableOfContentsProps>(
  ({ headings }) => {
    const activeId = useSignal<string | null>(null);
    const isOpen = useSignal(false);

    // eslint-disable-next-line qwik/no-use-visible-task
    useVisibleTask$(
      () => {
        if (headings.length === 0) return;

        const observer = new IntersectionObserver(
          (entries) => {
            const intersecting = entries.filter((entry) => entry.isIntersecting);
            if (intersecting.length > 0) {
              intersecting.sort(
                (a, b) => a.boundingClientRect.top - b.boundingClientRect.top
              );
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
      },
      { strategy: "document-ready" }
    );

    const handleClick = $((e: Event, id: string) => {
      e.preventDefault();
      activeId.value = id;

      const element = document.getElementById(id);
      if (element) {
        const offset = window.innerHeight * 0.2;
        const elementPosition = element.getBoundingClientRect().top;
        const offsetPosition = elementPosition + window.scrollY - offset;

        window.scrollTo({
          top: offsetPosition,
        });
      }
    });

    if (headings.length === 0) return null;

    return (
      <>
        {/* Mobile: Collapsible inline */}
        <details
          class={styles.tocMobile}
          open={isOpen.value}
        >
          <summary onClick$={() => (isOpen.value = !isOpen.value)}>
            On this page
          </summary>
          <nav class={styles.tocNav}>
            {headings.map((heading) => (
              <a
                key={heading.id}
                href={`#${heading.id}`}
                onClick$={(e) => handleClick(e, heading.id)}
                aria-current={activeId.value === heading.id ? "true" : undefined}
                class={[
                  styles.tocLink,
                  heading.level === 3 ? styles.tocLevel3 : "",
                  activeId.value === heading.id ? styles.tocActive : "",
                ].join(" ")}
              >
                {heading.text}
              </a>
            ))}
          </nav>
        </details>

        {/* Desktop: Sticky sidebar */}
        <aside class={styles.tocDesktop}>
          <h3 class={styles.tocTitle}>On this page</h3>
          <nav class={styles.tocNav}>
            {headings.map((heading) => (
              <a
                key={heading.id}
                href={`#${heading.id}`}
                onClick$={(e) => handleClick(e, heading.id)}
                aria-current={activeId.value === heading.id ? "true" : undefined}
                class={[
                  styles.tocLink,
                  heading.level === 3 ? styles.tocLevel3 : "",
                  activeId.value === heading.id ? styles.tocActive : "",
                ].join(" ")}
              >
                {heading.text}
              </a>
            ))}
          </nav>
        </aside>
      </>
    );
  }
);
