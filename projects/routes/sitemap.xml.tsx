import { define } from "../utils.ts";
import { getProjects } from "../lib/api.ts";

export default define.page(async function Sitemap() {
  const projects = await getProjects();
  const baseUrl = "https://projects.werdxz.info";

  // Use the most recent project update for the index page
  const mostRecentUpdate = projects.reduce((latest, project) => {
    const projectDate = new Date(project.updated_at);
    return projectDate > latest ? projectDate : latest;
  }, new Date(0));
  const indexLastMod = mostRecentUpdate.toISOString().split('T')[0];

  const xml = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>${baseUrl}/</loc>
    <lastmod>${indexLastMod}</lastmod>
    <changefreq>weekly</changefreq>
    <priority>1.0</priority>
  </url>
${projects.map(project => {
    const lastmod = new Date(project.updated_at).toISOString().split('T')[0];
    return `  <url>
    <loc>${baseUrl}/${project.slug}</loc>
    <lastmod>${lastmod}</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.8</priority>
  </url>`;
  }).join('\n')}
</urlset>`;

  return new Response(xml, {
    headers: {
      "Content-Type": "application/xml",
    },
  });
});
