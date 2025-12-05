import type { RequestHandler } from "@builder.io/qwik-city";
import { createSitemap, type SitemapEntry } from "./create-sitemap";

const API_BASE_URL = import.meta.env.PUBLIC_API_BASE_URL || 'https://api.werdxz.info/v1';
const BASE_URL = "https://blog.werdxz.info";

interface Post {
  slug: string;
  updated_at: string;
}

interface PostsResponse {
  posts: Post[];
}

function formatDate(dateStr: string | undefined): string | undefined {
  if (!dateStr) return undefined;
  const date = new Date(dateStr);
  if (isNaN(date.getTime())) return undefined;
  return date.toISOString().split('T')[0];
}

export const onGet: RequestHandler = async ({ send, cacheControl }) => {
  cacheControl({
    public: true,
    maxAge: 3600,
    sMaxAge: 3600,
    staleWhileRevalidate: 86400,
  });

  try {
    const response = await fetch(`${API_BASE_URL}/posts?limit=1000&page=1`);
    if (!response.ok) {
      throw new Error(`API request failed: ${response.statusText}`);
    }

    const data: PostsResponse = await response.json();
    const posts = data.posts ?? [];

    const today = new Date().toISOString().split('T')[0];

    const mostRecentUpdate = posts.reduce<string>((latest, post) => {
      const postDate = formatDate(post.updated_at);
      if (!postDate) return latest;
      return postDate > latest ? postDate : latest;
    }, today);

    const entries: SitemapEntry[] = [
      {
        loc: "/",
        lastmod: mostRecentUpdate,
        changefreq: "daily",
        priority: 1.0,
      },
      ...posts.map((post) => ({
        loc: `/posts/${post.slug}`,
        lastmod: formatDate(post.updated_at),
        changefreq: "monthly" as const,
        priority: 0.8,
      })),
    ];

    const sitemap = createSitemap(BASE_URL, entries);

    send(new Response(sitemap, {
      status: 200,
      headers: { "Content-Type": "text/xml" },
    }));
  } catch (error) {
    console.error("[Sitemap] Error generating sitemap:", error);

    const fallbackEntries: SitemapEntry[] = [
      {
        loc: "/",
        lastmod: new Date().toISOString().split('T')[0],
        changefreq: "daily",
        priority: 1.0,
      },
    ];

    send(new Response(createSitemap(BASE_URL, fallbackEntries), {
      status: 200,
      headers: { "Content-Type": "text/xml" },
    }));
  }
};

export const onHead: RequestHandler = async ({ send, cacheControl }) => {
  cacheControl({
    public: true,
    maxAge: 3600,
    sMaxAge: 3600,
    staleWhileRevalidate: 86400,
  });

  send(new Response(null, {
    status: 200,
    headers: { "Content-Type": "text/xml" },
  }));
};
