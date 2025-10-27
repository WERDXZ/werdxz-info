import { signal } from "@preact/signals";
import { useEffect } from "preact/hooks";
import type { Project } from "../lib/api.ts";
import { StageBadge } from "../components/StageBadge.tsx";

interface ProjectSearchProps {
  projects: Project[];
  initialQuery?: string;
}

const searchQuery = signal("");

export default function ProjectSearch({ projects, initialQuery = "" }: ProjectSearchProps) {
  // Initialize from URL on mount
  useEffect(() => {
    searchQuery.value = initialQuery;
  }, [initialQuery]);

  // Update URL when search changes
  useEffect(() => {
    const params = new URLSearchParams(window.location.search);
    if (searchQuery.value) {
      params.set('q', searchQuery.value);
    } else {
      params.delete('q');
    }
    const newUrl = params.toString()
      ? `${window.location.pathname}?${params.toString()}`
      : window.location.pathname;
    window.history.replaceState({}, '', newUrl);
  }, [searchQuery.value]);

  // Filter projects based on search query
  const filteredProjects = projects.filter((project) => {
    if (!searchQuery.value) return true;

    const query = searchQuery.value.toLowerCase();
    return (
      project.name.toLowerCase().includes(query) ||
      project.description.toLowerCase().includes(query) ||
      (project.tags && project.tags.some(tag => tag.toLowerCase().includes(query)))
    );
  });

  return (
    <>
      <input
        type="search"
        aria-label="Search projects by name, description, or tags"
        placeholder="Search projects..."
        value={searchQuery.value}
        onInput={(e) => (searchQuery.value = (e.target as HTMLInputElement).value)}
        class="search-input"
      />

      {searchQuery.value && (
        <p class="search-results-count">
          {filteredProjects.length} {filteredProjects.length === 1 ? 'project' : 'projects'} found
        </p>
      )}

      {filteredProjects.length === 0 ? (
        <p class="no-results">No projects found matching "{searchQuery.value}"</p>
      ) : (
        filteredProjects.map((project) => (
          <ProjectCard key={project.id} project={project} />
        ))
      )}
    </>
  );
}

function ProjectCard({ project }: { project: Project }) {
  return (
    <article class="project-card">
      <a href={`/${project.slug}`} class="card-link">
        <header class="card-header">
          <h2 class="project-title">{project.name}</h2>
          <div class="badges">
            <StageBadge stage={project.stage} />
            {project.open_to_contributors && (
              <span class="badge contributor-badge">Open to Contributors</span>
            )}
          </div>
        </header>

        <p class="project-description">{project.description}</p>

        {project.tags && project.tags.length > 0 && (
          <ul class="tags">
            {project.tags.map((tag) => (
              <li key={tag} class="tag">{tag}</li>
            ))}
          </ul>
        )}
      </a>

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
    </article>
  );
}

