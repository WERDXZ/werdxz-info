import type { RequestHandler } from "@builder.io/qwik-city";

const API_BASE_URL = import.meta.env.PUBLIC_API_BASE_URL || 'https://api.werdxz.info/v1';

interface Post {
  slug: string;
  updated_at: string;
}

interface PostsResponse {
  posts: Post[];
  pagination: {
    total: number;
  };
}

export const onGet: RequestHandler = async ({ send, cacheControl }) => {
  cacheControl({
    public: true,
    maxAge: 3600, // 1 hour
    sMaxAge: 3600,
    staleWhileRevalidate: 86400, // 1 day
  });

  const baseUrl = "https://blog.werdxz.info";

  try {
    // Fetch all posts with a large limit
    const response = await fetch(`${API_BASE_URL}/posts?limit=1000&page=1`);
    if (!response.ok) {
      throw new Error(`API request failed: ${response.statusText}`);
    }

    const data: PostsResponse = await response.json();
    const posts = data.posts;

    // Calculate most recent update for index
    const mostRecentUpdate = posts.reduce((latest, post) => {
      const postDate = new Date(post.updated_at);
      return postDate > latest ? postDate : latest;
    }, new Date(0));
    const indexLastMod = mostRecentUpdate.toISOString().split('T')[0];

    const xml = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>${baseUrl}/</loc>
    <lastmod>${indexLastMod}</lastmod>
    <changefreq>daily</changefreq>
    <priority>1.0</priority>
  </url>
  <url>
    <loc>${baseUrl}/demo/todolist</loc>
    <lastmod>${new Date().toISOString().split('T')[0]}</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.5</priority>
  </url>
  <url>
    <loc>${baseUrl}/demo/flower</loc>
    <lastmod>${new Date().toISOString().split('T')[0]}</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.5</priority>
  </url>
${posts.map(post => {
      const lastmod = new Date(post.updated_at).toISOString().split('T')[0];
      return `  <url>
    <loc>${baseUrl}/posts/${post.slug}</loc>
    <lastmod>${lastmod}</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.8</priority>
  </url>`;
    }).join('\n')}
</urlset>`;

    send(new Response(xml, {
      status: 200,
      headers: {
        "Content-Type": "application/xml; charset=utf-8",
      },
    }));
  } catch (error) {
    console.error("[Sitemap] Error generating sitemap:", error);

    // Return minimal sitemap on error
    const fallbackXml = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>${baseUrl}/</loc>
    <lastmod>${new Date().toISOString().split('T')[0]}</lastmod>
    <changefreq>daily</changefreq>
    <priority>1.0</priority>
  </url>
</urlset>`;

    send(new Response(fallbackXml, {
      status: 200,
      headers: {
        "Content-Type": "application/xml; charset=utf-8",
      },
    }));
  }
};
