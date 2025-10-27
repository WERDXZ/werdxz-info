import { component$, useVisibleTask$ } from "@builder.io/qwik";
import { routeLoader$ } from "@builder.io/qwik-city";
import type { DocumentHead } from "@builder.io/qwik-city";
import { Link } from "@builder.io/qwik-city";
import { fetchBlog } from "~/lib/api";
import { formatBlogDate } from "~/lib/date";
import { renderMarkdown } from "~/lib/markdown";
import styles from "./index.module.css";

export const useBlog = routeLoader$(async ({ params, status }) => {
  try {
    const blog = await fetchBlog(params.slug);

    let htmlContent = "";
    if (blog.content) {
      htmlContent = await renderMarkdown(blog.content);
    }

    return { blog, htmlContent };
  } catch (error) {
    console.error("[SSR] Failed to fetch blog:", params.slug, error);
    status(404);
    return null;
  }
});

export default component$(() => {
  const blogSignal = useBlog();
  const data = blogSignal.value;

  // eslint-disable-next-line qwik/no-use-visible-task
  useVisibleTask$(() => {
    if (typeof window === 'undefined') return;

    // Dynamically import highlight.js to avoid SSR issues
    import('highlight.js').then((hljs) => {
      // Highlight all code blocks
      document.querySelectorAll('pre code').forEach((block) => {
        hljs.default.highlightElement(block as HTMLElement);
      });
    });
  }, { strategy: 'document-ready' });

  if (!data) {
    return (
      <section class={styles.notFound}>
        <h1>Blog Post Not Found</h1>
        <p>Sorry, the blog post you're looking for doesn't exist.</p>
        <Link href="/" class={styles.backLink}>
          ← Back to Blog
        </Link>
      </section>
    );
  }

  const { blog, htmlContent} = data;
  const date = formatBlogDate(blog.published_at);

  return (
    <article>
      <Link href="/" class={styles.backLink}>
        ← Back to Blog
      </Link>

      <header class={styles.header}>
        <h1 class={styles.title}>{blog.title}</h1>
        <p class={styles.meta}>
          <time class={styles.date} dateTime={blog.published_at}>
            {date}
          </time>
          {blog.read_time_minutes && (
            <span class={styles.readTime}>
              {blog.read_time_minutes} min read
            </span>
          )}
        </p>
        {blog.tags && blog.tags.length > 0 && (
          <ul class={styles.tags}>
            {blog.tags.map((tag) => (
              <li key={tag} class={styles.tag}>
                <Link href={`/?tags=${tag}`}>{tag}</Link>
              </li>
            ))}
          </ul>
        )}
      </header>

      <section class={styles.content} dangerouslySetInnerHTML={htmlContent} />
    </article>
  );
});

export const head: DocumentHead = ({ resolveValue }) => {
  const data = resolveValue(useBlog);
  if (!data) {
    return {
      title: "Blog Post Not Found | werdxz.info",
    };
  }

  const { blog } = data;
  return {
    title: `${blog.title} | werdxz.info`,
    meta: [
      {
        name: "description",
        content: blog.summary,
      },
      {
        property: "og:title",
        content: blog.title,
      },
      {
        property: "og:description",
        content: blog.summary,
      },
      {
        property: "og:type",
        content: "article",
      },
      {
        property: "article:published_time",
        content: blog.published_at,
      },
      ...(blog.tags?.map((tag) => ({
        property: "article:tag",
        content: tag,
      })) || []),
    ],
  };
};
