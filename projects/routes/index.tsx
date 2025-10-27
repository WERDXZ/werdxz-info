import { Head } from "fresh/runtime";
import { define } from "../utils.ts";
import { getProjects } from "../lib/api.ts";
import ProjectSearch from "../islands/ProjectSearch.tsx";

export default define.page(async function ProjectsIndex(ctx) {
  const projects = await getProjects();
  const initialQuery = ctx.url.searchParams.get("q") || "";

  return (
    <>
      <Head>
        <title>Projects | werdxz</title>
        <meta name="description" content="Open-source projects by werdxz" />

        {/* OpenGraph tags */}
        <meta property="og:title" content="Projects | werdxz" />
        <meta property="og:description" content="Open-source projects by werdxz" />
        <meta property="og:type" content="website" />
        <meta property="og:url" content="https://projects.werdxz.info/" />

        {/* Twitter Card tags */}
        <meta name="twitter:card" content="summary" />
        <meta name="twitter:title" content="Projects | werdxz" />
        <meta name="twitter:description" content="Open-source projects by werdxz" />
      </Head>

      <ProjectSearch projects={projects} initialQuery={initialQuery} />
    </>
  );
});
