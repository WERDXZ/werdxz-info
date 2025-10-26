import { component$, useVisibleTask$ } from "@builder.io/qwik";
import { routeLoader$ } from "@builder.io/qwik-city";
import type { DocumentHead } from "@builder.io/qwik-city";
import { Link } from "@builder.io/qwik-city";
import { fetchPost } from "~/lib/api";
import { formatPostDate } from "~/lib/date";
import { renderMarkdown } from "~/lib/markdown";
import styles from "./index.module.css";

export const usePost = routeLoader$(async ({ params, status }) => {
  try {
    const post = await fetchPost(params.slug);

    let htmlContent = "";
    if (post.content) {
      htmlContent = await renderMarkdown(post.content);
    }

    return { post, htmlContent };
  } catch (error) {
    console.error("[SSR] Failed to fetch post:", params.slug, error);
    status(404);
    return null;
  }
});

export default component$(() => {
  const postSignal = usePost();
  const data = postSignal.value;

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
        <h1>Post Not Found</h1>
        <p>Sorry, the post you're looking for doesn't exist.</p>
        <Link href="/" class={styles.backLink}>
          ← Back to Blog
        </Link>
      </section>
    );
  }

  const { post, htmlContent} = data;
  const date = formatPostDate(post.published_at);

  return (
    <article>
      <Link href="/" class={styles.backLink}>
        ← Back to Blog
      </Link>

      <header class={styles.header}>
        <h1 class={styles.title}>{post.title}</h1>
        <p class={styles.meta}>
          <time class={styles.date} dateTime={post.published_at}>
            {date}
          </time>
          {post.read_time_minutes && (
            <span class={styles.readTime}>
              {post.read_time_minutes} min read
            </span>
          )}
        </p>
        {post.tags && post.tags.length > 0 && (
          <ul class={styles.tags}>
            {post.tags.map((tag) => (
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
  const data = resolveValue(usePost);
  if (!data) {
    return {
      title: "Post Not Found | werdxz.info",
    };
  }

  const { post } = data;
  return {
    title: `${post.title} | werdxz.info`,
    meta: [
      {
        name: "description",
        content: post.summary,
      },
      {
        property: "og:title",
        content: post.title,
      },
      {
        property: "og:description",
        content: post.summary,
      },
      {
        property: "og:type",
        content: "article",
      },
      {
        property: "article:published_time",
        content: post.published_at,
      },
      ...(post.tags?.map((tag) => ({
        property: "article:tag",
        content: tag,
      })) || []),
    ],
  };
};
