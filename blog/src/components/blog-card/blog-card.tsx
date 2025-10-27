import { component$ } from "@builder.io/qwik";
import { Link } from "@builder.io/qwik-city";
import type { Blog } from "~/lib/types";
import { formatBlogDate } from "~/lib/date";
import styles from "./blog-card.module.css";

interface BlogCardProps {
  blog: Blog;
}

export const BlogCard = component$<BlogCardProps>(({ blog }) => {
  const date = formatBlogDate(blog.published_at);

  return (
    <article class={styles.card}>
      <Link href={`/blog/${blog.slug}`} class={styles.cardLink}>
        <h2 class={styles.title}>{blog.title}</h2>
        <p class={styles.summary}>{blog.summary}</p>
        <footer class={styles.meta}>
          <time class={styles.date} dateTime={blog.published_at}>{date}</time>
          {blog.read_time_minutes && (
            <span class={styles.readTime}>{blog.read_time_minutes} min read</span>
          )}
        </footer>
      </Link>
      {blog.tags && blog.tags.length > 0 && (
        <ul class={styles.tags}>
          {blog.tags.map((tag) => (
            <li key={tag} class={styles.tag}>
              <Link href={`/?tags=${tag}`}>{tag}</Link>
            </li>
          ))}
        </ul>
      )}
    </article>
  );
});
