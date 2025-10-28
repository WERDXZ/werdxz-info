import { component$ } from "@builder.io/qwik";
import { Link } from "@builder.io/qwik-city";
import type { Post } from "~/lib/types";
import { formatPostDate } from "~/lib/date";
import styles from "./post-card.module.css";

interface PostCardProps {
  post: Post;
}

export const PostCard = component$<PostCardProps>(({ post }) => {
  const date = formatPostDate(post.published_at);

  return (
    <article class={styles.card}>
      <Link href={`/posts/${post.slug}`} class={styles.cardLink}>
        <h2 class={styles.title}>{post.title}</h2>
        <p class={styles.summary}>{post.summary}</p>
        <footer class={styles.meta}>
          <time class={styles.date} dateTime={post.published_at}>{date}</time>
          {post.read_time_minutes && (
            <span class={styles.readTime}>{post.read_time_minutes} min read</span>
          )}
        </footer>
      </Link>
      {post.tags && post.tags.length > 0 && (
        <ul class={styles.tags}>
          {post.tags.map((tag) => (
            <li key={tag} class={styles.tag}>
              <Link href={`/?tags=${tag}`}>{tag}</Link>
            </li>
          ))}
        </ul>
      )}
    </article>
  );
});
