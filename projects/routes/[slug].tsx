import { Head } from "fresh/runtime";
import { define } from "../utils.ts";
import { getProjectBySlug, fetchReadme, type Project } from "../lib/api.ts";
import { extractHeadings, markdownToHTML } from "../lib/markdown.ts";
import TableOfContents from "../islands/TableOfContents.tsx";
import { StageBadge } from "../components/StageBadge.tsx";

export default define.page(async function ProjectDetail(ctx) {
    const { slug } = ctx.params;
    const project = await getProjectBySlug(slug);

    if (!project) {
      return (
        <>
          <Head>
            <title>Project Not Found | werdxz</title>
          </Head>

          <div class="not-found">
            <h1>Project Not Found</h1>
            <p>The project "{slug}" could not be found.</p>
            <a href="/" f-client-nav>← Back to Projects</a>
          </div>
        </>
      );
    }

    const readme = await fetchReadme(project.readme_url);
    const headings = extractHeadings(readme);

    return (
      <>
        <Head>
          <title>{project.name} | werdxz Projects</title>
          <meta name="description" content={project.description} />

          {/* OpenGraph tags */}
          <meta property="og:title" content={`${project.name} | werdxz Projects`} />
          <meta property="og:description" content={project.description} />
          <meta property="og:type" content="article" />
          <meta property="og:url" content={`https://projects.werdxz.info/${project.slug}`} />

          {/* Twitter Card tags */}
          <meta name="twitter:card" content="summary" />
          <meta name="twitter:title" content={`${project.name} | werdxz Projects`} />
          <meta name="twitter:description" content={project.description} />

          <link rel="stylesheet" href="/markdown.css" />
          <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-tomorrow.min.css" />
          <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/prism.min.js"></script>
          <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/plugins/autoloader/prism-autoloader.min.js"></script>
        </Head>

        <>
          <nav class="breadcrumb" aria-label="Breadcrumb">
            <a href="/" f-client-nav>Projects</a> / {project.name}
          </nav>

          <TableOfContents headings={headings} />

          <article class="project-detail">
            <header class="detail-header">
              <h1 class="project-title">{project.name}</h1>
              <div class="badges">
                <StageBadge stage={project.stage} />
                {project.open_to_contributors && (
                  <span class="badge contributor-badge">Open to Contributors</span>
                )}
              </div>
              <p class="project-description">{project.description}</p>

              {project.tags && project.tags.length > 0 && (
                <ul class="tags">
                  {project.tags.map((tag) => (
                    <li key={tag} class="tag">{tag}</li>
                  ))}
                </ul>
              )}

              {project.urls && project.urls.length > 0 && (
                <nav class="project-urls" aria-label="Project links">
                  {project.urls.map((url) => (
                    <a
                      key={url.label}
                      href={url.url}
                      target="_blank"
                      rel="noopener noreferrer"
                      class="url-link"
                    >
                      {url.label} →
                    </a>
                  ))}
                </nav>
              )}
            </header>

            <section class="readme-content" dangerouslySetInnerHTML={{ __html: markdownToHTML(readme) }} />
          </article>
        </>
      </>
    );
  }
);

